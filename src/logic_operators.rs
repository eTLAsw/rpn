use crate::CmdResult;
use crate::help;
use crate::stack;

// Logic areas only work on integers, which is a special case of fractions. See stack.rs for more info.

fn and() -> CmdResult {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return CmdResult::Error("AND operation requires two integer values"),
        };

        let result = a & b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        CmdResult::Success
    } else {
        CmdResult::Error("Not enough values on stack")
    }
}

fn not() -> CmdResult {
    if let Some(values) = stack::get(1, stack::AcceptedTypes::INTEGERS) {
        let a = match &values[0] {
            stack::StackValue::Integer(a) => *a,
            _ => return CmdResult::Error("NOT operation requires an integer value"),
        };

        let result = !a;

        stack::drop(1);
        stack::push(stack::StackValue::Integer(result));

        CmdResult::Success
    } else {
        CmdResult::Error("Not enough values on stack")
    }
}

fn or() -> CmdResult {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return CmdResult::Error("OR operation requires two integer values"),
        };

        let result = a | b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        CmdResult::Success
    } else {
        CmdResult::Error("Not enough values on stack")
    }
}

fn xor() -> CmdResult {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return CmdResult::Error("XOR operation requires two integer values"),
        };

        let result = a ^ b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        CmdResult::Success
    } else {
        CmdResult::Error("Not enough values on stack")
    }
}

pub fn commands(cmd: &str) -> CmdResult {
    match cmd {
        "and" => and(),
        "or" => or(),
        "not" => not(),
        "xor" => xor(),
        _ => CmdResult::NoMatch,
    }
}

pub const HELP: help::Category = help::Category {
    category: "Logic Operators",
    commands: &[
        help::Cmd {
            command: "and",
            help: "Performs a bitwise AND operation on the top two integers on the stack.",
        },
        help::Cmd {
            command: "or",
            help: "Performs a bitwise OR operation on the top two integers on the stack.",
        },
        help::Cmd {
            command: "not",
            help: "Performs a bitwise NOT operation on the top integer on the stack.",
        },
        help::Cmd {
            command: "xor",
            help: "Performs a bitwise XOR operation on the top two integers on the stack.",
        },
    ],
};
