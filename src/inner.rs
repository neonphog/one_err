use crate::*;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct OneErrInner {
    pub(crate) kind: Box<str>,
    pub(crate) fields: Option<TopMap>,
}

impl OneErrInner {
    pub fn new(kind: Box<str>) -> Self {
        Self { kind, fields: None }
    }

    pub fn set_field<T>(&mut self, name: Box<str>, t: T)
    where
        T: Into<serde_json::Value>,
    {
        if self.fields.is_none() {
            self.fields = Some(TopMap::new());
        }
        self.fields.as_mut().unwrap().insert(name, t.into());
    }

    pub fn get_field(&self, name: &str) -> Option<&serde_json::Value> {
        if let Some(f) = &self.fields {
            return f.get(name);
        }
        None
    }
}

impl std::fmt::Display for OneErrInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_display(f, self)
    }
}

impl std::fmt::Debug for OneErrInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            fmt_debug_alt(f, self)
        } else {
            fmt_display(f, self)
        }
    }
}

impl std::error::Error for OneErrInner {}

impl From<OneErrInner> for std::io::Error {
    fn from(e: OneErrInner) -> Self {
        Self::new(str_kind_to_err(&e.kind), e)
    }
}

impl serde::Serialize for OneErrInner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut count = 1;

        if let Some(f) = &self.fields {
            count += f.len();
        }

        let mut map = serializer.serialize_map(Some(count))?;

        map.serialize_entry("error", &self.kind)?;

        if let Some(f) = &self.fields {
            for (n, v) in f.iter() {
                map.serialize_entry(n, v)?;
            }
        }

        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for OneErrInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = OneErrInner;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("OneErr Data")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut kind_str: Option<Box<str>> = None;
                let mut inner_map = TopMap::new();
                loop {
                    let r: Option<(Box<str>, serde_json::Value)> = access.next_entry()?;
                    match r {
                        None => break,
                        Some((k, v)) => {
                            if &*k == ERROR {
                                match v {
                                    serde_json::Value::String(s) => {
                                        kind_str = Some(s.into());
                                    }
                                    _ => return Err(serde::de::Error::custom("invalid type for 'error' field")),
                                }
                            } else {
                                inner_map.insert(k, v);
                            }
                        }
                    }
                }
                match kind_str {
                    None => Err(serde::de::Error::custom("invalid error data contains no 'error' field")),
                    Some(kind_str) => {
                        Ok(OneErrInner {
                            kind: kind_str,
                            fields: if inner_map.is_empty() {
                                None
                            } else {
                                Some(inner_map)
                            },
                        })
                    }
                }
            }
        }

        deserializer.deserialize_map(V)
    }
}
