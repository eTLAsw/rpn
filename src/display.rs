use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use lazy_static::lazy_static;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};
use std::sync::Mutex;

use crate::modes;
use crate::stack;

// All terminal handling is done here to later allow for different UIs.

lazy_static! {
    static ref TERMINAL: Mutex<Option<Terminal<CrosstermBackend<Stdout>>>> = Mutex::new(None);
}

pub fn setup() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    *TERMINAL.lock().unwrap() = Some(terminal);
    Ok(())
}

pub fn restore() -> anyhow::Result<()> {
    if let Some(mut terminal) = TERMINAL.lock().unwrap().take() {
        terminal.show_cursor()?;
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    }
    Ok(())
}

pub fn draw(input: &str, error: &str) -> anyhow::Result<()> {
    let mut terminal_lock = TERMINAL.lock().unwrap();
    let terminal = terminal_lock.as_mut().expect("Terminal is not setup");

    terminal.draw(|f| {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::{Color, Modifier, Style},
            text::{Line, Span},
            widgets::Paragraph,
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),    // Stack area
                    Constraint::Length(1), // Status area
                    Constraint::Length(1), // Input area
                ]
                .as_ref(),
            )
            .split(f.area());

        // Stack area
        let mut stack_contents = stack::get_stack_contents();
        while stack_contents.len() < f.area().height as usize - 2 {
            stack_contents.insert(0, "".to_string());
        }
        let main_area = Paragraph::new(stack_contents.join("\n"));
        f.render_widget(main_area, chunks[0]);

        // Status area, inverted colors. Also used for error messages.
        let status = Paragraph::new(Line::from(Span::styled(
            if error.is_empty() {
                let modes = modes::get_modes_string();
                modes.to_string()
            } else {
                error.to_string()
            },
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        f.render_widget(status, chunks[1]);

        // Input area
        let input = Paragraph::new("> ".to_string() + input);
        f.render_widget(input, chunks[2]);
    })?;

    Ok(())
}
