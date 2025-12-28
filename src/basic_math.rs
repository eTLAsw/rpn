use crate::stack;

// Basic mathematical operations that work with both floats and fractions.

pub fn add() -> Result<(), &'static str> {
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
                return Ok(());
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 + f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return Ok(());
            }
            _ => {
                return Err("Unknown data types for addition");
            }
        }
    }
    Err("Not enough or wrong values on stack")
}

pub fn div() -> Result<(), &'static str> {
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
                    return Err("Division by zero");
                }
                let result_num = num1 * denom2;
                let result_denom = denom1 * num2;
                stack::drop(2);
                stack::push(stack::StackValue::Fraction((result_num, result_denom)));
                return Ok(());
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                if *f2 == 0.0 {
                    return Err("Division by zero");
                }
                let result = f1 / f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return Ok(());
            }
            _ => {
                return Err("Unknown data types for division");
            }
        }
    }
    Err("Not enough or wrong values on stack")
}

pub fn mul() -> Result<(), &'static str> {
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
                return Ok(());
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 * f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return Ok(());
            }
            _ => {
                return Err("Unknown data types for multiplication");
            }
        }
    }
    Err("Not enough or wrong values on stack")
}

pub fn sub() -> Result<(), &'static str> {
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
                return Ok(());
            }
            (stack::StackValue::Float(f1), stack::StackValue::Float(f2)) => {
                let result = f1 - f2;
                stack::drop(2);
                stack::push(stack::StackValue::Float(result));
                return Ok(());
            }
            _ => {
                return Err("Unknown data types for subtraction");
            }
        }
    }
    Err("Not enough or wrong values on stack")
}
