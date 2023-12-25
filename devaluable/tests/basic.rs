use devaluable::FromValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use valuable::Valuable;

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
