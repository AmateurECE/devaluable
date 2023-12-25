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

#[derive(PartialEq, Debug, Default, Valuable)]
struct UnnamedStruct(String);

#[derive(Default)]
struct UnnamedStructVisitor(String);
impl ::devaluable::FromValue for UnnamedStruct {
    fn from_value(value: ::valuable::Value) -> Option<Self> {
        if let ::valuable::Value::Structable(data) = value {
            match data.definition() {
                ::valuable::StructDef::Static {
                    name: "UnnamedStruct",
                    fields: ::valuable::Fields::Unnamed(_),
                    ..
                } => {
                    let mut visitor = UnnamedStructVisitor::default();
                    data.visit(&mut visitor);
                    Some(visitor.into())
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
impl ::valuable::Visit for UnnamedStructVisitor {
    fn visit_value(&mut self, _value: ::valuable::Value<'_>) {
        unreachable!()
    }

    fn visit_unnamed_fields(&mut self, values: &[::valuable::Value<'_>]) {
        let mut iter = values.iter();
        self.0 = iter
            .next()
            .and_then(|value| ::devaluable::FromValue::from_value(*value))
            .unwrap_or(::std::default::Default::default());
    }
}
impl Into<UnnamedStruct> for UnnamedStructVisitor {
    fn into(self) -> UnnamedStruct {
        UnnamedStruct(self.0)
    }
}

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
