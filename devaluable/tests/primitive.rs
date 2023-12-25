use devaluable::FromValue;
use valuable::Valuable;

#[test]
fn primitive_u128() {
    let input = 12;
    let output = u128::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_u16() {
    let input = 12;
    let output = u16::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_u32() {
    let input = 12;
    let output = u32::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_u64() {
    let input = 12;
    let output = u64::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_u8() {
    let input = 12;
    let output = u8::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_usize() {
    let input = 12;
    let output = usize::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}
#[test]
fn primitive_bool() {
    let input = true;
    let output = bool::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_char() {
    let input = 'a';
    let output = char::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_f32() {
    let input = 1.1;
    let output = f32::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_f64() {
    let input = 1.1;
    let output = f64::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_i128() {
    let input = 12;
    let output = i128::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_i16() {
    let input = 12;
    let output = i16::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_i32() {
    let input = 12;
    let output = i32::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_i64() {
    let input = 12;
    let output = i64::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_i8() {
    let input = 12;
    let output = i8::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_isize() {
    let input = 12;
    let output = isize::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}
