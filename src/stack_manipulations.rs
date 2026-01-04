use crate::CmdResult;
use crate::help;
use crate::stack;

// TODO: There should be some error handling here.

pub fn drop() -> CmdResult {
    stack::drop(1);
    CmdResult::Success
}

pub fn dup() -> CmdResult {
    let stack = stack::get_values(1);
    if let Some(value) = stack {
        stack::push(value[0]);
    }
    CmdResult::Success
}

pub fn swap() -> CmdResult {
    let stack = stack::get_values(2);
    if let Some(values) = stack {
        stack::drop(2);
        stack::push(values[1]);
        stack::push(values[0]);
    }
    CmdResult::Success
}

pub fn undo() -> CmdResult {
    stack::undo();
    CmdResult::Success
}

pub fn commands(cmd: &str) -> CmdResult {
    match cmd {
        "drop" => drop(),
        "dup" => dup(),
        "swap" => swap(),
        "undo" => undo(),
        _ => CmdResult::NoMatch,
    }
}

pub const HELP: help::Category = help::Category {
    category: "Stack Manipulations",
    commands: &[
        help::Cmd {
            command: "drop",
            help: "Removes the top value from the stack.",
        },
        help::Cmd {
            command: "dup",
            help: "Duplicates the top value on the stack.",
        },
        help::Cmd {
            command: "swap",
            help: "Swaps the top two values on the stack.",
        },
        help::Cmd {
            command: "undo",
            help: "Undoes the last stack manipulation.",
        },
    ],
};
