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
        if let Value::Bool(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for char {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Char(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for f32 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::F32(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for f64 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::F64(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for i128 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::I128(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for i16 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::I16(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for i32 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::I32(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for i64 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::I64(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for i8 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::I8(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for isize {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Isize(value) = value {
            Some(value)
        } else {
            None
        }
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

impl FromValue for u128 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::U128(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for u16 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::U16(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for u32 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::U32(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for u64 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::U64(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for u8 {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::U8(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for usize {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Usize(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

impl FromValue for () {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Unit = value {
            Some(())
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
