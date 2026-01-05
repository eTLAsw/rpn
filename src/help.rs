use crate::CmdResult;

pub struct Cmd {
    pub command: &'static str,
    pub help: &'static str,
}

pub struct Category {
    pub category: &'static str,
    pub commands: &'static [Cmd],
}

const ALL_CATEGORIES: &[&Category] = &[
    &crate::basic_math::HELP,
    &crate::logic_operators::HELP,
    &crate::modes::HELP,
    &crate::stack_manipulations::HELP,
];

static DISPLAY_HELP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn help() -> CmdResult {
    DISPLAY_HELP.store(true, std::sync::atomic::Ordering::SeqCst);
    CmdResult::Success
}

pub fn commands(cmd: &str) -> CmdResult {
    match cmd {
        "help" => help(),
        _ => CmdResult::NoMatch,
    }
}

// TODO: This is a rather crude implementation of a help function.

pub fn clear_help() {
    DISPLAY_HELP.store(false, std::sync::atomic::Ordering::SeqCst);
}

pub fn display_help() -> bool {
    DISPLAY_HELP.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn get_help() -> Vec<String> {
    let mut help_lines = Vec::new();

    for category in ALL_CATEGORIES {
        help_lines.push(format!("Category: {}", category.category));
        for cmd in category.commands {
            help_lines.push(format!("  {}: {}", cmd.command, cmd.help));
        }
        help_lines.push(String::new());
    }

    help_lines
}

pub fn print_help() {
    for category in ALL_CATEGORIES {
        println!("Category: {}", category.category);
        for cmd in category.commands {
            println!("  {}: {}", cmd.command, cmd.help);
        }
        println!();
    }
}
