/// Value type for additional data fields on OneErr instances.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Empty / No Data
    Null,

    /// Boolean Type
    Bool(bool),

    /// Signed Integer Type
    I64(i64),

    /// Unsigned Integer Type
    U64(u64),

    /// Floating Point Type
    F64(f64),

    /// String Type
    String(Box<str>),
}

impl Value {
    /// If this value is a bool type, return that bool.
    pub fn as_bool(&self) -> Option<bool> {
        self.into()
    }

    /// If this value is an i64 type, return that i64.
    pub fn as_i64(&self) -> Option<i64> {
        self.into()
    }

    /// If this value is a u64 type, return that u64.
    pub fn as_u64(&self) -> Option<u64> {
        self.into()
    }

    /// If this value is an f64 type, return that f64.
    pub fn as_f64(&self) -> Option<f64> {
        self.into()
    }

    /// If this value is a str type, return that str.
    pub fn as_str(&self) -> Option<&str> {
        self.into()
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<u64> for Value {
    fn from(u: u64) -> Self {
        Self::U64(u)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Self::String(s.as_str().into())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

impl From<Box<str>> for Value {
    fn from(s: Box<str>) -> Self {
        Self::String(s)
    }
}

impl<'lt> From<&'lt Value> for Option<bool> {
    fn from(v: &'lt Value) -> Self {
        match v {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl<'lt> From<&'lt Value> for Option<i64> {
    fn from(v: &'lt Value) -> Self {
        match v {
            Value::I64(i) => Some(*i),
            _ => None,
        }
    }
}

impl<'lt> From<&'lt Value> for Option<u64> {
    fn from(v: &'lt Value) -> Self {
        match v {
            Value::U64(u) => Some(*u),
            _ => None,
        }
    }
}

impl<'lt> From<&'lt Value> for Option<f64> {
    fn from(v: &'lt Value) -> Self {
        match v {
            Value::F64(f) => Some(*f),
            _ => None,
        }
    }
}

impl<'lt> From<&'lt Value> for Option<&'lt str> {
    fn from(v: &'lt Value) -> Self {
        match v {
            Value::String(s) => Some(&*s),
            _ => None,
        }
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::I64(i) => serializer.serialize_i64(*i),
            Self::U64(u) => serializer.serialize_u64(*u),
            Self::F64(f) => serializer.serialize_f64(*f),
            Self::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Value;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("any valid JSON primitive (arrays / objects not yet supported)")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::I64(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::U64(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::F64(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value.into()))
            }

            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }
        }

        deserializer.deserialize_any(V)
    }
}
