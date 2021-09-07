//! Rust translation of errno.h

// we could write fancy macros to do most of this...
// but it compiles faster when just written out.


// there are a couple inconsistent things accross os-es
const EPERM_1: i32 = 1;
const EPERM_2: i32 = 13;
const EWOULDBLOCK_1: i32 = 11;
const EWOULDBLOCK_2: i32 = 41;
const EDEADLOCK_1: i32 = 35;
const EDEADLOCK_2: i32 = 58;

// incase we get an errno not in our list
const EOTHER: i32 = -1;

/// Rust Errno Enum
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrNo {
    /// Permission denied (os errno 1 or 13)
    Perm = EPERM_1,

    /// No such file or directory (os errno 2)
    NoEnt = libc::ENOENT,

    /// No such process (os errno 3)
    Srch = libc::ESRCH,

    /// Interrupted system call (os errno 4)
    Intr = libc::EINTR,

    /// I/O error (os errno 5)
    Io = libc::EIO,

    /// No such device or address (os errno 6)
    NxIo = libc::ENXIO,

    /// Arg list too long (os errno 7)
    TooBig = libc::E2BIG,

    /// Exec format error (os errno 8)
    NoExec = libc::ENOEXEC,

    /// Bad file number (os errno 9)
    BadF = libc::EBADF,

    /// No child processes (os errno 10)
    Child = libc::ECHILD,

    /// Out of memory (os errno 12)
    NoMem = libc::ENOMEM,

    /// Bad address (os errno 14)
    Fault = libc::EFAULT,

    /// Block device required (os errno 15)
    NotBlk = libc::ENOTBLK,

    /// Device or resource busy (os errno 16)
    Busy = libc::EBUSY,

    /// File exists (os errno 17)
    Exist = libc::EEXIST,

    /// Cross-device link (os errno 18)
    XDev = libc::EXDEV,

    /// No such device (os errno 19)
    NoDev = libc::ENODEV,

    /// Not a directory (os errno 20)
    NotDir = libc::ENOTDIR,

    /// Is a directory (os errno 21)
    IsDir = libc::EISDIR,

    /// Invalid argument (os errno 22)
    Inval = libc::EINVAL,

    /// File table overflow (os errno 23)
    NFile = libc::ENFILE,

    /// Too many open files (os errno 24)
    MFile = libc::EMFILE,

    /// Not a typewriter (os errno 25)
    NotTy = libc::ENOTTY,

    /// Text file busy (os errno 26)
    TxtBsy = libc::ETXTBSY,

    /// File too large (os errno 27)
    FBig = libc::EFBIG,

    /// No space left on device (os errno 28)
    NoSpc = libc::ENOSPC,

    /// Illegal seek (os errno 29)
    SPipe = libc::ESPIPE,

    /// Read-only file system (os errno 30)
    ROFS = libc::EROFS,

    /// Too many links (os errno 31)
    MLink = libc::EMLINK,

    /// Broken pipe (os errno 32)
    Pipe = libc::EPIPE,

    /// Math argument out of domain of func (os errno 33)
    Dom = libc::EDOM,

    /// Math result not representable (os errno 34)
    Range = libc::ERANGE,

    /// Resource deadlock would occur (os errno 35 or 58)
    DeadLk = EDEADLOCK_1,

    /// File name too long (os errno 36)
    NameTooLong = libc::ENAMETOOLONG,

    /// No record locks available (os errno 37)
    NoLck = libc::ENOLCK,

    /// Function not implemented (os errno 38)
    NoSys = libc::ENOSYS,

    /// Directory not empty (os errno 39)
    NotEmpty = libc::ENOTEMPTY,

    /// Too many symbolic links encountered (os errno 40)
    Loop = libc::ELOOP,

    /// Operation would block (os errno 11 or 41)
    WouldBlock = EWOULDBLOCK_1,

    /// No message of desired type (os errno 42)
    NoMsg = libc::ENOMSG,

    /// Identifier removed (os errno 43)
    IdRm = libc::EIDRM,

    /// Channel number out of range (os errno 44)
    ChRng = libc::ECHRNG,

    /// Level 2 not synchronized (os errno 45)
    L2NSync = libc::EL2NSYNC,

    /// Level 3 halted (os errno 46)
    L3Hlt = libc::EL3HLT,

    /// Level 3 reset (os errno 47)
    L3Rst = libc::EL3RST,

    /// Link number out of range (os errno 48)
    LNRng = libc::ELNRNG,

    /// Protocol driver not attached (os errno 49)
    Unatch = libc::EUNATCH,

    /// No CSI structure available (os errno 50)
    NoCSI = libc::ENOCSI,

    /// Level 2 halted (os errno 51)
    L2Hlt = libc::EL2HLT,

    /// Invalid exchange (os errno 52)
    BadE = libc::EBADE,

    /// Invalid request descriptor (os errno 53)
    BadR = libc::EBADR,

    /// Exchange full (os errno 54)
    XFull = libc::EXFULL,

    /// No anode (os errno 55)
    NoAno = libc::ENOANO,

    /// Invalid request code (os errno 56)
    BadRqC = libc::EBADRQC,

    /// Invalid slot (os errno 57)
    BadSlt = libc::EBADSLT,

    /// Bad font file format (os errno 59)
    BFont = libc::EBFONT,

    /// Device not a stream (os errno 60)
    NoStr = libc::ENOSTR,

    /// No data available (os errno 61)
    NoData = libc::ENODATA,

    /// Timer expired (os errno 62)
    Time = libc::ETIME,

    /// Out of streams resources (os errno 63)
    NoSR = libc::ENOSR,

    /// Machine is not on the network (os errno 64)
    NoNet = libc::ENONET,

    /// Package not installed (os errno 65)
    NoPkg = libc::ENOPKG,

    /// Object is remote (os errno 66)
    Remote = libc::EREMOTE,

    /// Link has been severed (os errno 67)
    NoLink = libc::ENOLINK,

    /// Advertise error (os errno 68)
    Adv = libc::EADV,

    /// Srmount error (os errno 69)
    SrMnt = libc::ESRMNT,

    /// Communication error on send (os errno 70)
    Comm = libc::ECOMM,

    /// Protocol error (os errno 71)
    Proto = libc::EPROTO,

    /// Multihop attempted (os errno 72)
    MultiHop = libc::EMULTIHOP,

    /// RFS specific error (os errno 73)
    DotDot = libc::EDOTDOT,

    /// Not a data message (os errno 74)
    BadMsg = libc::EBADMSG,

    /// Value too large for defined data type (os errno 75)
    Overflow = libc::EOVERFLOW,

    /// Name not unique on network (os errno 76)
    NotUniq = libc::ENOTUNIQ,

    /// File descriptor in bad state (os errno 77)
    BadFD = libc::EBADFD,

    /// Remote address changed (os errno 78)
    RemChg = libc::EREMCHG,

    /// Can not access a needed shared library (os errno 79)
    LibAcc = libc::ELIBACC,

    /// Accessing a corrupted shared library (os errno 80)
    LibBad = libc::ELIBBAD,

    /// .lib section in a.out corrupted (os errno 81)
    LibScn = libc::ELIBSCN,

    /// Attempting to link in too many shared libraries (os errno 82)
    LibMax = libc::ELIBMAX,

    /// Cannot exec a shared library directly (os errno 83)
    LibExec = libc::ELIBEXEC,

    /// Illegal byte sequence (os errno 84)
    IlSeq = libc::EILSEQ,

    /// Interrupted system call should be restarted (os errno 85)
    Restart = libc::ERESTART,

    /// Streams pipe error (os errno 86)
    StrPipe = libc::ESTRPIPE,

    /// Too many users (os errno 87)
    Users = libc::EUSERS,

    /// Socket operation on non-socket (os errno 88)
    NotSock = libc::ENOTSOCK,

    /// Destination address required (os errno 89)
    DestAddrReq = libc::EDESTADDRREQ,

    /// Message too long (os errno 90)
    MsgSize = libc::EMSGSIZE,

    /// Protocol wrong type for socket (os errno 91)
    ProtoType = libc::EPROTOTYPE,

    /// Protocol not available (os errno 92)
    NoProtoOpt = libc::ENOPROTOOPT,

    /// Protocol not supported (os errno 93)
    ProtoNoSupport = libc::EPROTONOSUPPORT,

    /// Socket type not supported (os errno 94)
    SockTNoSupport = libc::ESOCKTNOSUPPORT,

    /// Operation not supported on transport endpoint (os errno 95)
    OpNotSupp = libc::EOPNOTSUPP,

    /// Protocol family not supported (os errno 96)
    PFNoSupport = libc::EPFNOSUPPORT,

    /// Address family not supported by protocol (os errno 97)
    AFNoSupport = libc::EAFNOSUPPORT,

    /// Address already in use (os errno 98)
    AddrInUse = libc::EADDRINUSE,

    /// Cannot assign requested address (os errno 99)
    AddrNotAvail = libc::EADDRNOTAVAIL,

    /// Network is down (os errno 100)
    NetDown = libc::ENETDOWN,

    /// Network is unreachable (os errno 101)
    NetUnreach = libc::ENETUNREACH,

    /// Network dropped connection because of reset (os errno 102)
    NetReset = libc::ENETRESET,

    /// Software caused connection abort (os errno 103)
    ConnAborted = libc::ECONNABORTED,

    /// Connection reset by peer (os errno 104)
    ConnReset = libc::ECONNRESET,

    /// No buffer space available (os errno 105)
    NoBufS = libc::ENOBUFS,

    /// Transport endpoint is already connected (os errno 106)
    IsConn = libc::EISCONN,

    /// Transport endpoint is not connected (os errno 107)
    NotConn = libc::ENOTCONN,

    /// Cannot send after transport endpoint shutdown (os errno 108)
    Shutdown = libc::ESHUTDOWN,

    /// Too many references: cannot splice (os errno 109)
    TooManyRefs = libc::ETOOMANYREFS,

    /// Connection timed out (os errno 110)
    TimedOut = libc::ETIMEDOUT,

    /// Connection refused (os errno 111)
    ConnRefused = libc::ECONNREFUSED,

    /// Host is down (os errno 112)
    HostDown = libc::EHOSTDOWN,

    /// No route to host (os errno 113)
    HostUnreach = libc::EHOSTUNREACH,

    /// Operation already in progress (os errno 114)
    Already = libc::EALREADY,

    /// Operation now in progress (os errno 115)
    InProgress = libc::EINPROGRESS,

    /// Stale NFS file handle (os errno 116)
    Stale = libc::ESTALE,

    /// Structure needs cleaning (os errno 117)
    UClean = libc::EUCLEAN,

    /// Not a Xlibc::ENIX named type file (os errno 118)
    NotNam = libc::ENOTNAM,

    /// No Xlibc::ENIX semaphores available (os errno 119)
    NAvail = libc::ENAVAIL,

    /// Is a named type file (os errno 120)
    IsNam = libc::EISNAM,

    /// Remote I/O error (os errno 121)
    RemoteIO = libc::EREMOTEIO,

    /// Other / Unrecognized Error
    Other = EOTHER,
}

impl std::fmt::Display for ErrNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&'static str>::from(self))
    }
}

impl ErrNo {
    /// Get descriptive text for this errno variant.
    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::Perm => "Permission denied (os errno 1 or 13)",
            Self::NoEnt => "No such file or directory (os errno 2)",
            Self::Srch => "No such process (os errno 3)",
            Self::Intr => "Interrupted system call (os errno 4)",
            Self::Io => "I/O error (os errno 5)",
            Self::NxIo => "No such device or address (os errno 6)",
            Self::TooBig => "Arg list too long (os errno 7)",
            Self::NoExec => "Exec format error (os errno 8)",
            Self::BadF => "Bad file number (os errno 9)",
            Self::Child => "No child processes (os errno 10)",
            Self::NoMem => "Out of memory (os errno 12)",
            Self::Fault => "Bad address (os errno 14)",
            Self::NotBlk => "Block device required (os errno 15)",
            Self::Busy => "Device or resource busy (os errno 16)",
            Self::Exist => "File exists (os errno 17)",
            Self::XDev => "Cross-device link (os errno 18)",
            Self::NoDev => "No such device (os errno 19)",
            Self::NotDir => "Not a directory (os errno 20)",
            Self::IsDir => "Is a directory (os errno 21)",
            Self::Inval => "Invalid argument (os errno 22)",
            Self::NFile => "File table overflow (os errno 23)",
            Self::MFile => "Too many open files (os errno 24)",
            Self::NotTy => "Not a typewriter (os errno 25)",
            Self::TxtBsy => "Text file busy (os errno 26)",
            Self::FBig => "File too large (os errno 27)",
            Self::NoSpc => "No space left on device (os errno 28)",
            Self::SPipe => "Illegal seek (os errno 29)",
            Self::ROFS => "Read-only file system (os errno 30)",
            Self::MLink => "Too many links (os errno 31)",
            Self::Pipe => "Broken pipe (os errno 32)",
            Self::Dom => "Math argument out of domain of func (os errno 33)",
            Self::Range => "Math result not representable (os errno 34)",
            Self::DeadLk => "Resource deadlock would occur (os errno 35 or 58)",
            Self::NameTooLong => "File name too long (os errno 36)",
            Self::NoLck => "No record locks available (os errno 37)",
            Self::NoSys => "Function not implemented (os errno 38)",
            Self::NotEmpty => "Directory not empty (os errno 39)",
            Self::Loop => "Too many symbolic links encountered (os errno 40)",
            Self::WouldBlock => "Operation would block (os errno 11 or 41)",
            Self::NoMsg => "No message of desired type (os errno 42)",
            Self::IdRm => "Identifier removed (os errno 43)",
            Self::ChRng => "Channel number out of range (os errno 44)",
            Self::L2NSync => "Level 2 not synchronized (os errno 45)",
            Self::L3Hlt => "Level 3 halted (os errno 46)",
            Self::L3Rst => "Level 3 reset (os errno 47)",
            Self::LNRng => "Link number out of range (os errno 48)",
            Self::Unatch => "Protocol driver not attached (os errno 49)",
            Self::NoCSI => "No CSI structure available (os errno 50)",
            Self::L2Hlt => "Level 2 halted (os errno 51)",
            Self::BadE => "Invalid exchange (os errno 52)",
            Self::BadR => "Invalid request descriptor (os errno 53)",
            Self::XFull => "Exchange full (os errno 54)",
            Self::NoAno => "No anode (os errno 55)",
            Self::BadRqC => "Invalid request code (os errno 56)",
            Self::BadSlt => "Invalid slot (os errno 57)",
            Self::BFont => "Bad font file format (os errno 59)",
            Self::NoStr => "Device not a stream (os errno 60)",
            Self::NoData => "No data available (os errno 61)",
            Self::Time => "Timer expired (os errno 62)",
            Self::NoSR => "Out of streams resources (os errno 63)",
            Self::NoNet => "Machine is not on the network (os errno 64)",
            Self::NoPkg => "Package not installed (os errno 65)",
            Self::Remote => "Object is remote (os errno 66)",
            Self::NoLink => "Link has been severed (os errno 67)",
            Self::Adv => "Advertise error (os errno 68)",
            Self::SrMnt => "Srmount error (os errno 69)",
            Self::Comm => "Communication error on send (os errno 70)",
            Self::Proto => "Protocol error (os errno 71)",
            Self::MultiHop => "Multihop attempted (os errno 72)",
            Self::DotDot => "RFS specific error (os errno 73)",
            Self::BadMsg => "Not a data message (os errno 74)",
            Self::Overflow => "Value too large for defined data type (os errno 75)",
            Self::NotUniq => "Name not unique on network (os errno 76)",
            Self::BadFD => "File descriptor in bad state (os errno 77)",
            Self::RemChg => "Remote address changed (os errno 78)",
            Self::LibAcc => "Can not access a needed shared library (os errno 79)",
            Self::LibBad => "Accessing a corrupted shared library (os errno 80)",
            Self::LibScn => ".lib section in a.out corrupted (os errno 81)",
            Self::LibMax => "Attempting to link in too many shared libraries (os errno 82)",
            Self::LibExec => "Cannot exec a shared library directly (os errno 83)",
            Self::IlSeq => "Illegal byte sequence (os errno 84)",
            Self::Restart => "Interrupted system call should be restarted (os errno 85)",
            Self::StrPipe => "Streams pipe error (os errno 86)",
            Self::Users => "Too many users (os errno 87)",
            Self::NotSock => "Socket operation on non-socket (os errno 88)",
            Self::DestAddrReq => "Destination address required (os errno 89)",
            Self::MsgSize => "Message too long (os errno 90)",
            Self::ProtoType => "Protocol wrong type for socket (os errno 91)",
            Self::NoProtoOpt => "Protocol not available (os errno 92)",
            Self::ProtoNoSupport => "Protocol not supported (os errno 93)",
            Self::SockTNoSupport => "Socket type not supported (os errno 94)",
            Self::OpNotSupp => "Operation not supported on transport endpoint (os errno 95)",
            Self::PFNoSupport => "Protocol family not supported (os errno 96)",
            Self::AFNoSupport => "Address family not supported by protocol (os errno 97)",
            Self::AddrInUse => "Address already in use (os errno 98)",
            Self::AddrNotAvail => "Cannot assign requested address (os errno 99)",
            Self::NetDown => "Network is down (os errno 100)",
            Self::NetUnreach => "Network is unreachable (os errno 101)",
            Self::NetReset => "Network dropped connection because of reset (os errno 102)",
            Self::ConnAborted => "Software caused connection abort (os errno 103)",
            Self::ConnReset => "Connection reset by peer (os errno 104)",
            Self::NoBufS => "No buffer space available (os errno 105)",
            Self::IsConn => "Transport endpoint is already connected (os errno 106)",
            Self::NotConn => "Transport endpoint is not connected (os errno 107)",
            Self::Shutdown => "Cannot send after transport endpoint shutdown (os errno 108)",
            Self::TooManyRefs => "Too many references: cannot splice (os errno 109)",
            Self::TimedOut => "Connection timed out (os errno 110)",
            Self::ConnRefused => "Connection refused (os errno 111)",
            Self::HostDown => "Host is down (os errno 112)",
            Self::HostUnreach => "No route to host (os errno 113)",
            Self::Already => "Operation already in progress (os errno 114)",
            Self::InProgress => "Operation now in progress (os errno 115)",
            Self::Stale => "Stale NFS file handle (os errno 116)",
            Self::UClean => "Structure needs cleaning (os errno 117)",
            Self::NotNam => "Not a Xlibc::ENIX named type file (os errno 118)",
            Self::NAvail => "No Xlibc::ENIX semaphores available (os errno 119)",
            Self::IsNam => "Is a named type file (os errno 120)",
            Self::RemoteIO => "Remote I/O error (os errno 121)",
            Self::Other => "Other / Unrecognized Error",
        }
    }
}

impl From<ErrNo> for i32 {
    fn from(e: ErrNo) -> Self {
        e as i32
    }
}

impl From<&ErrNo> for i32 {
    fn from(e: &ErrNo) -> Self {
        *e as i32
    }
}

impl From<i32> for ErrNo {
    fn from(e: i32) -> Self {
        match e {
            EPERM_1 => Self::Perm,
            libc::ENOENT => Self::NoEnt,
            libc::ESRCH => Self::Srch,
            libc::EINTR => Self::Intr,
            libc::EIO => Self::Io,
            libc::ENXIO => Self::NxIo,
            libc::E2BIG => Self::TooBig,
            libc::ENOEXEC => Self::NoExec,
            libc::EBADF => Self::BadF,
            libc::ECHILD => Self::Child,
            EWOULDBLOCK_1 => Self::WouldBlock,
            libc::ENOMEM => Self::NoMem,
            EPERM_2 => Self::Perm,
            libc::EFAULT => Self::Fault,
            libc::ENOTBLK => Self::NotBlk,
            libc::EBUSY => Self::Busy,
            libc::EEXIST => Self::Exist,
            libc::EXDEV => Self::XDev,
            libc::ENODEV => Self::NoDev,
            libc::ENOTDIR => Self::NotDir,
            libc::EISDIR => Self::IsDir,
            libc::EINVAL => Self::Inval,
            libc::ENFILE => Self::NFile,
            libc::EMFILE => Self::MFile,
            libc::ENOTTY => Self::NotTy,
            libc::ETXTBSY => Self::TxtBsy,
            libc::EFBIG => Self::FBig,
            libc::ENOSPC => Self::NoSpc,
            libc::ESPIPE => Self::SPipe,
            libc::EROFS => Self::ROFS,
            libc::EMLINK => Self::MLink,
            libc::EPIPE => Self::Pipe,
            libc::EDOM => Self::Dom,
            libc::ERANGE => Self::Range,
            EDEADLOCK_1 => Self::DeadLk,
            libc::ENAMETOOLONG => Self::NameTooLong,
            libc::ENOLCK => Self::NoLck,
            libc::ENOSYS => Self::NoSys,
            libc::ENOTEMPTY => Self::NotEmpty,
            libc::ELOOP => Self::Loop,
            EWOULDBLOCK_2 => Self::WouldBlock,
            libc::ENOMSG => Self::NoMsg,
            libc::EIDRM => Self::IdRm,
            libc::ECHRNG => Self::ChRng,
            libc::EL2NSYNC => Self::L2NSync,
            libc::EL3HLT => Self::L3Hlt,
            libc::EL3RST => Self::L3Rst,
            libc::ELNRNG => Self::LNRng,
            libc::EUNATCH => Self::Unatch,
            libc::ENOCSI => Self::NoCSI,
            libc::EL2HLT => Self::L2Hlt,
            libc::EBADE => Self::BadE,
            libc::EBADR => Self::BadR,
            libc::EXFULL => Self::XFull,
            libc::ENOANO => Self::NoAno,
            libc::EBADRQC => Self::BadRqC,
            libc::EBADSLT => Self::BadSlt,
            EDEADLOCK_2 => Self::DeadLk,
            libc::EBFONT => Self::BFont,
            libc::ENOSTR => Self::NoStr,
            libc::ENODATA => Self::NoData,
            libc::ETIME => Self::Time,
            libc::ENOSR => Self::NoSR,
            libc::ENONET => Self::NoNet,
            libc::ENOPKG => Self::NoPkg,
            libc::ENOLINK => Self::NoLink,
            libc::EADV => Self::Adv,
            libc::ESRMNT => Self::SrMnt,
            libc::ECOMM => Self::Comm,
            libc::EPROTO => Self::Proto,
            libc::EMULTIHOP => Self::MultiHop,
            libc::EDOTDOT => Self::DotDot,
            libc::EBADMSG => Self::BadMsg,
            libc::EOVERFLOW => Self::Overflow,
            libc::ENOTUNIQ => Self::NotUniq,
            libc::EBADFD => Self::BadFD,
            libc::EREMCHG => Self::RemChg,
            libc::ELIBACC => Self::LibAcc,
            libc::ELIBBAD => Self::LibBad,
            libc::ELIBSCN => Self::LibScn,
            libc::ELIBMAX => Self::LibMax,
            libc::ELIBEXEC => Self::LibExec,
            libc::EILSEQ => Self::IlSeq,
            libc::ERESTART => Self::Restart,
            libc::ESTRPIPE => Self::StrPipe,
            libc::EUSERS => Self::Users,
            libc::ENOTSOCK => Self::NotSock,
            libc::EDESTADDRREQ => Self::DestAddrReq,
            libc::EMSGSIZE => Self::MsgSize,
            libc::EPROTOTYPE => Self::ProtoType,
            libc::ENOPROTOOPT => Self::NoProtoOpt,
            libc::EPROTONOSUPPORT => Self::ProtoNoSupport,
            libc::ESOCKTNOSUPPORT => Self::SockTNoSupport,
            libc::EOPNOTSUPP => Self::OpNotSupp,
            libc::EPFNOSUPPORT => Self::PFNoSupport,
            libc::EAFNOSUPPORT => Self::AFNoSupport,
            libc::EADDRINUSE => Self::AddrInUse,
            libc::EADDRNOTAVAIL => Self::AddrNotAvail,
            libc::ENETDOWN => Self::NetDown,
            libc::ENETUNREACH => Self::NetUnreach,
            libc::ENETRESET => Self::NetReset,
            libc::ECONNABORTED => Self::ConnAborted,
            libc::ECONNRESET => Self::ConnReset,
            libc::ENOBUFS => Self::NoBufS,
            libc::EISCONN => Self::IsConn,
            libc::ENOTCONN => Self::NotConn,
            libc::ESHUTDOWN => Self::Shutdown,
            libc::ETOOMANYREFS => Self::TooManyRefs,
            libc::ETIMEDOUT => Self::TimedOut,
            libc::ECONNREFUSED => Self::ConnRefused,
            libc::EHOSTDOWN => Self::HostDown,
            libc::EHOSTUNREACH => Self::HostUnreach,
            libc::EALREADY => Self::Already,
            libc::EINPROGRESS => Self::InProgress,
            libc::ESTALE => Self::Stale,
            libc::EUCLEAN => Self::UClean,
            libc::ENOTNAM => Self::NotNam,
            libc::ENAVAIL => Self::NAvail,
            libc::EISNAM => Self::IsNam,
            libc::EREMOTEIO => Self::RemoteIO,
            _ => Self::Other,
        }
    }
}

impl From<ErrNo> for std::io::ErrorKind {
    fn from(e: ErrNo) -> Self {
        (&e).into()
    }
}

impl From<&ErrNo> for std::io::ErrorKind {
    fn from(e: &ErrNo) -> Self {
        use std::io::ErrorKind::*;
        match e {
            ErrNo::AddrInUse => AddrInUse,
            ErrNo::AddrNotAvail => AddrNotAvailable,
            ErrNo::ConnAborted => ConnectionAborted,
            ErrNo::ConnRefused => ConnectionRefused,
            ErrNo::ConnReset => ConnectionReset,
            ErrNo::Perm => PermissionDenied,
            ErrNo::Exist => AlreadyExists,
            ErrNo::Intr => Interrupted,
            ErrNo::Inval => InvalidInput,
            ErrNo::NoEnt => NotFound,
            ErrNo::NoMem => OutOfMemory,
            ErrNo::NoSys => Unsupported,
            ErrNo::NotConn => NotConnected,
            ErrNo::Pipe => BrokenPipe,
            ErrNo::WouldBlock => WouldBlock,
            ErrNo::TimedOut => TimedOut,
            _ => Other,
        }
    }
}

const EPERM_STR: &str = "EPERM";
const ENOENT_STR: &str = "ENOENT";
const ESRCH_STR: &str = "ESRCH";
const EINTR_STR: &str = "EINTR";
const EIO_STR: &str = "EIO";
const ENXIO_STR: &str = "ENXIO";
const E2BIG_STR: &str = "E2BIG";
const ENOEXEC_STR: &str = "ENOEXEC";
const EBADF_STR: &str = "EBADF";
const ECHILD_STR: &str = "ECHILD";
const ENOMEM_STR: &str = "ENOMEM";
const EFAULT_STR: &str = "EFAULT";
const ENOTBLK_STR: &str = "ENOTBLK";
const EBUSY_STR: &str = "EBUSY";
const EEXIST_STR: &str = "EEXIST";
const EXDEV_STR: &str = "EXDEV";
const ENODEV_STR: &str = "ENODEV";
const ENOTDIR_STR: &str = "ENOTDIR";
const EISDIR_STR: &str = "EISDIR";
const EINVAL_STR: &str = "EINVAL";
const ENFILE_STR: &str = "ENFILE";
const EMFILE_STR: &str = "EMFILE";
const ENOTTY_STR: &str = "ENOTTY";
const ETXTBSY_STR: &str = "ETXTBSY";
const EFBIG_STR: &str = "EFBIG";
const ENOSPC_STR: &str = "ENOSPC";
const ESPIPE_STR: &str = "ESPIPE";
const EROFS_STR: &str = "EROFS";
const EMLINK_STR: &str = "EMLINK";
const EPIPE_STR: &str = "EPIPE";
const EDOM_STR: &str = "EDOM";
const ERANGE_STR: &str = "ERANGE";
const EDEADLK_STR: &str = "EDEADLK";
const ENAMETOOLONG_STR: &str = "ENAMETOOLONG";
const ENOLCK_STR: &str = "ENOLCK";
const ENOSYS_STR: &str = "ENOSYS";
const ENOTEMPTY_STR: &str = "ENOTEMPTY";
const ELOOP_STR: &str = "ELOOP";
const EWOULDBLOCK_STR: &str = "EWOULDBLOCK";
const ENOMSG_STR: &str = "ENOMSG";
const EIDRM_STR: &str = "EIDRM";
const ECHRNG_STR: &str = "ECHRNG";
const EL2NSYNC_STR: &str = "EL2NSYNC";
const EL3HLT_STR: &str = "EL3HLT";
const EL3RST_STR: &str = "EL3RST";
const ELNRNG_STR: &str = "ELNRNG";
const EUNATCH_STR: &str = "EUNATCH";
const ENOCSI_STR: &str = "ENOCSI";
const EL2HLT_STR: &str = "EL2HLT";
const EBADE_STR: &str = "EBADE";
const EBADR_STR: &str = "EBADR";
const EXFULL_STR: &str = "EXFULL";
const ENOANO_STR: &str = "ENOANO";
const EBADRQC_STR: &str = "EBADRQC";
const EBADSLT_STR: &str = "EBADSLT";
const EBFONT_STR: &str = "EBFONT";
const ENOSTR_STR: &str = "ENOSTR";
const ENODATA_STR: &str = "ENODATA";
const ETIME_STR: &str = "ETIME";
const ENOSR_STR: &str = "ENOSR";
const ENONET_STR: &str = "ENONET";
const ENOPKG_STR: &str = "ENOPKG";
const ENOLINK_STR: &str = "ENOLINK";
const EADV_STR: &str = "EADV";
const ESRMNT_STR: &str = "ESRMNT";
const ECOMM_STR: &str = "ECOMM";
const EPROTO_STR: &str = "EPROTO";
const EMULTIHOP_STR: &str = "EMULTIHOP";
const EDOTDOT_STR: &str = "EDOTDOT";
const EBADMSG_STR: &str = "EBADMSG";
const EOVERFLOW_STR: &str = "EOVERFLOW";
const ENOTUNIQ_STR: &str = "ENOTUNIQ";
const EBADFD_STR: &str = "EBADFD";
const EREMCHG_STR: &str = "EREMCHG";
const ELIBACC_STR: &str = "ELIBACC";
const ELIBBAD_STR: &str = "ELIBBAD";
const ELIBSCN_STR: &str = "ELIBSCN";
const ELIBMAX_STR: &str = "ELIBMAX";
const ELIBEXEC_STR: &str = "ELIBEXEC";
const EILSEQ_STR: &str = "EILSEQ";
const ERESTART_STR: &str = "ERESTART";
const ESTRPIPE_STR: &str = "ESTRPIPE";
const EUSERS_STR: &str = "EUSERS";
const ENOTSOCK_STR: &str = "ENOTSOCK";
const EDESTADDRREQ_STR: &str = "EDESTADDRREQ";
const EMSGSIZE_STR: &str = "EMSGSIZE";
const EPROTOTYPE_STR: &str = "EPROTOTYPE";
const ENOPROTOOPT_STR: &str = "ENOPROTOOPT";
const EPROTONOSUPPORT_STR: &str = "EPROTONOSUPPORT";
const ESOCKTNOSUPPORT_STR: &str = "ESOCKTNOSUPPORT";
const EOPNOTSUPP_STR: &str = "EOPNOTSUPP";
const EPFNOSUPPORT_STR: &str = "EPFNOSUPPORT";
const EAFNOSUPPORT_STR: &str = "EAFNOSUPPORT";
const EADDRINUSE_STR: &str = "EADDRINUSE";
const EADDRNOTAVAIL_STR: &str = "EADDRNOTAVAIL";
const ENETDOWN_STR: &str = "ENETDOWN";
const ENETUNREACH_STR: &str = "ENETUNREACH";
const ENETRESET_STR: &str = "ENETRESET";
const ECONNABORTED_STR: &str = "ECONNABORTED";
const ECONNRESET_STR: &str = "ECONNRESET";
const ENOBUFS_STR: &str = "ENOBUFS";
const EISCONN_STR: &str = "EISCONN";
const ENOTCONN_STR: &str = "ENOTCONN";
const ESHUTDOWN_STR: &str = "ESHUTDOWN";
const ETOOMANYREFS_STR: &str = "ETOOMANYREFS";
const ETIMEDOUT_STR: &str = "ETIMEDOUT";
const ECONNREFUSED_STR: &str = "ECONNREFUSED";
const EHOSTDOWN_STR: &str = "EHOSTDOWN";
const EHOSTUNREACH_STR: &str = "EHOSTUNREACH";
const EALREADY_STR: &str = "EALREADY";
const EINPROGRESS_STR: &str = "EINPROGRESS";
const ESTALE_STR: &str = "ESTALE";
const EUCLEAN_STR: &str = "EUCLEAN";
const ENOTNAM_STR: &str = "ENOTNAM";
const ENAVAIL_STR: &str = "ENAVAIL";
const EISNAM_STR: &str = "EISNAM";
const EREMOTEIO_STR: &str = "EREMOTEIO";
const EOTHER_STR: &str = "EOTHER";

impl From<ErrNo> for &'static str {
    fn from(e: ErrNo) -> &'static str {
        (&e).into()
    }
}

impl From<&ErrNo> for &'static str {
    fn from(e: &ErrNo) -> &'static str {
        match *e {
            ErrNo::Perm => EPERM_STR,
            ErrNo::NoEnt => ENOENT_STR,
            ErrNo::Srch => ESRCH_STR,
            ErrNo::Intr => EINTR_STR,
            ErrNo::Io => EIO_STR,
            ErrNo::NxIo => ENXIO_STR,
            ErrNo::TooBig => E2BIG_STR,
            ErrNo::NoExec => ENOEXEC_STR,
            ErrNo::BadF => EBADF_STR,
            ErrNo::Child => ECHILD_STR,
            ErrNo::NoMem => ENOMEM_STR,
            ErrNo::Fault => EFAULT_STR,
            ErrNo::NotBlk => ENOTBLK_STR,
            ErrNo::Busy => EBUSY_STR,
            ErrNo::Exist => EEXIST_STR,
            ErrNo::XDev => EXDEV_STR,
            ErrNo::NoDev => ENODEV_STR,
            ErrNo::NotDir => ENOTDIR_STR,
            ErrNo::IsDir => EISDIR_STR,
            ErrNo::Inval => EINVAL_STR,
            ErrNo::NFile => ENFILE_STR,
            ErrNo::MFile => EMFILE_STR,
            ErrNo::NotTy => ENOTTY_STR,
            ErrNo::TxtBsy => ETXTBSY_STR,
            ErrNo::FBig => EFBIG_STR,
            ErrNo::NoSpc => ENOSPC_STR,
            ErrNo::SPipe => ESPIPE_STR,
            ErrNo::ROFS => EROFS_STR,
            ErrNo::MLink => EMLINK_STR,
            ErrNo::Pipe => EPIPE_STR,
            ErrNo::Dom => EDOM_STR,
            ErrNo::Range => ERANGE_STR,
            ErrNo::DeadLk => EDEADLK_STR,
            ErrNo::NameTooLong => ENAMETOOLONG_STR,
            ErrNo::NoLck => ENOLCK_STR,
            ErrNo::NoSys => ENOSYS_STR,
            ErrNo::NotEmpty => ENOTEMPTY_STR,
            ErrNo::Loop => ELOOP_STR,
            ErrNo::WouldBlock => EWOULDBLOCK_STR,
            ErrNo::NoMsg => ENOMSG_STR,
            ErrNo::IdRm => EIDRM_STR,
            ErrNo::ChRng => ECHRNG_STR,
            ErrNo::L2NSync => EL2NSYNC_STR,
            ErrNo::L3Hlt => EL3HLT_STR,
            ErrNo::L3Rst => EL3RST_STR,
            ErrNo::LNRng => ELNRNG_STR,
            ErrNo::Unatch => EUNATCH_STR,
            ErrNo::NoCSI => ENOCSI_STR,
            ErrNo::L2Hlt => EL2HLT_STR,
            ErrNo::BadE => EBADE_STR,
            ErrNo::BadR => EBADR_STR,
            ErrNo::XFull => EXFULL_STR,
            ErrNo::NoAno => ENOANO_STR,
            ErrNo::BadRqC => EBADRQC_STR,
            ErrNo::BadSlt => EBADSLT_STR,
            ErrNo::BFont => EBFONT_STR,
            ErrNo::NoStr => ENOSTR_STR,
            ErrNo::NoData => ENODATA_STR,
            ErrNo::Time => ETIME_STR,
            ErrNo::NoSR => ENOSR_STR,
            ErrNo::NoNet => ENONET_STR,
            ErrNo::NoPkg => ENOPKG_STR,
            ErrNo::NoLink => ENOLINK_STR,
            ErrNo::Adv => EADV_STR,
            ErrNo::SrMnt => ESRMNT_STR,
            ErrNo::Comm => ECOMM_STR,
            ErrNo::Proto => EPROTO_STR,
            ErrNo::MultiHop => EMULTIHOP_STR,
            ErrNo::DotDot => EDOTDOT_STR,
            ErrNo::BadMsg => EBADMSG_STR,
            ErrNo::Overflow => EOVERFLOW_STR,
            ErrNo::NotUniq => ENOTUNIQ_STR,
            ErrNo::BadFD => EBADFD_STR,
            ErrNo::RemChg => EREMCHG_STR,
            ErrNo::LibAcc => ELIBACC_STR,
            ErrNo::LibBad => ELIBBAD_STR,
            ErrNo::LibScn => ELIBSCN_STR,
            ErrNo::LibMax => ELIBMAX_STR,
            ErrNo::LibExec => ELIBEXEC_STR,
            ErrNo::IlSeq => EILSEQ_STR,
            ErrNo::Restart => ERESTART_STR,
            ErrNo::StrPipe => ESTRPIPE_STR,
            ErrNo::Users => EUSERS_STR,
            ErrNo::NotSock => ENOTSOCK_STR,
            ErrNo::DestAddrReq => EDESTADDRREQ_STR,
            ErrNo::MsgSize => EMSGSIZE_STR,
            ErrNo::ProtoType => EPROTOTYPE_STR,
            ErrNo::NoProtoOpt => ENOPROTOOPT_STR,
            ErrNo::ProtoNoSupport => EPROTONOSUPPORT_STR,
            ErrNo::SockTNoSupport => ESOCKTNOSUPPORT_STR,
            ErrNo::OpNotSupp => EOPNOTSUPP_STR,
            ErrNo::PFNoSupport => EPFNOSUPPORT_STR,
            ErrNo::AFNoSupport => EAFNOSUPPORT_STR,
            ErrNo::AddrInUse => EADDRINUSE_STR,
            ErrNo::AddrNotAvail => EADDRNOTAVAIL_STR,
            ErrNo::NetDown => ENETDOWN_STR,
            ErrNo::NetUnreach => ENETUNREACH_STR,
            ErrNo::NetReset => ENETRESET_STR,
            ErrNo::ConnAborted => ECONNABORTED_STR,
            ErrNo::ConnReset => ECONNRESET_STR,
            ErrNo::NoBufS => ENOBUFS_STR,
            ErrNo::IsConn => EISCONN_STR,
            ErrNo::NotConn => ENOTCONN_STR,
            ErrNo::Shutdown => ESHUTDOWN_STR,
            ErrNo::TooManyRefs => ETOOMANYREFS_STR,
            ErrNo::TimedOut => ETIMEDOUT_STR,
            ErrNo::ConnRefused => ECONNREFUSED_STR,
            ErrNo::HostDown => EHOSTDOWN_STR,
            ErrNo::HostUnreach => EHOSTUNREACH_STR,
            ErrNo::Already => EALREADY_STR,
            ErrNo::InProgress => EINPROGRESS_STR,
            ErrNo::Stale => ESTALE_STR,
            ErrNo::UClean => EUCLEAN_STR,
            ErrNo::NotNam => ENOTNAM_STR,
            ErrNo::NAvail => ENAVAIL_STR,
            ErrNo::IsNam => EISNAM_STR,
            ErrNo::RemoteIO => EREMOTEIO_STR,
            _ => EOTHER_STR,
        }
    }
}

impl From<&str> for ErrNo {
    fn from(e: &str) -> Self {
        match e {
            EPERM_STR => Self::Perm,
            ENOENT_STR => Self::NoEnt,
            ESRCH_STR => Self::Srch,
            EINTR_STR => Self::Intr,
            EIO_STR => Self::Io,
            ENXIO_STR => Self::NxIo,
            E2BIG_STR => Self::TooBig,
            ENOEXEC_STR => Self::NoExec,
            EBADF_STR => Self::BadF,
            ECHILD_STR => Self::Child,
            ENOMEM_STR => Self::NoMem,
            EFAULT_STR => Self::Fault,
            ENOTBLK_STR => Self::NotBlk,
            EBUSY_STR => Self::Busy,
            EEXIST_STR => Self::Exist,
            EXDEV_STR => Self::XDev,
            ENODEV_STR => Self::NoDev,
            ENOTDIR_STR => Self::NotDir,
            EISDIR_STR => Self::IsDir,
            EINVAL_STR => Self::Inval,
            ENFILE_STR => Self::NFile,
            EMFILE_STR => Self::MFile,
            ENOTTY_STR => Self::NotTy,
            ETXTBSY_STR => Self::TxtBsy,
            EFBIG_STR => Self::FBig,
            ENOSPC_STR => Self::NoSpc,
            ESPIPE_STR => Self::SPipe,
            EROFS_STR => Self::ROFS,
            EMLINK_STR => Self::MLink,
            EPIPE_STR => Self::Pipe,
            EDOM_STR => Self::Dom,
            ERANGE_STR => Self::Range,
            EDEADLK_STR => Self::DeadLk,
            ENAMETOOLONG_STR => Self::NameTooLong,
            ENOLCK_STR => Self::NoLck,
            ENOSYS_STR => Self::NoSys,
            ENOTEMPTY_STR => Self::NotEmpty,
            ELOOP_STR => Self::Loop,
            EWOULDBLOCK_STR => Self::WouldBlock,
            ENOMSG_STR => Self::NoMsg,
            EIDRM_STR => Self::IdRm,
            ECHRNG_STR => Self::ChRng,
            EL2NSYNC_STR => Self::L2NSync,
            EL3HLT_STR => Self::L3Hlt,
            EL3RST_STR => Self::L3Rst,
            ELNRNG_STR => Self::LNRng,
            EUNATCH_STR => Self::Unatch,
            ENOCSI_STR => Self::NoCSI,
            EL2HLT_STR => Self::L2Hlt,
            EBADE_STR => Self::BadE,
            EBADR_STR => Self::BadR,
            EXFULL_STR => Self::XFull,
            ENOANO_STR => Self::NoAno,
            EBADRQC_STR => Self::BadRqC,
            EBADSLT_STR => Self::BadSlt,
            EBFONT_STR => Self::BFont,
            ENOSTR_STR => Self::NoStr,
            ENODATA_STR => Self::NoData,
            ETIME_STR => Self::Time,
            ENOSR_STR => Self::NoSR,
            ENONET_STR => Self::NoNet,
            ENOPKG_STR => Self::NoPkg,
            ENOLINK_STR => Self::NoLink,
            EADV_STR => Self::Adv,
            ESRMNT_STR => Self::SrMnt,
            ECOMM_STR => Self::Comm,
            EPROTO_STR => Self::Proto,
            EMULTIHOP_STR => Self::MultiHop,
            EDOTDOT_STR => Self::DotDot,
            EBADMSG_STR => Self::BadMsg,
            EOVERFLOW_STR => Self::Overflow,
            ENOTUNIQ_STR => Self::NotUniq,
            EBADFD_STR => Self::BadFD,
            EREMCHG_STR => Self::RemChg,
            ELIBACC_STR => Self::LibAcc,
            ELIBBAD_STR => Self::LibBad,
            ELIBSCN_STR => Self::LibScn,
            ELIBMAX_STR => Self::LibMax,
            ELIBEXEC_STR => Self::LibExec,
            EILSEQ_STR => Self::IlSeq,
            ERESTART_STR => Self::Restart,
            ESTRPIPE_STR => Self::StrPipe,
            EUSERS_STR => Self::Users,
            ENOTSOCK_STR => Self::NotSock,
            EDESTADDRREQ_STR => Self::DestAddrReq,
            EMSGSIZE_STR => Self::MsgSize,
            EPROTOTYPE_STR => Self::ProtoType,
            ENOPROTOOPT_STR => Self::NoProtoOpt,
            EPROTONOSUPPORT_STR => Self::ProtoNoSupport,
            ESOCKTNOSUPPORT_STR => Self::SockTNoSupport,
            EOPNOTSUPP_STR => Self::OpNotSupp,
            EPFNOSUPPORT_STR => Self::PFNoSupport,
            EAFNOSUPPORT_STR => Self::AFNoSupport,
            EADDRINUSE_STR => Self::AddrInUse,
            EADDRNOTAVAIL_STR => Self::AddrNotAvail,
            ENETDOWN_STR => Self::NetDown,
            ENETUNREACH_STR => Self::NetUnreach,
            ENETRESET_STR => Self::NetReset,
            ECONNABORTED_STR => Self::ConnAborted,
            ECONNRESET_STR => Self::ConnReset,
            ENOBUFS_STR => Self::NoBufS,
            EISCONN_STR => Self::IsConn,
            ENOTCONN_STR => Self::NotConn,
            ESHUTDOWN_STR => Self::Shutdown,
            ETOOMANYREFS_STR => Self::TooManyRefs,
            ETIMEDOUT_STR => Self::TimedOut,
            ECONNREFUSED_STR => Self::ConnRefused,
            EHOSTDOWN_STR => Self::HostDown,
            EHOSTUNREACH_STR => Self::HostUnreach,
            EALREADY_STR => Self::Already,
            EINPROGRESS_STR => Self::InProgress,
            ESTALE_STR => Self::Stale,
            EUCLEAN_STR => Self::UClean,
            ENOTNAM_STR => Self::NotNam,
            ENAVAIL_STR => Self::NAvail,
            EISNAM_STR => Self::IsNam,
            EREMOTEIO_STR => Self::RemoteIO,
            _ => Self::Other,
        }
    }
}

impl serde::Serialize for ErrNo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let r: &'static str = self.into();
        serializer.serialize_str(r)
    }
}

impl<'de> serde::Deserialize<'de> for ErrNo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let e: &'de str = serde::Deserialize::deserialize(deserializer)?;
        Ok(e.into())
    }
}
