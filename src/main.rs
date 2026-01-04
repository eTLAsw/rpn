use crossterm::event::{self, Event, KeyCode};
use std::env;

mod basic_math;
mod display;
mod logic_operators;
mod modes;
mod stack;
mod stack_manipulations;

pub enum CmdResult {
    Success,
    NoMatch,
    Error(&'static str),
}

const COMMANDS_FUNCTIONS: [fn(&str) -> CmdResult; 4] = [
    basic_math::commands,
    logic_operators::commands,
    modes::commands,
    stack_manipulations::commands,
];

fn parse_input(input: &mut String) -> Result<(), String> {
    let trimmed = input.split_whitespace().collect::<Vec<_>>().join(" ");

    for commands in COMMANDS_FUNCTIONS.iter() {
        match commands(&trimmed.as_str()) {
            CmdResult::Success => {
                input.clear();
                return Ok(());
            }
            CmdResult::Error(e) => {
                return Err(e.to_string());
            }
            CmdResult::NoMatch => {
                // Continue processing
            }
        }
    }

    if trimmed.starts_with("h") {
        // Hexadecimal
        let hex_str = &trimmed[1..];
        if let Ok(value) = i128::from_str_radix(hex_str, 16) {
            stack::push(stack::StackValue::Fraction((value, 1)));
            input.clear();
            return Ok(());
        }
    }

    if trimmed.starts_with("b") {
        // Binary
        let bin_str = &trimmed[1..];
        if let Ok(value) = i128::from_str_radix(bin_str, 2) {
            stack::push(stack::StackValue::Fraction((value, 1)));
            input.clear();
            return Ok(());
        }
    }

    if let Ok(value) = trimmed.parse::<i128>() {
        stack::push(stack::StackValue::Fraction((value, 1)));
        input.clear();
        return Ok(());
    }

    if let Ok(value) = trimmed.parse::<f64>() {
        stack::push(stack::StackValue::Float(value));
        input.clear();
        return Ok(());
    }

    Err("Invalid input format".to_string())
}

fn main_loop() -> anyhow::Result<()> {
    let mut input_buffer = String::new();
    let mut error_message = String::new();

    loop {
        display::draw(&input_buffer, &error_message).expect("Failed to draw UI");

        if event::poll(std::time::Duration::from_millis(100))? {
            error_message.clear();
            if let Event::Key(key) = event::read().unwrap() {
                if input_buffer.len() == 0 {
                    // If input buffer is empty, check for shortcut keys first.
                    let result = match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(c) => basic_math::quick_commands(&c),
                        _ => CmdResult::NoMatch,
                    };

                    match result {
                        CmdResult::Success => {
                            continue;
                        }
                        CmdResult::Error(e) => {
                            error_message = e.to_string();
                        }
                        CmdResult::NoMatch => {
                            // Continue to normal input processing
                        }
                    }
                }
                match key.code {
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        input_buffer.clear();
                    }
                    KeyCode::Enter => match parse_input(&mut input_buffer) {
                        Ok(_) => {}
                        Err(e) => {
                            error_message = e;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        // If there are command line arguments, process them and exit.
        for arg in args {
            match parse_input(&mut arg.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error parsing '{}': {}", arg, e);
                    break;
                }
            }
        }
    } else {
        // Enter interactive mode.
        display::setup().expect("Failed to setup terminal");
        let err = main_loop();
        display::restore().expect("Failed to restore terminal");
        match err {
            Ok(_) => {
                // Show the resulting stack contents on exit.
                stack::get_stack_contents()
                    .iter()
                    .for_each(|line| println!("{}", line));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
