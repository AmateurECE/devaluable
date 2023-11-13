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
