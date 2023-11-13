use valuable::Value;

pub use devaluable_macros::FromValue;

/// Consume a [Value] to produce an instance of Self. The easiest way to  get
/// an implementation of this trait is to use the
/// [FromValue][devaluable_macros::FromValue] proc-macro.
pub trait FromValue: Sized {
    fn from_value(value: Value) -> Option<Self>;
}

impl FromValue for bool {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for char {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for f32 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for f64 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for i128 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for i16 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for i32 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for i64 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for i8 {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for isize {
    fn from_value(value: Value) -> Option<Self> {
        todo!()
    }
}

impl FromValue for String {
    fn from_value(value: valuable::Value<'_>) -> Option<Self> {
        if let Value::String(content) = value {
            Some(content.to_string())
        } else {
            None
        }
    }
}

struct VecCollector<'a, V>(pub &'a mut Vec<V>)
where
    V: FromValue;

impl<V> valuable::Visit for VecCollector<'_, V>
where
    V: FromValue,
{
    fn visit_value(&mut self, value: Value<'_>) {
        if let Some(content) = V::from_value(value) {
            self.0.push(content);
        }
    }

    fn visit_primitive_slice(&mut self, slice: valuable::Slice<'_>) {
        self.0.extend(slice.iter().filter_map(|v| V::from_value(v)))
    }
}

impl<V> FromValue for Vec<V>
where
    V: FromValue,
{
    fn from_value(value: Value<'_>) -> Option<Self> {
        if let Value::Listable(list) = value {
            let mut vector = Vec::<V>::new();
            let mut collector = VecCollector(&mut vector);
            list.visit(&mut collector);
            Some(vector)
        } else {
            None
        }
    }
}
