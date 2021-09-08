#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::collapsible_else_if)]
//! OneErr to rule them all.
//!
//! There are some great error helper crates out there.
//! My favorites are [thiserror](https://crates.io/crates/thiserror) and
//! [anyhow](https://crates.io/crates/anyhow).
//!
//! But sometimes you just need something different. The `thiserror` crate can
//! over time lead to giant trees of nested error enums, while `anyhow` is
//! difficult to match on.
//!
//! Sometimes you need to interoperate with std::io::Error, but that type is
//! awkward to construct, not `Clone`, and cannot be serialized.
//!
//! `OneErr` is a newtype over std::io::Error, but makes it clonable,
//! serializable, and hopefully more ergonomic.
//!
//! ### std::io::ErrorKind Matching
//!
//! ```rust
//! use one_err::*;
//!
//! for res in [
//!     Ok("not-error"),
//!     Err(OneErr::from(std::io::ErrorKind::InvalidInput)),
//!     Err(OneErr::from(std::io::ErrorKind::ConnectionRefused)),
//! ] {
//!     match res.map_err(|e| e.kind()) {
//!         Ok(ok) => assert_eq!("not-error", ok),
//!         Err(std::io::ErrorKind::InvalidInput) => (),
//!         Err(std::io::ErrorKind::ConnectionRefused) => (),
//!         oth => panic!("unexpected: {:?}", oth),
//!     }
//! }
//! ```
//!
//! ### ErrNo Matching
//!
//! ```rust
//! use one_err::*;
//!
//! for res in [
//!     Ok("not-error"),
//!     Err(OneErr::from(ErrNo::NoData)),
//!     Err(OneErr::from(ErrNo::Proto)),
//! ] {
//!     match res.map_err(|e| e.errno()) {
//!         Ok(ok) => assert_eq!("not-error", ok),
//!         Err(ErrNo::NoData) => (),
//!         Err(ErrNo::Proto) => (),
//!         oth => panic!("unexpected: {:?}", oth),
//!     }
//! }
//! ```
//!
//! ### Custom Matching
//!
//! ```rust
//! use one_err::*;
//!
//! const ERR_FOO: &str = "FOO";
//! const ERR_BAR: &str = "BAR";
//!
//! for res in [
//!     Ok("not-error"),
//!     Err(OneErr::with_message(ERR_FOO, "foo test")),
//!     Err(OneErr::with_message(ERR_BAR, "bar test")),
//! ] {
//!     match res.as_ref().map_err(|e| (e.str_kind(), e)) {
//!         Ok(ok) => assert_eq!("not-error", *ok),
//!         Err((ERR_FOO, e)) => assert_eq!("foo test", e.get_message().unwrap()),
//!         Err((ERR_BAR, e)) => assert_eq!("bar test", e.get_message().unwrap()),
//!         oth => panic!("unexpected: {:?}", oth),
//!     }
//! }
//! ```
//!
//! ### std::io Interoperability
//!
//! ```rust
//! use one_err::*;
//! use std::io::Read;
//!
//! const CUSTOM_ERR: &str = "CustomError";
//!
//! /// My custom Read that always errors.
//! pub struct ErrReader;
//!
//! impl Read for ErrReader {
//!     fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
//!         Err(OneErr::new(CUSTOM_ERR).into())
//!     }
//! }
//!
//! assert_eq!(
//!     r#"{"error":"CustomError"}"#,
//!     &ErrReader.read(&mut []).unwrap_err().to_string(),
//! );
//! ```
//!
//! ### Serialization and Parsing
//!
//! ```rust
//! use one_err::*;
//!
//! const CUSTOM_ERR: &str = "CustomError";
//!
//! let err = OneErr::new(CUSTOM_ERR);
//! let enc = err.to_string();
//!
//! assert_eq!(
//!     r#"{"error":"CustomError"}"#,
//!     &enc,
//! );
//!
//! let dec: OneErr = enc.parse().unwrap();
//! assert_eq!(err, dec);
//! ```

mod errno_;
pub use errno_::*;

pub mod io_error;

mod value;
pub use value::*;

mod inner;
use inner::*;

mod util;
use util::*;

mod one_err;
pub use crate::one_err::*;

#[cfg(test)]
mod test;
