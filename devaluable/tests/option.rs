use devaluable::FromValue;
use valuable::Valuable;

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
