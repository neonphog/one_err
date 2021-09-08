use crate::*;

#[derive(Clone, PartialEq)]
pub(crate) struct OneErrInner(pub(crate) Box<Option<TopMap>>);

impl OneErrInner {
    pub fn new() -> Self {
        Self(Box::new(None))
    }

    pub fn set_field<T>(&mut self, name: Box<str>, t: T)
    where
        T: Into<Value>,
    {
        if self.0.is_none() {
            self.0 = Box::new(Some(TopMap::new()));
        }
        self.0.as_mut().as_mut().unwrap().insert(name, t.into());
    }

    pub fn get_field<'lt, V>(&'lt self, name: &str) -> Option<V>
    where
        Option<V>: From<&'lt Value>,
    {
        if let Some(f) = &*self.0 {
            match f.get(name) {
                None => None,
                Some(v) => v.into(),
            }
        } else {
            None
        }
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

impl serde::Serialize for OneErrInner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        if let Some(f) = &*self.0 {
            let mut map = serializer.serialize_map(Some(f.len()))?;
            for (n, v) in f.iter() {
                map.serialize_entry(n, v)?;
            }
            map.end()
        } else {
            serializer.serialize_map(Some(0))?.end()
        }
    }
}
