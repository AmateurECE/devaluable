use devaluable::FromValue;
use valuable::Valuable;

#[derive(PartialEq, Debug, Valuable, FromValue)]
enum TestEnum {
    Variant,
    VariantWithUnnamedValue(bool),
    VariantWithNamedValue { test: bool },
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
