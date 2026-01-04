use crate::CmdResult;
use crate::help;
use crate::stack;

// Basic mathematical operations that work with both floats and fractions.

fn add() -> CmdResult {
    if let Some(values) = stack::get(
        2,
        stack::AcceptedTypes::FLOAT | stack::AcceptedTypes::FRACTIONS,
    ) {
        match (&values[0], &values[1]) {
            (
                stack::StackValue::Fraction((num1, denom1)),
                stack::StackValue::Fraction((num2, denom2)),
            ) => {
                let result_num = num1 * denom2 + num2 * denom1;
                let result_denom = denom1 * denom2;
                stack::drop(2);
                stack::push(stack::StackValue::Fraction((result_num, result_denom)));
                return CmdResult::Success;
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 + f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return CmdResult::Success;
            }
            _ => {
                return CmdResult::Error("Unknown data types for addition");
            }
        }
    }
    CmdResult::Error("Not enough or wrong values on stack")
}

fn div() -> CmdResult {
    if let Some(values) = stack::get(
        2,
        stack::AcceptedTypes::FLOAT | stack::AcceptedTypes::FRACTIONS,
    ) {
        match (&values[0], &values[1]) {
            (
                stack::StackValue::Fraction((num1, denom1)),
                stack::StackValue::Fraction((num2, denom2)),
            ) => {
                if *num2 == 0 {
                    return CmdResult::Error("Division by zero");
                }
                let result_num = num1 * denom2;
                let result_denom = denom1 * num2;
                stack::drop(2);
                stack::push(stack::StackValue::Fraction((result_num, result_denom)));
                return CmdResult::Success;
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                if *f2 == 0.0 {
                    return CmdResult::Error("Division by zero");
                }
                let result = f1 / f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return CmdResult::Success;
            }
            _ => {
                return CmdResult::Error("Unknown data types for division");
            }
        }
    }
    CmdResult::Error("Not enough or wrong values on stack")
}

fn mul() -> CmdResult {
    if let Some(value) = stack::get(
        2,
        stack::AcceptedTypes::FLOAT | stack::AcceptedTypes::FRACTIONS,
    ) {
        match (&value[0], &value[1]) {
            (
                stack::StackValue::Fraction((num1, denom1)),
                stack::StackValue::Fraction((num2, denom2)),
            ) => {
                let result_num = num1 * num2;
                let result_denom = denom1 * denom2;
                stack::drop(2);
                stack::push(stack::StackValue::Fraction((result_num, result_denom)));
                return CmdResult::Success;
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 * f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return CmdResult::Success;
            }
            _ => {
                return CmdResult::Error("Unknown data types for multiplication");
            }
        }
    }
    CmdResult::Error("Not enough or wrong values on stack")
}

fn sub() -> CmdResult {
    if let Some(value) = stack::get(
        2,
        stack::AcceptedTypes::FLOAT | stack::AcceptedTypes::FRACTIONS,
    ) {
        match (&value[0], &value[1]) {
            (
                stack::StackValue::Fraction((num1, denom1)),
                stack::StackValue::Fraction((num2, denom2)),
            ) => {
                let result_num = num1 * denom2 - num2 * denom1;
                let result_denom = denom1 * denom2;
                stack::drop(2);
                stack::push(stack::StackValue::Fraction((result_num, result_denom)));
                return CmdResult::Success;
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 - f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return CmdResult::Success;
            }
            _ => {
                return CmdResult::Error("Unknown data types for subtraction");
            }
        }
    }
    CmdResult::Error("Not enough or wrong values on stack")
}

pub fn commands(cmd: &str) -> CmdResult {
    match cmd {
        "add" => add(),
        "sub" => sub(),
        "mul" => mul(),
        "div" => div(),
        _ => CmdResult::NoMatch,
    }
}

pub fn quick_commands(cmd: &char) -> CmdResult {
    match cmd {
        '+' => add(),
        '-' => sub(),
        '*' => mul(),
        '/' => div(),
        _ => CmdResult::NoMatch,
    }
}

pub const HELP: help::Category = help::Category {
    category: "Basic Math",
    commands: &[
        help::Cmd {
            command: "add",
            help: "Adds the top two numbers on the stack.",
        },
        help::Cmd {
            command: "sub",
            help: "Subtracts the top two numbers on the stack.",
        },
        help::Cmd {
            command: "mul",
            help: "Multiplies the top two numbers on the stack.",
        },
        help::Cmd {
            command: "div",
            help: "Divides the top two numbers on the stack.",
        },
    ],
};
