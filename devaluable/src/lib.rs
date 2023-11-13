use valuable::Value;

pub use devaluable_macros::FromValue;

/// Consume a [Value] to produce an instance of Self. The easiest way to  get
/// an implementation of this trait is to use the
/// [FromValue][devaluable_macros::FromValue] proc-macro.
pub trait FromValue: Sized {
    fn from_value(value: Value) -> Option<Self>;
}
