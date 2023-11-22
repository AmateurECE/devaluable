#![allow(non_upper_case_globals)]

use devaluable::FromValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
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
struct UnnamedStructVisitor(UnnamedStruct);
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
                        Some(visitor.0)
                    },
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
        self.0.0 = iter
            .next()
            .and_then(|value| ::devaluable::FromValue::from_value(*value))
            .unwrap_or(::std::default::Default::default());
    }
}

#[test]
fn unnamed_struct() {
    let input = UnnamedStruct("Hello, World!".to_string());
    let output = UnnamedStruct::from_value(input.as_value());
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

#[derive(PartialEq, Debug, Valuable)]
enum TestEnum {
    Variant,
    VariantWithUnnamedValue(bool),
    VariantWithNamedValue { test: bool },
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::Variant
    }
}

#[derive(Default)]
struct VariantWithUnnamedValueVisitor(bool);
impl valuable::Visit for VariantWithUnnamedValueVisitor {
    fn visit_value(&mut self, _value: valuable::Value<'_>) {
        unreachable!()
    }

    fn visit_unnamed_fields(&mut self, values: &[valuable::Value<'_>]) {
        let mut iter = values.iter();
        self.0 = iter
            .next()
            .and_then(|value| bool::from_value(*value))
            .unwrap_or(::std::default::Default::default());
    }
}
impl Into<TestEnum> for VariantWithUnnamedValueVisitor {
    fn into(self) -> TestEnum {
        TestEnum::VariantWithUnnamedValue(self.0)
    }
}

#[derive(Default)]
struct VariantWithNamedValueVisitor {
    test: bool,
}
impl valuable::Visit for VariantWithNamedValueVisitor {
    fn visit_value(&mut self, _value: valuable::Value<'_>) {
        unreachable!()
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        named_values
            .iter()
            .for_each(|(field, value)| match field.name() {
                "test" => {
                    let result: Option<bool> = ::devaluable::FromValue::from_value(*value);
                    if let Some(result) = result {
                        self.test = result;
                    }
                }
                _ => {}
            });
    }
}
impl Into<TestEnum> for VariantWithNamedValueVisitor {
    fn into(self) -> TestEnum {
        let Self { test } = self;
        TestEnum::VariantWithNamedValue { test }
    }
}

impl FromValue for TestEnum {
    fn from_value(value: valuable::Value) -> Option<Self> {
        if let valuable::Value::Enumerable(enumerable) = value {
            if let ("TestEnum", valuable::Variant::Static(variant)) =
                (enumerable.definition().name(), enumerable.variant())
            {
                match (variant.name(), variant.fields()) {
                    ("Variant", _) => Some(TestEnum::Variant),
                    ("VariantWithUnnamedValue", valuable::Fields::Unnamed(_)) => {
                        let mut visitor = VariantWithUnnamedValueVisitor::default();
                        enumerable.visit(&mut visitor);
                        Some(visitor.into())
                    }
                    ("VariantWithNamedValue", valuable::Fields::Named(_)) => {
                        let mut visitor = VariantWithNamedValueVisitor::default();
                        enumerable.visit(&mut visitor);
                        Some(visitor.into())
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
fn enumerable_fieldless() {
    let input = TestEnum::Variant;
    let output = TestEnum::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn enumerable_unnamed_fields() {
    let input = TestEnum::VariantWithUnnamedValue(false);
    let output = TestEnum::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[test]
fn enumerable_named_fields() {
    let input = TestEnum::VariantWithNamedValue { test: false };
    let output = TestEnum::from_value(input.as_value());
    assert!(output.is_some());
    assert_eq!(input, output.unwrap());
}

#[derive(PartialEq, Debug, Default, Valuable, FromValue)]
struct OptionStruct {
    member: Option<i64>,
}

#[test]
fn option_struct() {
    let input = OptionStruct { member: Some(12) };
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
