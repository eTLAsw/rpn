use bitflags::bitflags;
use std::sync::Mutex;

use crate::modes;

type FractionType = (i128, i128);

bitflags! {
    pub struct AcceptedTypes: usize {
        const INTEGERS = 0b00000001;
        const FRACTIONS = 0b00000010;
        const FLOAT = 0b00000100;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StackValue {
    Integer(i128),
    Float(f64),
    Fraction(FractionType),
}

#[derive(Clone)]
struct Stack {
    values: Vec<StackValue>,
}

static STACK: Mutex<Stack> = Mutex::new(Stack { values: Vec::new() });
static STACKBACKUP: Mutex<Vec<Stack>> = Mutex::new(Vec::new());

fn backup_stack(stack: &Stack) {
    let mut backup = STACKBACKUP.lock().unwrap();
    backup.push(stack.clone());

    if backup.len() > 10 {
        backup.remove(0);
    }
}

pub fn drop(count: usize) {
    let mut stack = STACK.lock().unwrap();

    if count > stack.values.len() {
        return;
    }

    let start = stack.values.len() - count;
    backup_stack(&stack);
    stack.values.drain(start..);
}

pub fn undo() {
    let mut stack = STACK.lock().unwrap();
    let mut backup = STACKBACKUP.lock().unwrap();

    if let Some(previous) = backup.pop() {
        *stack = previous;
    }
}

pub fn push(value: StackValue) {
    let mut stack = STACK.lock().unwrap();

    let mut value = value;

    if let StackValue::Float(f) = value {
        if (f * 1000.0).fract() == 0.0
            && f * 1000.0 >= (i128::MIN as f64)
            && f * 1000.0 <= (i128::MAX as f64)
        {
            // If float is an integer within i128 range, convert to fraction
            value = StackValue::Fraction(((f * 1000.0) as i128, 1000));
        }
    }

    if let StackValue::Fraction((num, denom)) = value {
        let gcd = |a: i128, b: i128| {
            let mut a = a.abs();
            let mut b = b.abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        };

        let divisor = gcd(num, denom);
        let simplified = (num / divisor, denom / divisor);
        if simplified.1 < 0 {
            // Ensure denominator is positive
            value = StackValue::Fraction((-simplified.0, -simplified.1));
        } else {
            value = StackValue::Fraction(simplified);
        }
    }

    // Handle ints as fractions to avoid unnecessary complexity in all operations
    if let StackValue::Integer(i) = value {
        if i >= i128::MIN && i <= i128::MAX {
            // Convert integer to fraction
            value = StackValue::Fraction((i, 1));
        }
    }

    backup_stack(&stack);
    stack.values.push(value);
}

pub fn get(count: usize, types: AcceptedTypes) -> Option<Vec<StackValue>> {
    let stack = STACK.lock().unwrap();

    if count > stack.values.len() {
        return None;
    }

    let start = stack.values.len() - count;

    if types.contains(AcceptedTypes::INTEGERS) {
        let mut result = Vec::with_capacity(count);
        for v in &stack.values[start..] {
            match v {
                StackValue::Fraction(frac) => {
                    if frac.1 != 1 {
                        break;
                    }
                    result.push(StackValue::Integer(frac.0));
                }
                _ => {
                    break;
                }
            }
        }
        if result.len() == count {
            return Some(result);
        }
    }

    if types.contains(AcceptedTypes::FRACTIONS) {
        let mut result = Vec::with_capacity(count);
        for v in &stack.values[start..] {
            match v {
                StackValue::Fraction(frac) => result.push(StackValue::Fraction(*frac)),
                _ => {
                    break;
                }
            }
        }
        if result.len() == count {
            return Some(result);
        }
    }

    if types.contains(AcceptedTypes::FLOAT) {
        let mut result = Vec::with_capacity(count);
        for v in &stack.values[start..] {
            match v {
                StackValue::Fraction(frac) => {
                    result.push(StackValue::Float(frac.0 as f64 / frac.1 as f64))
                }
                StackValue::Float(f) => result.push(StackValue::Float(*f)),
                _ => {
                    break;
                }
            }
        }
        if result.len() == count {
            return Some(result);
        }
    }

    None
}

pub fn get_values(count: usize) -> Option<Vec<StackValue>> {
    let stack = STACK.lock().unwrap();

    if count > stack.values.len() {
        return None;
    }

    let start = stack.values.len() - count;
    Some(stack.values[start..].to_vec())
}

fn get_binary_representation(value: i128) -> String {
    let u = value as u128;
    match (modes::get_binary_mode(), modes::get_binary_width()) {
        (modes::BinaryMode::Signed, modes::BinaryWidth::W8) => {
            if let Ok(v) = i8::try_from(value) {
                return format!(" = {:02x} {:08b}", v, v);
            }
        }
        (modes::BinaryMode::Signed, modes::BinaryWidth::W16) => {
            if let Ok(v) = i16::try_from(value) {
                return format!(" = {:04x} {:016b}", v, v);
            }
        }
        (modes::BinaryMode::Signed, modes::BinaryWidth::W32) => {
            if let Ok(v) = i32::try_from(value) {
                return format!(" = {:08x} {:032b}", v, v);
            }
        }
        (modes::BinaryMode::Signed, modes::BinaryWidth::W64) => {
            if let Ok(v) = i64::try_from(value) {
                return format!(" = {:016x} {:064b}", v, v);
            }
        }
        (modes::BinaryMode::Unsigned, modes::BinaryWidth::W8) => {
            if let Ok(v) = u8::try_from(u) {
                return format!(" = {:02x} {:08b}", v, v);
            }
        }
        (modes::BinaryMode::Unsigned, modes::BinaryWidth::W16) => {
            if let Ok(v) = u16::try_from(u) {
                return format!(" = {:04x} {:016b}", v, v);
            }
        }
        (modes::BinaryMode::Unsigned, modes::BinaryWidth::W32) => {
            if let Ok(v) = u32::try_from(u) {
                return format!(" = {:08x} {:032b}", v, v);
            }
        }
        (modes::BinaryMode::Unsigned, modes::BinaryWidth::W64) => {
            if let Ok(v) = u64::try_from(u) {
                return format!(" = {:016x} {:064b}", v, v);
            }
        }
    }
    String::new()
}

pub fn get_stack_contents() -> Vec<String> {
    let stack = STACK.lock().unwrap();
    let mut output: Vec<String> = Vec::new();
    for (i, v) in stack.values.iter().enumerate() {
        match v {
            StackValue::Float(f) => {
                output.push(format!("{:>4}: {:.6}", stack.values.len() - i - 1, f))
            }
            StackValue::Fraction((num, denom)) => {
                if *denom == 1 {
                    let buf: String = format!(
                        "{:>4}: {}{}",
                        stack.values.len() - i - 1,
                        num,
                        get_binary_representation(*num)
                    );
                    output.push(format!("{}", buf));
                } else {
                    output.push(format!(
                        "{:>4}: {}/{} = {}",
                        stack.values.len() - i - 1,
                        num,
                        denom,
                        *num as f64 / *denom as f64
                    ));
                }
            }
            _ => {
                // Unknown type, this shouldn't happen?
                output.push("Error".to_string());
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_drop() {
        push(StackValue::Integer(42));
        push(StackValue::Fraction((3, 4)));
        push(StackValue::Float(2.5));

        let values = get_values(3).unwrap();
        assert_eq!(values.len(), 3);

        drop(3);
        assert!(get_values(1).is_none());

        assert_eq!(get_stackbackup_length(), 4);
    }
}
