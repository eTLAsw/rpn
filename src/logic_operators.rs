use crate::stack;

// Logic areas only work on integers, which is a special case of fractions. See stack.rs for more info.

pub fn and() -> Result<(), &'static str> {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return Err("AND operation requires two integer values"),
        };

        let result = a & b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        Ok(())
    } else {
        Err("Not enough values on stack")
    }
}

pub fn not() -> Result<(), &'static str> {
    if let Some(values) = stack::get(1, stack::AcceptedTypes::INTEGERS) {
        let a = match &values[0] {
            stack::StackValue::Integer(a) => *a,
            _ => return Err("NOT operation requires an integer value"),
        };

        let result = !a;

        stack::drop(1);
        stack::push(stack::StackValue::Integer(result));

        Ok(())
    } else {
        Err("Not enough values on stack")
    }
}
pub fn or() -> Result<(), &'static str> {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return Err("OR operation requires two integer values"),
        };

        let result = a | b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        Ok(())
    } else {
        Err("Not enough values on stack")
    }
}

pub fn xor() -> Result<(), &'static str> {
    if let Some(values) = stack::get(2, stack::AcceptedTypes::INTEGERS) {
        let (a, b) = match (&values[0], &values[1]) {
            (stack::StackValue::Integer(a), stack::StackValue::Integer(b)) => (*a, *b),
            _ => return Err("XOR operation requires two integer values"),
        };

        let result = a ^ b;

        stack::drop(2);
        stack::push(stack::StackValue::Integer(result));

        Ok(())
    } else {
        Err("Not enough values on stack")
    }
}
