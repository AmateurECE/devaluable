use devaluable::FromValue;
use valuable::Valuable;

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
