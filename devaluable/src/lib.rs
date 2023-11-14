use std::{collections::HashMap, hash::Hash, path::PathBuf};

use valuable::Value;

pub use devaluable_macros::FromValue;

/// Consume a [Value] to produce an instance of Self. The easiest way to  get
/// an implementation of this trait is to use the
/// [FromValue][devaluable_macros::FromValue] proc-macro.
pub trait FromValue: Sized {
    fn from_value(value: Value) -> Option<Self>;
}

macro_rules! primitive_from_value {
    ($name:ty, $variant:ident) => {
        impl FromValue for $name {
            fn from_value(value: Value) -> Option<Self> {
                if let Value::$variant(value) = value {
                    Some(value)
                } else {
                    None
                }
            }
        }
    };
}

primitive_from_value!(bool, Bool);
primitive_from_value!(char, Char);
primitive_from_value!(f32, F32);
primitive_from_value!(f64, F64);
primitive_from_value!(i128, I128);
primitive_from_value!(i16, I16);
primitive_from_value!(i32, I32);
primitive_from_value!(i64, I64);
primitive_from_value!(i8, I8);
primitive_from_value!(isize, Isize);
primitive_from_value!(u128, U128);
primitive_from_value!(u16, U16);
primitive_from_value!(u32, U32);
primitive_from_value!(u64, U64);
primitive_from_value!(u8, U8);
primitive_from_value!(usize, Usize);

impl FromValue for String {
    fn from_value(value: valuable::Value<'_>) -> Option<Self> {
        if let Value::String(content) = value {
            Some(content.to_string())
        } else {
            None
        }
    }
}

impl FromValue for PathBuf {
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Path(path) = value {
            Some(PathBuf::from(path))
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

struct MapCollector<'a, K, V>(pub &'a mut HashMap<K, V>)
where
    K: FromValue,
    V: FromValue;
impl<K, V> valuable::Visit for MapCollector<'_, K, V>
where
    K: FromValue + Eq + Hash,
    V: FromValue,
{
    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        if let (Some(key), Some(value)) = (K::from_value(key), V::from_value(value)) {
            self.0.insert(key, value);
        }
    }

    fn visit_value(&mut self, _: Value<'_>) {
        // Unreachable because from_value checks that the value is a Mappable,
        // and calls visit on the Mappable instance.
        unreachable!()
    }
}

impl<K, V> FromValue for HashMap<K, V>
where
    K: FromValue + Eq + Hash,
    V: FromValue,
{
    fn from_value(value: Value) -> Option<Self> {
        if let Value::Mappable(map) = value {
            let mut hash_map: HashMap<K, V> = HashMap::new();
            let mut collector = MapCollector(&mut hash_map);
            map.visit(&mut collector);
            Some(hash_map)
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
    fn visit_value(&mut self, _: Value<'_>) {
        // Unreachable because from_value checks that the value is a Listable,
        // and calls visit on the Listable instance.
        unreachable!()
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
