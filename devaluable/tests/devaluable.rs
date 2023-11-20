#![allow(non_upper_case_globals)]

use devaluable::FromValue;
use std::{collections::HashMap, path::{Path, PathBuf}};
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
    let mut input = HashMap::new();
    input.insert("Foo".to_string(), "Bar".to_string());
    input.insert("Baz".to_string(), "Luhrmann".to_string());
    let output = HashMap::<String, String>::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn path() {
    let input = Path::new("a.txt");
    let output = PathBuf::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn string() {
    let input = "Test".to_string();
    let output = String::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn enumerable() {
    todo!()
}

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct OptionStruct {
    member: Option<i64>,
}

#[test]
fn option_struct() {
    let input = OptionStruct {
        member: Some(12),
    };
    let output = OptionStruct::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn option_some() {
    let input: Option<bool> = Some(true);
    let output = Option::<bool>::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn option_none() {
    let input: Option<bool> = None;
    let output = Option::<bool>::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct CompositeStruct {
    foo: i64,
    bar: Vec<i64>,
    baz: HashMap<String, String>,
    test: PathBuf,
}

#[test]
fn composite_struct() {
    let mut baz = HashMap::<String, String>::new();
    baz.insert("Name".to_string(), "P. Sherman".to_string());
    baz.insert("Street".to_string(), "42 Wallaby Way".to_string());
    let test = PathBuf::from("a.txt");
    let input = CompositeStruct {
        foo: 12,
        bar: vec![1, 2, 3, 4, 5],
        baz,
        test,
    };

    let output = CompositeStruct::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
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
