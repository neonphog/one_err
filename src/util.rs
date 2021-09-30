#[cfg(feature = "preserve_order")]
pub(crate) type TopMap = indexmap::IndexMap<Box<str>, crate::Value>;

#[cfg(not(feature = "preserve_order"))]
pub(crate) type TopMap = std::collections::HashMap<Box<str>, crate::Value>;

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

pub(crate) fn fmt_display<'a, T: ?Sized + serde::Serialize>(
    f: &mut std::fmt::Formatter<'a>,
    t: &T,
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

pub(crate) fn fmt_debug_alt<'a, T: ?Sized + serde::Serialize>(
    f: &mut std::fmt::Formatter<'a>,
    t: &T,
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

use crate::io_error::*;

pub(crate) fn err_kind_to_str(kind: std::io::ErrorKind) -> &'static str {
    use std::io::ErrorKind::*;
    match kind {
        NotFound => NOT_FOUND_STR,
        PermissionDenied => PERMISSION_DENIED_STR,
        ConnectionRefused => CONNECTION_REFUSED_STR,
        ConnectionReset => CONNECTION_RESET_STR,
        ConnectionAborted => CONNECTION_ABORTED_STR,
        NotConnected => NOT_CONNECTED_STR,
        AddrInUse => ADDR_IN_USE_STR,
        AddrNotAvailable => ADDR_NOT_AVAILABLE_STR,
        BrokenPipe => BROKEN_PIPE_STR,
        AlreadyExists => ALREADY_EXISTS_STR,
        WouldBlock => WOULD_BLOCK_STR,
        InvalidInput => INVALID_INPUT_STR,
        InvalidData => INVALID_DATA_STR,
        TimedOut => TIMED_OUT_STR,
        WriteZero => WRITE_ZERO_STR,
        Interrupted => INTERRUPTED_STR,
        UnexpectedEof => UNEXPECTED_EOF_STR,
        //#[cfg(version("1.53"))]
        //Unsupported => UNSUPPORTED_STR,
        //#[cfg(version("1.54"))]
        //OutOfMemory => OUT_OF_MEMORY_STR,
        _ => OTHER_STR,
    }
}

pub(crate) fn parse_err_str(
    s: &str,
) -> (std::io::ErrorKind, Option<crate::ErrNo>) {
    use std::io::ErrorKind::*;
    let kind = match s {
        NOT_FOUND_STR => NotFound,
        PERMISSION_DENIED_STR => PermissionDenied,
        CONNECTION_REFUSED_STR => ConnectionRefused,
        CONNECTION_RESET_STR => ConnectionReset,
        CONNECTION_ABORTED_STR => ConnectionAborted,
        NOT_CONNECTED_STR => NotConnected,
        ADDR_IN_USE_STR => AddrInUse,
        ADDR_NOT_AVAILABLE_STR => AddrNotAvailable,
        BROKEN_PIPE_STR => BrokenPipe,
        ALREADY_EXISTS_STR => AlreadyExists,
        WOULD_BLOCK_STR => WouldBlock,
        INVALID_INPUT_STR => InvalidInput,
        INVALID_DATA_STR => InvalidData,
        TIMED_OUT_STR => TimedOut,
        WRITE_ZERO_STR => WriteZero,
        INTERRUPTED_STR => Interrupted,
        UNEXPECTED_EOF_STR => UnexpectedEof,
        //#[cfg(version("1.53"))]
        //UNSUPPORTED_STR => Unsupported,
        //#[cfg(version("1.54"))]
        //OUT_OF_MEMORY_STR => OutOfMemory,
        _ => Other,
    };

    let os = if let Other = kind {
        let os = crate::ErrNo::from(s);
        if let crate::ErrNo::Other = os {
            None
        } else {
            Some(os)
        }
    } else {
        None
    };

    (kind, os)
}
