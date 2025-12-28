use crossterm::event::{self, Event, KeyCode};
use std::collections::HashMap;
use std::env;

mod basic_math;
mod display;
mod logic_operators;
mod modes;
mod stack;
mod stack_manipulations;

fn parse_input(input: &mut String) -> Result<(), String> {
    let mut commands: HashMap<&str, fn() -> Result<(), &'static str>> = HashMap::new();

    // TODO: This way of handling commands is messy, do this more elegantly and modular.

    commands.insert("add", basic_math::add);
    commands.insert("div", basic_math::div);
    commands.insert("mul", basic_math::mul);
    commands.insert("sub", basic_math::sub);

    commands.insert("and", logic_operators::and);
    commands.insert("not", logic_operators::not);
    commands.insert("or", logic_operators::or);
    commands.insert("xor", logic_operators::xor);

    commands.insert("drop", stack_manipulations::drop);
    commands.insert("dup", stack_manipulations::dup);
    commands.insert("swap", stack_manipulations::swap);
    commands.insert("undo", stack_manipulations::undo);

    commands.insert("signed", modes::set_binary_mode_signed);
    commands.insert("unsigned", modes::set_binary_mode_unsigned);
    commands.insert("width8", modes::set_binary_width_8);
    commands.insert("width16", modes::set_binary_width_16);
    commands.insert("width32", modes::set_binary_width_32);
    commands.insert("width64", modes::set_binary_width_64);

    let trimmed = input.split_whitespace().collect::<Vec<_>>().join(" ");

    if let Some(&action) = commands.get(trimmed.as_str()) {
        action().map_err(|e| e.to_string())?;
        input.clear();
        return Ok(());
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

                        // TODO: Could this also be done in a cleaner way?
                        KeyCode::Char('+') => basic_math::add(),
                        KeyCode::Char('/') => basic_math::div(),
                        KeyCode::Char('*') => basic_math::mul(),
                        KeyCode::Char('-') => basic_math::sub(),

                        // TODO: An empty error message is a special case meaning "not a shortcut key". Find a better way to handle this.
                        _ => Err(""),
                    };

                    match result {
                        Ok(()) => {
                            continue;
                        }
                        Err(e) => {
                            if e != "" {
                                error_message = e.to_string();
                            } else {
                                // Not a shortcut key, continue to normal input handling.
                            }
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
            Ok(_) => println!("Exiting..."),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Show the resulting stack contents on exit.
    stack::get_stack_contents()
        .iter()
        .for_each(|line| println!("{}", line));
}
