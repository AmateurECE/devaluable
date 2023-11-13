#![allow(non_upper_case_globals)]

use devaluable::FromValue;
use valuable::Valuable;

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct OneStringStruct {
    message: String,
}

#[test]
fn one_string_struct() {
    let input = OneStringStruct {
        message: "Test".to_string(),
    };
    let output = OneStringStruct::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct OneVecStruct {
    contents: Vec<i64>,
}

#[test]
fn one_vec_struct() {
    let input = OneVecStruct {
        contents: vec![0, 1, 2, 3],
    };
    let output = OneVecStruct::from_value(input.as_value());
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

#[test]
fn listable() {
    let input: Vec<i64> = vec![0, 1, 2, 3];
    let output = Vec::<i64>::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn mappable() {
    todo!()
}

#[test]
fn path() {
    todo!()
}

#[test]
fn string() {
    let input = "Test".to_string();
    let output = String::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn primitive_unit() {
    let input = ();
    let output: Option<()> = FromValue::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn enumerable() {
    todo!()
}

#[test]
fn option_struct() {
    todo!()
}

#[test]
fn option() {
    todo!()
}

#[test]
fn composite_struct() {
    todo!()
}

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
