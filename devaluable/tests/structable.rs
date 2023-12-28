#![allow(non_upper_case_globals)]

use devaluable::FromValue;
use std::{collections::HashMap, path::PathBuf};
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

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct UnnamedStruct(String);

#[test]
fn unnamed_struct() {
    let input = UnnamedStruct("Hello, World!".to_string());
    let output = UnnamedStruct::from_value(input.as_value());
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
