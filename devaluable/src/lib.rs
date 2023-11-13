use valuable::Value;

pub use devaluable_macros::FromValue;

/// Consume a [Value] to produce an instance of Self. The easiest way to  get
/// an implementation of this trait is to use the
/// [FromValue][devaluable_macros::FromValue] proc-macro.
pub trait FromValue {
    fn from_value(value: Value) -> Self;
}

#[cfg(test)]
#[allow(non_upper_case_globals)]
mod tests {
    use super::*;
    use valuable::Valuable;

    #[derive(PartialEq, Debug, Valuable, FromValue)]
    struct OneStringStruct {
        message: String,
    }

    #[test]
    fn one_string_struct() {
        let input = OneStringStruct {
            message: "Test".to_string(),
        };
        let output = OneStringStruct::from_value(input.as_value());
        assert_eq!(input, output);
    }
}
