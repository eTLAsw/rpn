use crate::stack;

pub fn drop() -> Result<(), &'static str> {
    stack::drop(1);
    Ok(())
}

pub fn dup() -> Result<(), &'static str> {
    let stack = stack::get_values(1);
    if let Some(value) = stack {
        stack::push(value[0]);
    }
    Ok(())
}

pub fn swap() -> Result<(), &'static str> {
    let stack = stack::get_values(2);
    if let Some(values) = stack {
        stack::drop(2);
        stack::push(values[1]);
        stack::push(values[0]);
    }
    Ok(())
}

pub fn undo() -> Result<(), &'static str> {
    stack::undo();
    Ok(())
}
