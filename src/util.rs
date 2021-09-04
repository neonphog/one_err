#[cfg(feature = "preserve_order")]
pub(crate) type TopMap = indexmap::IndexMap<Box<str>, serde_json::Value>;

#[cfg(not(feature = "preserve_order"))]
pub(crate) type TopMap =
    std::collections::HashMap<Box<str>, serde_json::Value>;

#[cfg(feature = "std")]
pub(crate) struct IoToFmt<'ltr, 'ltf>(pub &'ltr mut std::fmt::Formatter<'ltf>);

#[cfg(feature = "std")]
impl<'ltr, 'ltf> std::io::Write for IoToFmt<'ltr, 'ltf> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(match std::str::from_utf8(buf) {
            Ok(buf) => {
                self.0.write_str(buf).map_err(|_| {
                    std::io::Error::from(std::io::ErrorKind::Other)
                })?;
                buf.as_bytes().len()
            }
            Err(_) => {
                let tmp = String::from_utf8_lossy(buf);
                self.0.write_str(&tmp).map_err(|_| {
                    std::io::Error::from(std::io::ErrorKind::Other)
                })?;
                tmp.as_bytes().len()
            }
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(crate) fn fmt_display<'a, 'b, 'c, T: ?Sized + serde::Serialize>(
    f: &'a mut std::fmt::Formatter<'b>,
    t: &'c T,
) -> std::fmt::Result {
    #[cfg(feature = "std")]
    {
        serde_json::to_writer(IoToFmt(f), t).map_err(|_| std::fmt::Error)
    }
    #[cfg(not(feature = "std"))]
    {
        let s = serde_json::to_string(t).map_err(|_| std::fmt::Error)?;
        f.write_str(&s).map_err(|_| std::fmt::Error)
    }
}

pub(crate) fn fmt_debug_alt<'a, 'b, 'c, T: ?Sized + serde::Serialize>(
    f: &'a mut std::fmt::Formatter<'b>,
    t: &'c T,
) -> std::fmt::Result {
    #[cfg(feature = "std")]
    {
        serde_json::to_writer_pretty(IoToFmt(f), t).map_err(|_| std::fmt::Error)
    }
    #[cfg(not(feature = "std"))]
    {
        let s = serde_json::to_string_pretty(t).map_err(|_| std::fmt::Error)?;
        f.write_str(&s).map_err(|_| std::fmt::Error)
    }
}

const NOT_FOUND: &str = "NotFound";
const PERMISSION_DENIED: &str = "PermissionDenied";
const CONNECTION_REFUSED: &str = "ConnectionRefused";
const CONNECTION_RESET: &str = "ConnectionReset";
const CONNECTION_ABORTED: &str = "ConnectionAborted";
const NOT_CONNECTED: &str = "NotConnected";
const ADDR_IN_USE: &str = "AddrInUse";
const ADDR_NOT_AVAILABLE: &str = "AddrNotAvailable";
const BROKEN_PIPE: &str = "BrokenPipe";
const ALREADY_EXISTS: &str = "AlreadyExists";
const WOULD_BLOCK: &str = "WouldBlock";
const INVALID_INPUT: &str = "InvalidInput";
const INVALID_DATA: &str = "InvalidData";
const TIMED_OUT: &str = "TimedOut";
const WRITE_ZERO: &str = "WriteZero";
const INTERRUPTED: &str = "Interrupted";
const UNEXPECTED_EOF: &str = "UnexpectedEof";
const UNSUPPORTED: &str = "Unsupported";
const OUT_OF_MEMORY: &str = "OutOfMemory";
const OTHER: &str = "Other";

pub(crate) fn err_kind_to_str(kind: std::io::ErrorKind) -> &'static str {
    use std::io::ErrorKind::*;
    match kind {
        NotFound => NOT_FOUND,
        PermissionDenied => PERMISSION_DENIED,
        ConnectionRefused => CONNECTION_REFUSED,
        ConnectionReset => CONNECTION_RESET,
        ConnectionAborted => CONNECTION_ABORTED,
        NotConnected => NOT_CONNECTED,
        AddrInUse => ADDR_IN_USE,
        AddrNotAvailable => ADDR_NOT_AVAILABLE,
        BrokenPipe => BROKEN_PIPE,
        AlreadyExists => ALREADY_EXISTS,
        WouldBlock => WOULD_BLOCK,
        InvalidInput => INVALID_INPUT,
        InvalidData => INVALID_DATA,
        TimedOut => TIMED_OUT,
        WriteZero => WRITE_ZERO,
        Interrupted => INTERRUPTED,
        UnexpectedEof => UNEXPECTED_EOF,
        Unsupported => UNSUPPORTED,
        OutOfMemory => OUT_OF_MEMORY,
        Other | _ => OTHER,
    }
}

pub(crate) fn str_kind_to_err(kind: &str) -> std::io::ErrorKind {
    use std::io::ErrorKind::*;
    match kind {
        NOT_FOUND => NotFound,
        PERMISSION_DENIED => PermissionDenied,
        CONNECTION_REFUSED => ConnectionRefused,
        CONNECTION_RESET => ConnectionReset,
        CONNECTION_ABORTED => ConnectionAborted,
        NOT_CONNECTED => NotConnected,
        ADDR_IN_USE => AddrInUse,
        ADDR_NOT_AVAILABLE => AddrNotAvailable,
        BROKEN_PIPE => BrokenPipe,
        ALREADY_EXISTS => AlreadyExists,
        WOULD_BLOCK => WouldBlock,
        INVALID_INPUT => InvalidInput,
        INVALID_DATA => InvalidData,
        TIMED_OUT => TimedOut,
        WRITE_ZERO => WriteZero,
        INTERRUPTED => Interrupted,
        UNEXPECTED_EOF => UnexpectedEof,
        UNSUPPORTED => Unsupported,
        OUT_OF_MEMORY => OutOfMemory,
        OTHER | _ => Other,
    }
}
