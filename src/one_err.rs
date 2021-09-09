use super::*;

const ERROR: &str = "error";
const OS: &str = "os";
const SOURCE: &str = "source";
const BACKTRACE: &str = "backtrace";
const MESSAGE: &str = "message";

/// OneErr to rule them all. See crate docs for usage.
pub struct OneErr(std::io::Error);

impl std::fmt::Display for OneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_display(f, self)
    }
}

impl std::fmt::Debug for OneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            fmt_debug_alt(f, self)
        } else {
            fmt_display(f, self)
        }
    }
}

impl std::error::Error for OneErr {}

impl Clone for OneErr {
    fn clone(&self) -> Self {
        let k = self.0.kind();
        Self(std::io::Error::new(k, self.priv_as_inner().clone()))
    }
}

impl PartialEq for OneErr {
    fn eq(&self, oth: &Self) -> bool {
        if self.str_kind() != oth.str_kind() {
            return false;
        }
        self.priv_as_inner() == oth.priv_as_inner()
    }
}

impl Eq for OneErr {}

impl From<std::io::ErrorKind> for OneErr {
    fn from(k: std::io::ErrorKind) -> Self {
        Self(std::io::Error::new(k, OneErrInner::new()))
    }
}

impl From<i32> for OneErr {
    fn from(e: i32) -> Self {
        ErrNo::from(e).into()
    }
}

impl From<ErrNo> for OneErr {
    fn from(e: ErrNo) -> Self {
        let k: std::io::ErrorKind = (&e).into();

        let mut inner = OneErrInner::new();
        if let std::io::ErrorKind::Other = k {
            if let ErrNo::Other = e {
                /* pass */
            } else {
                inner.set_field(OS.into(), i32::from(&e) as i64);
            }
        }

        Self(std::io::Error::new(k, inner))
    }
}

impl From<std::io::Error> for OneErr {
    fn from(e: std::io::Error) -> Self {
        // we have to be careful about this one...
        // we need the inner to be a OneErrInner.

        // if our inner data is already a OneErrorInner,
        // we can just wrap it up, call it good.
        if let Some(r) = e.get_ref() {
            if r.downcast_ref::<OneErrInner>().is_some() {
                return Self(e);
            }
        }

        // if there is an os errno, use that
        if let Some(e) = e.raw_os_error() {
            return e.into();
        }

        // otherwise, just go off the io::ErrorKind
        let message = format!("{}", e);
        let mut inner = OneErrInner::new();
        inner.set_field(MESSAGE.into(), message);
        Self(std::io::Error::new(e.kind(), inner))
    }
}

impl From<()> for OneErr {
    fn from(_: ()) -> Self {
        std::io::ErrorKind::Other.into()
    }
}

impl From<String> for OneErr {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<&String> for OneErr {
    fn from(s: &String) -> Self {
        s.as_str().into()
    }
}

impl From<&str> for OneErr {
    fn from(s: &str) -> Self {
        OneErr::new(s)
    }
}

impl From<OneErr> for std::io::Error {
    fn from(e: OneErr) -> Self {
        e.0
    }
}

impl From<&OneErr> for std::io::Error {
    fn from(e: &OneErr) -> Self {
        e.clone().0
    }
}

impl std::ops::Deref for OneErr {
    type Target = std::io::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<std::io::Error> for OneErr {
    fn as_ref(&self) -> &std::io::Error {
        &self.0
    }
}

impl serde::Serialize for OneErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let (kind, os, inner) = self.priv_as_parts();
        let error: &str = if let Some(os) = os {
            if let ErrNo::Other = os {
                if let Some(f) = &*inner.0 {
                    if let Some(e_str) = f.get(ERROR) {
                        if let Some(e_str) = e_str.as_str() {
                            e_str
                        } else {
                            <&'static str>::from(os)
                        }
                    } else {
                        <&'static str>::from(os)
                    }
                } else {
                    <&'static str>::from(os)
                }
            } else {
                <&'static str>::from(os)
            }
        } else {
            if let std::io::ErrorKind::Other = kind {
                if let Some(f) = &*inner.0 {
                    if let Some(e_str) = f.get(ERROR) {
                        if let Some(e_str) = e_str.as_str() {
                            e_str
                        } else {
                            err_kind_to_str(kind)
                        }
                    } else {
                        err_kind_to_str(kind)
                    }
                } else {
                    err_kind_to_str(kind)
                }
            } else {
                err_kind_to_str(kind)
            }
        };

        let mut count = 1;
        if let Some(f) = &*inner.0 {
            count += f.len();

            if f.contains_key(ERROR) {
                count -= 1;
            }

            if f.contains_key(OS) {
                count -= 1;
            }
        }

        let mut map = serializer.serialize_map(Some(count))?;

        map.serialize_entry(ERROR, &error)?;

        if let Some(f) = &*inner.0 {
            for (n, v) in f.iter() {
                match &**n {
                    ERROR | OS => continue,
                    _ => map.serialize_entry(n, v)?,
                }
            }
        }

        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for OneErr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = TopMap;

            fn expecting(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                f.write_str("OneErr Map")
            }

            fn visit_map<A>(
                self,
                mut access: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut out_map = TopMap::new();
                loop {
                    let r: Option<(Box<str>, Value)> = access.next_entry()?;
                    match r {
                        None => break,
                        Some((k, v)) => {
                            out_map.insert(k, v);
                        }
                    }
                }
                Ok(out_map)
            }
        }
        let mut top_map: TopMap = deserializer.deserialize_map(V)?;
        if let Some(error) = top_map.remove(ERROR) {
            if let Some(error) = error.as_str() {
                let (kind, os) = parse_err_str(error);
                if let Some(os) = os {
                    if let std::io::ErrorKind::Other = kind {
                        top_map
                            .insert(OS.into(), (i32::from(&os) as i64).into());
                    }
                } else {
                    if let std::io::ErrorKind::Other = kind {
                        if error != "Other" && error != "EOTHER" {
                            top_map.insert(ERROR.into(), error.into());
                        }
                    }
                }
                if top_map.is_empty() {
                    Ok(Self(std::io::Error::new(kind, OneErrInner::new())))
                } else {
                    Ok(Self(std::io::Error::new(
                        kind,
                        OneErrInner(Box::new(Some(top_map))),
                    )))
                }
            } else {
                Err(serde::de::Error::custom("required 'error' field is a str"))
            }
        } else {
            Err(serde::de::Error::custom("required 'error' field"))
        }
    }
}

impl std::str::FromStr for OneErr {
    type Err = OneErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| {
            OneErr::with_message(crate::io_error::INVALID_DATA_STR, &e)
        })
    }
}

impl OneErr {
    /// Create a new OneErr error instance.
    pub fn new<K>(kind: K) -> Self
    where
        K: std::fmt::Display,
    {
        let kind_str = kind.to_string();
        let (kind, os) = parse_err_str(&kind_str);
        if let Some(os) = os {
            if let ErrNo::Other = os {
                let mut inner = OneErrInner::new();
                inner.set_field(ERROR.into(), kind_str);

                Self(std::io::Error::new(kind, inner))
            } else {
                os.into()
            }
        } else {
            if let std::io::ErrorKind::Other = kind {
                let mut inner = OneErrInner::new();
                inner.set_field(ERROR.into(), kind_str);

                Self(std::io::Error::new(kind, inner))
            } else {
                kind.into()
            }
        }
    }

    /// Create a new OneErr error instance with a message.
    pub fn with_message<K, M>(kind: K, message: M) -> Self
    where
        K: std::fmt::Display,
        M: std::fmt::Display,
    {
        let mut err = Self::new(kind);
        err.priv_as_inner_mut()
            .set_field(MESSAGE.into(), message.to_string());
        err
    }

    /// Get the std::io::ErrorKind associated with this instance.
    pub fn io_kind(&self) -> std::io::ErrorKind {
        self.0.kind()
    }

    /// Get the ErrNo associated with this instance, if any.
    pub fn errno(&self) -> ErrNo {
        let inner = self.priv_as_inner();

        if let Some(os) = inner.get_field::<i64>(OS) {
            return ErrNo::from(os as i32);
        }

        ErrNo::from(self.0.kind())
    }

    /// Get the &str kind associated with this instance.
    /// This can be more descriptive where `io_kind()` or `errno()`
    /// might return 'Other'.
    pub fn str_kind(&self) -> &str {
        let inner = self.priv_as_inner();

        if let Some(e_str) = inner.get_field::<&str>(ERROR) {
            return e_str;
        }

        if let Some(os) = inner.get_field::<i64>(OS) {
            let os = ErrNo::from(os as i32);
            return <&'static str>::from(os);
        }

        err_kind_to_str(self.0.kind())
    }

    /// Get a reference to the inner std::io::Error of this instance.
    pub fn as_io(&self) -> &std::io::Error {
        self.as_ref()
    }

    /// Set an additional data field on this OneErr.
    /// Will panic on reserved names: "error", "os", "source",
    /// "backtrace", "message".
    pub fn set_field<K, T>(&mut self, name: &K, t: T) -> &mut Self
    where
        K: ?Sized + std::fmt::Display,
        T: Into<Value>,
    {
        let name = name.to_string().into_boxed_str();
        match &*name {
            ERROR | OS | SOURCE | BACKTRACE | MESSAGE => {
                panic!("field name '{}' is reserved", name)
            }
            _ => (),
        }

        self.priv_as_inner_mut().set_field(name, t);
        self
    }

    /// Get the message associated with this instance, or empty string.
    pub fn get_message(&self) -> Option<&str> {
        self.get_field(MESSAGE)
    }

    /// Get the value of an additional field associated with
    /// this error, or None if no such field exists. Valid output types:
    /// `&str`, `bool`, `i64`, `u64`, and `f64`.
    pub fn get_field<'lt, R, V>(&'lt self, name: R) -> Option<V>
    where
        R: AsRef<str>,
        Option<V>: From<&'lt Value>,
    {
        self.priv_as_inner().get_field(name.as_ref())
    }
}

// -- private -- //

impl OneErr {
    pub(crate) fn priv_as_inner(&self) -> &OneErrInner {
        // we can do all these unwraps because we control
        // our inner type to always be valid and of OneErrInner type.
        self.0
            .get_ref()
            .unwrap()
            .downcast_ref::<OneErrInner>()
            .unwrap()
    }

    pub(crate) fn priv_as_parts(
        &self,
    ) -> (std::io::ErrorKind, Option<ErrNo>, &OneErrInner) {
        let kind = self.0.kind();
        let inner = self.priv_as_inner();
        let os = inner.get_field::<i64>(OS).map(|e| ErrNo::from(e as i32));
        (kind, os, inner)
    }

    pub(crate) fn priv_as_inner_mut(&mut self) -> &mut OneErrInner {
        // we can do all these unwraps because we control
        // our inner type to always be valid and of OneErrInner type.
        self.0
            .get_mut()
            .unwrap()
            .downcast_mut::<OneErrInner>()
            .unwrap()
    }
}
