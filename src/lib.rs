#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
//! OneErr to rule them all

const ERROR: &str = "error";
const OS: &str = "os";
const SOURCE: &str = "source";
const BACKTRACE: &str = "backtrace";
const MESSAGE: &str = "message";

mod util;
use util::*;

mod inner;
use inner::*;

/// OneErr to rule them all
pub struct OneErr(std::io::Error);

impl std::fmt::Display for OneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.priv_as_inner().fmt(f)
    }
}

impl std::fmt::Debug for OneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.priv_as_inner().fmt(f)
    }
}

impl std::error::Error for OneErr {}

impl Clone for OneErr {
    fn clone(&self) -> Self {
        self.priv_as_inner().clone().into()
    }
}

impl PartialEq for OneErr {
    fn eq(&self, oth: &Self) -> bool {
        self.priv_as_inner() == oth.priv_as_inner()
    }
}

impl Eq for OneErr {}

impl From<std::io::ErrorKind> for OneErr {
    fn from(k: std::io::ErrorKind) -> Self {
        Self(std::io::Error::new(
            k,
            OneErrInner::new(err_kind_to_str(k).into()),
        ))
    }
}

impl From<std::io::Error> for OneErr {
    fn from(e: std::io::Error) -> Self {
        // we have to be careful about this one...
        // we need the inner to be a OneErrInner.

        // first, capture some things just in case
        let kind = e.kind();
        let os = e.raw_os_error();

        // see if the inner type is already a OneErrInner
        let message = if e.get_ref().is_some() {
            match e.into_inner().unwrap().downcast::<OneErrInner>() {
                Ok(r) => return (*r).into(),
                Err(r) => format!("{}", r),
            }
        } else {
            format!("{}", e)
        };

        // otherwise, build one
        let mut inner = OneErrInner::new(err_kind_to_str(kind).into());
        if let Some(os) = os {
            inner.set_field(OS.into(), os);
        }
        inner.set_field(MESSAGE.into(), message);
        Self(std::io::Error::new(kind, inner))
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

impl std::ops::DerefMut for OneErr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<std::io::Error> for OneErr {
    fn as_ref(&self) -> &std::io::Error {
        &self.0
    }
}

impl AsMut<std::io::Error> for OneErr {
    fn as_mut(&mut self) -> &mut std::io::Error {
        &mut self.0
    }
}

impl serde::Serialize for OneErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.priv_as_inner().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for OneErr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner: OneErrInner = serde::Deserialize::deserialize(deserializer)?;
        Ok(inner.into())
    }
}

impl OneErr {
    /// Create a new OneErr error instance.
    pub fn new<K, M>(kind: &K, message: &M) -> Self
    where
        K: ?Sized + std::fmt::Display,
        M: ?Sized + std::fmt::Display,
    {
        let kind = kind.to_string().into_boxed_str();
        let io_kind = str_kind_to_err(&kind);
        let mut inner = OneErrInner::new(kind);
        inner.set_field(MESSAGE.into(), message.to_string());
        Self(std::io::Error::new(io_kind, inner))
    }

    /// Get the std::io::ErrorKind associated with this instance.
    pub fn kind(&self) -> std::io::ErrorKind {
        self.0.kind()
    }

    /// Get the &str kind associated with this instance.
    /// This can be more descriptive where `kind()` might return 'Other'.
    pub fn str_kind(&self) -> &str {
        &self.priv_as_inner().kind
    }

    /// Get a reference to the inner std::io::Error of this instance.
    pub fn as_io(&self) -> &std::io::Error {
        self.as_ref()
    }

    /// Set an additional data field on this OneErr.
    /// Will panic on reserved names: "error", "os", "source",
    /// "backtrace", "message".
    pub fn set_field<K, T>(&mut self, name: &K, t: T)
    where
        K: ?Sized + std::fmt::Display,
        T: Into<serde_json::Value>,
    {
        let name = name.to_string().into_boxed_str();
        match &*name {
            ERROR | OS | SOURCE | BACKTRACE | MESSAGE => {
                panic!("field name '{}' is reserved", name)
            }
            _ => (),
        }

        self.priv_as_inner_mut().set_field(name, t);
    }

    /// Get the raw json value of a raw additional field associated with
    /// this error, or None if no such field exists.
    pub fn get_field<R>(&self, name: R) -> Option<&serde_json::Value>
    where
        R: AsRef<str>,
    {
        self.priv_as_inner().get_field(name.as_ref())
    }
}

// -- private -- //

impl From<OneErrInner> for OneErr {
    fn from(e: OneErrInner) -> Self {
        Self(e.into())
    }
}

impl OneErr {
    fn priv_as_inner(&self) -> &OneErrInner {
        // we can do all these unwraps because we control
        // our inner type to always be valid and of OneErrInner type.
        self.0
            .get_ref()
            .unwrap()
            .downcast_ref::<OneErrInner>()
            .unwrap()
    }

    fn priv_as_inner_mut(&mut self) -> &mut OneErrInner {
        // we can do all these unwraps because we control
        // our inner type to always be valid and of OneErrInner type.
        self.0
            .get_mut()
            .unwrap()
            .downcast_mut::<OneErrInner>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_bounds() {
        fn assert_bounds<T: 'static + Send + Sync + Unpin>(_t: T) {}
        let one_err: OneErr = std::io::ErrorKind::Other.into();
        assert_bounds(one_err);
    }

    #[test]
    fn display() {
        let e: OneErr = std::io::ErrorKind::NotFound.into();
        println!("{}", e);

        let r = std::io::Error::new(
            std::io::ErrorKind::NotFound.into(),
            "test zombies",
        );
        let e: OneErr = r.into();
        println!("{}", e);

        let e = OneErr::new("TestKind", "my special test error kind");
        println!("{} {:?}", e, e.as_io());

        let r = std::io::Error::from_raw_os_error(98);
        let e: OneErr = r.into();

        println!("GOT BEFORE: {}", e);
        let enc = serde_json::to_string_pretty(&e).unwrap();
        println!("GOT ENCODED: {}", enc);
        let dec: OneErr = serde_json::from_str(&enc).unwrap();
        println!("GOT DECODED: {}", dec);
    }
}
