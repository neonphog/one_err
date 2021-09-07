// we could write fancy macros to do most of this...
// but it compiles faster when just written out.

// there are a couple inconsistent things accross os-es
// and rusts std::io::ErrorKind combines a couple things we have to work around.
const EPERM_1: i32 = libc::EPERM;
const EPERM_2: i32 = libc::EACCES;
const EWOULDBLOCK_1: i32 = libc::EAGAIN;
const EWOULDBLOCK_2: i32 = libc::EWOULDBLOCK;
const EDEADLOCK_1: i32 = libc::EDEADLK;
const EDEADLOCK_2: i32 = libc::EDEADLOCK;

// incase we get an errno not in our list
const EOTHER: i32 = -1;

/// Rust translation of errno.h
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrNo {
    /// Permission denied
    Perm = EPERM_1,

    /// No such file or directory
    NoEnt = libc::ENOENT,

    /// No such process
    Srch = libc::ESRCH,

    /// Interrupted system call
    Intr = libc::EINTR,

    /// I/O error
    Io = libc::EIO,

    /// No such device or address
    NxIo = libc::ENXIO,

    /// Arg list too long
    TooBig = libc::E2BIG,

    /// Exec format error
    NoExec = libc::ENOEXEC,

    /// Bad file number
    BadF = libc::EBADF,

    /// No child processes
    Child = libc::ECHILD,

    /// Out of memory
    NoMem = libc::ENOMEM,

    /// Bad address
    Fault = libc::EFAULT,

    /// Block device required
    NotBlk = libc::ENOTBLK,

    /// Device or resource busy
    Busy = libc::EBUSY,

    /// File exists
    Exist = libc::EEXIST,

    /// Cross-device link
    XDev = libc::EXDEV,

    /// No such device
    NoDev = libc::ENODEV,

    /// Not a directory
    NotDir = libc::ENOTDIR,

    /// Is a directory
    IsDir = libc::EISDIR,

    /// Invalid argument
    Inval = libc::EINVAL,

    /// File table overflow
    NFile = libc::ENFILE,

    /// Too many open files
    MFile = libc::EMFILE,

    /// Not a typewriter
    NotTy = libc::ENOTTY,

    /// Text file busy
    TxtBsy = libc::ETXTBSY,

    /// File too large
    FBig = libc::EFBIG,

    /// No space left on device
    NoSpc = libc::ENOSPC,

    /// Illegal seek
    SPipe = libc::ESPIPE,

    /// Read-only file system
    ROFS = libc::EROFS,

    /// Too many links
    MLink = libc::EMLINK,

    /// Broken pipe
    Pipe = libc::EPIPE,

    /// Math argument out of domain of func
    Dom = libc::EDOM,

    /// Math result not representable
    Range = libc::ERANGE,

    /// Resource deadlock would occur
    DeadLk = EDEADLOCK_1,

    /// File name too long
    NameTooLong = libc::ENAMETOOLONG,

    /// No record locks available
    NoLck = libc::ENOLCK,

    /// Function not implemented
    NoSys = libc::ENOSYS,

    /// Directory not empty
    NotEmpty = libc::ENOTEMPTY,

    /// Too many symbolic links encountered
    Loop = libc::ELOOP,

    /// Operation would block
    WouldBlock = EWOULDBLOCK_1,

    /// No message of desired type
    NoMsg = libc::ENOMSG,

    /// Identifier removed
    IdRm = libc::EIDRM,

    // /// Channel number out of range
    // ChRng = libc::ECHRNG,

    // /// Level 2 not synchronized
    // L2NSync = libc::EL2NSYNC,

    // /// Level 3 halted
    // L3Hlt = libc::EL3HLT,

    // /// Level 3 reset
    // L3Rst = libc::EL3RST,

    // /// Link number out of range
    // LNRng = libc::ELNRNG,

    // /// Protocol driver not attached
    // Unatch = libc::EUNATCH,

    // /// No CSI structure available
    // NoCSI = libc::ENOCSI,

    // /// Level 2 halted
    // L2Hlt = libc::EL2HLT,

    // /// Invalid exchange
    // BadE = libc::EBADE,

    // /// Invalid request descriptor
    // BadR = libc::EBADR,

    // /// Exchange full
    // XFull = libc::EXFULL,

    // /// No anode
    // NoAno = libc::ENOANO,

    // /// Invalid request code
    // BadRqC = libc::EBADRQC,

    // /// Invalid slot
    // BadSlt = libc::EBADSLT,

    // /// Bad font file format
    // BFont = libc::EBFONT,
    /// Device not a stream
    NoStr = libc::ENOSTR,

    /// No data available
    NoData = libc::ENODATA,

    /// Timer expired
    Time = libc::ETIME,

    /// Out of streams resources
    NoSR = libc::ENOSR,

    // /// Machine is not on the network
    // NoNet = libc::ENONET,

    // /// Package not installed
    // NoPkg = libc::ENOPKG,
    /// Object is remote
    Remote = libc::EREMOTE,

    /// Link has been severed
    NoLink = libc::ENOLINK,

    // /// Advertise error
    // Adv = libc::EADV,

    // /// Srmount error
    // SrMnt = libc::ESRMNT,

    // /// Communication error on send
    // Comm = libc::ECOMM,
    /// Protocol error
    Proto = libc::EPROTO,

    /// Multihop attempted
    MultiHop = libc::EMULTIHOP,

    // /// RFS specific error
    // DotDot = libc::EDOTDOT,
    /// Not a data message
    BadMsg = libc::EBADMSG,

    /// Value too large for defined data type
    Overflow = libc::EOVERFLOW,

    // /// Name not unique on network
    // NotUniq = libc::ENOTUNIQ,

    // /// File descriptor in bad state
    // BadFD = libc::EBADFD,

    // /// Remote address changed
    // RemChg = libc::EREMCHG,

    // /// Can not access a needed shared library
    // LibAcc = libc::ELIBACC,

    // /// Accessing a corrupted shared library
    // LibBad = libc::ELIBBAD,

    // /// .lib section in a.out corrupted
    // LibScn = libc::ELIBSCN,

    // /// Attempting to link in too many shared libraries
    // LibMax = libc::ELIBMAX,

    // /// Cannot exec a shared library directly
    // LibExec = libc::ELIBEXEC,
    /// Illegal byte sequence
    IlSeq = libc::EILSEQ,

    // /// Interrupted system call should be restarted
    // Restart = libc::ERESTART,

    // /// Streams pipe error
    // StrPipe = libc::ESTRPIPE,
    /// Too many users
    Users = libc::EUSERS,

    /// Socket operation on non-socket
    NotSock = libc::ENOTSOCK,

    /// Destination address required
    DestAddrReq = libc::EDESTADDRREQ,

    /// Message too long
    MsgSize = libc::EMSGSIZE,

    /// Protocol wrong type for socket
    ProtoType = libc::EPROTOTYPE,

    /// Protocol not available
    NoProtoOpt = libc::ENOPROTOOPT,

    /// Protocol not supported
    ProtoNoSupport = libc::EPROTONOSUPPORT,

    /// Socket type not supported
    SockTNoSupport = libc::ESOCKTNOSUPPORT,

    /// Operation not supported on transport endpoint
    OpNotSupp = libc::EOPNOTSUPP,

    /// Protocol family not supported
    PFNoSupport = libc::EPFNOSUPPORT,

    /// Address family not supported by protocol
    AFNoSupport = libc::EAFNOSUPPORT,

    /// Address already in use
    AddrInUse = libc::EADDRINUSE,

    /// Cannot assign requested address
    AddrNotAvail = libc::EADDRNOTAVAIL,

    /// Network is down
    NetDown = libc::ENETDOWN,

    /// Network is unreachable
    NetUnreach = libc::ENETUNREACH,

    /// Network dropped connection because of reset
    NetReset = libc::ENETRESET,

    /// Software caused connection abort
    ConnAborted = libc::ECONNABORTED,

    /// Connection reset by peer
    ConnReset = libc::ECONNRESET,

    /// No buffer space available
    NoBufS = libc::ENOBUFS,

    /// Transport endpoint is already connected
    IsConn = libc::EISCONN,

    /// Transport endpoint is not connected
    NotConn = libc::ENOTCONN,

    /// Cannot send after transport endpoint shutdown
    Shutdown = libc::ESHUTDOWN,

    /// Too many references: cannot splice
    TooManyRefs = libc::ETOOMANYREFS,

    /// Connection timed out
    TimedOut = libc::ETIMEDOUT,

    /// Connection refused
    ConnRefused = libc::ECONNREFUSED,

    /// Host is down
    HostDown = libc::EHOSTDOWN,

    /// No route to host
    HostUnreach = libc::EHOSTUNREACH,

    /// Operation already in progress
    Already = libc::EALREADY,

    /// Operation now in progress
    InProgress = libc::EINPROGRESS,

    /// Stale NFS file handle
    Stale = libc::ESTALE,

    // /// Structure needs cleaning
    // UClean = libc::EUCLEAN,

    // /// Not a Xlibc::ENIX named type file
    // NotNam = libc::ENOTNAM,

    // /// No Xlibc::ENIX semaphores available
    // NAvail = libc::ENAVAIL,

    // /// Is a named type file
    // IsNam = libc::EISNAM,

    // /// Remote I/O error
    // RemoteIO = libc::EREMOTEIO,
    /// Other / Unrecognized Error
    Other = EOTHER,
}

impl std::fmt::Display for ErrNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&'static str>::from(self))
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
            libc::ENOENT => Self::NoEnt,
            libc::ESRCH => Self::Srch,
            libc::EINTR => Self::Intr,
            libc::EIO => Self::Io,
            libc::ENXIO => Self::NxIo,
            libc::E2BIG => Self::TooBig,
            libc::ENOEXEC => Self::NoExec,
            libc::EBADF => Self::BadF,
            libc::ECHILD => Self::Child,
            libc::ENOMEM => Self::NoMem,
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
            libc::ENAMETOOLONG => Self::NameTooLong,
            libc::ENOLCK => Self::NoLck,
            libc::ENOSYS => Self::NoSys,
            libc::ENOTEMPTY => Self::NotEmpty,
            libc::ELOOP => Self::Loop,
            libc::ENOMSG => Self::NoMsg,
            libc::EIDRM => Self::IdRm,
            //libc::ECHRNG => Self::ChRng,
            //libc::EL2NSYNC => Self::L2NSync,
            //libc::EL3HLT => Self::L3Hlt,
            //libc::EL3RST => Self::L3Rst,
            //libc::ELNRNG => Self::LNRng,
            //libc::EUNATCH => Self::Unatch,
            //libc::ENOCSI => Self::NoCSI,
            //libc::EL2HLT => Self::L2Hlt,
            //libc::EBADE => Self::BadE,
            //libc::EBADR => Self::BadR,
            //libc::EXFULL => Self::XFull,
            //libc::ENOANO => Self::NoAno,
            //libc::EBADRQC => Self::BadRqC,
            //libc::EBADSLT => Self::BadSlt,
            //libc::EBFONT => Self::BFont,
            libc::ENOSTR => Self::NoStr,
            libc::ENODATA => Self::NoData,
            libc::ETIME => Self::Time,
            libc::ENOSR => Self::NoSR,
            //libc::ENONET => Self::NoNet,
            //libc::ENOPKG => Self::NoPkg,
            libc::EREMOTE => Self::Remote,
            libc::ENOLINK => Self::NoLink,
            //libc::EADV => Self::Adv,
            //libc::ESRMNT => Self::SrMnt,
            //libc::ECOMM => Self::Comm,
            libc::EPROTO => Self::Proto,
            libc::EMULTIHOP => Self::MultiHop,
            //libc::EDOTDOT => Self::DotDot,
            libc::EBADMSG => Self::BadMsg,
            libc::EOVERFLOW => Self::Overflow,
            //libc::ENOTUNIQ => Self::NotUniq,
            //libc::EBADFD => Self::BadFD,
            //libc::EREMCHG => Self::RemChg,
            //libc::ELIBACC => Self::LibAcc,
            //libc::ELIBBAD => Self::LibBad,
            //libc::ELIBSCN => Self::LibScn,
            //libc::ELIBMAX => Self::LibMax,
            //libc::ELIBEXEC => Self::LibExec,
            libc::EILSEQ => Self::IlSeq,
            //libc::ERESTART => Self::Restart,
            //libc::ESTRPIPE => Self::StrPipe,
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
            //libc::EUCLEAN => Self::UClean,
            //libc::ENOTNAM => Self::NotNam,
            //libc::ENAVAIL => Self::NAvail,
            //libc::EISNAM => Self::IsNam,
            //libc::EREMOTEIO => Self::RemoteIO,
            x if x == EPERM_1 || x == EPERM_2 => Self::Perm,
            x if x == EDEADLOCK_1 || x == EDEADLOCK_2 => Self::DeadLk,
            x if x == EWOULDBLOCK_1 || x == EWOULDBLOCK_2 => Self::WouldBlock,
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

impl From<std::io::ErrorKind> for ErrNo {
    fn from(e: std::io::ErrorKind) -> Self {
        (&e).into()
    }
}

impl From<&std::io::ErrorKind> for ErrNo {
    fn from(e: &std::io::ErrorKind) -> Self {
        use std::io::ErrorKind::*;
        match e {
            AddrInUse => ErrNo::AddrInUse,
            AddrNotAvailable => ErrNo::AddrNotAvail,
            ConnectionAborted => ErrNo::ConnAborted,
            ConnectionRefused => ErrNo::ConnRefused,
            ConnectionReset => ErrNo::ConnReset,
            PermissionDenied => ErrNo::Perm,
            AlreadyExists => ErrNo::Exist,
            Interrupted => ErrNo::Intr,
            InvalidInput => ErrNo::Inval,
            NotFound => ErrNo::NoEnt,
            OutOfMemory => ErrNo::NoMem,
            Unsupported => ErrNo::NoSys,
            NotConnected => ErrNo::NotConn,
            BrokenPipe => ErrNo::Pipe,
            WouldBlock => ErrNo::WouldBlock,
            TimedOut => ErrNo::TimedOut,
            _ => ErrNo::Other,
        }
    }
}

/// Constants for working with ErrNo `str_kind()`.
pub mod errno {
    /// The const str 'EPERM'.
    pub const EPERM_STR: &str = "EPERM";

    /// The const str 'ENOENT'.
    pub const ENOENT_STR: &str = "ENOENT";

    /// The const str 'ESRCH'.
    pub const ESRCH_STR: &str = "ESRCH";

    /// The const str 'EINTR'.
    pub const EINTR_STR: &str = "EINTR";

    /// The const str 'EIO'.
    pub const EIO_STR: &str = "EIO";

    /// The const str 'ENXIO'.
    pub const ENXIO_STR: &str = "ENXIO";

    /// The const str 'E2BIG'.
    pub const E2BIG_STR: &str = "E2BIG";

    /// The const str 'ENOEXEC'.
    pub const ENOEXEC_STR: &str = "ENOEXEC";

    /// The const str 'EBADF'.
    pub const EBADF_STR: &str = "EBADF";

    /// The const str 'ECHILD'.
    pub const ECHILD_STR: &str = "ECHILD";

    /// The const str 'ENOMEM'.
    pub const ENOMEM_STR: &str = "ENOMEM";

    /// The const str 'EFAULT'.
    pub const EFAULT_STR: &str = "EFAULT";

    /// The const str 'ENOTBLK'.
    pub const ENOTBLK_STR: &str = "ENOTBLK";

    /// The const str 'EBUSY'.
    pub const EBUSY_STR: &str = "EBUSY";

    /// The const str 'EEXIST'.
    pub const EEXIST_STR: &str = "EEXIST";

    /// The const str 'EXDEV'.
    pub const EXDEV_STR: &str = "EXDEV";

    /// The const str 'ENODEV'.
    pub const ENODEV_STR: &str = "ENODEV";

    /// The const str 'ENOTDIR'.
    pub const ENOTDIR_STR: &str = "ENOTDIR";

    /// The const str 'EISDIR'.
    pub const EISDIR_STR: &str = "EISDIR";

    /// The const str 'EINVAL'.
    pub const EINVAL_STR: &str = "EINVAL";

    /// The const str 'ENFILE'.
    pub const ENFILE_STR: &str = "ENFILE";

    /// The const str 'EMFILE'.
    pub const EMFILE_STR: &str = "EMFILE";

    /// The const str 'ENOTTY'.
    pub const ENOTTY_STR: &str = "ENOTTY";

    /// The const str 'ETXTBSY'.
    pub const ETXTBSY_STR: &str = "ETXTBSY";

    /// The const str 'EFBIG'.
    pub const EFBIG_STR: &str = "EFBIG";

    /// The const str 'ENOSPC'.
    pub const ENOSPC_STR: &str = "ENOSPC";

    /// The const str 'ESPIPE'.
    pub const ESPIPE_STR: &str = "ESPIPE";

    /// The const str 'EROFS'.
    pub const EROFS_STR: &str = "EROFS";

    /// The const str 'EMLINK'.
    pub const EMLINK_STR: &str = "EMLINK";

    /// The const str 'EPIPE'.
    pub const EPIPE_STR: &str = "EPIPE";

    /// The const str 'EDOM'.
    pub const EDOM_STR: &str = "EDOM";

    /// The const str 'ERANGE'.
    pub const ERANGE_STR: &str = "ERANGE";

    /// The const str 'EDEADLK'.
    pub const EDEADLK_STR: &str = "EDEADLK";

    /// The const str 'ENAMETOOLONG'.
    pub const ENAMETOOLONG_STR: &str = "ENAMETOOLONG";

    /// The const str 'ENOLCK'.
    pub const ENOLCK_STR: &str = "ENOLCK";

    /// The const str 'ENOSYS'.
    pub const ENOSYS_STR: &str = "ENOSYS";

    /// The const str 'ENOTEMPTY'.
    pub const ENOTEMPTY_STR: &str = "ENOTEMPTY";

    /// The const str 'ELOOP'.
    pub const ELOOP_STR: &str = "ELOOP";

    /// The const str 'EWOULDBLOCK'.
    pub const EWOULDBLOCK_STR: &str = "EWOULDBLOCK";

    /// The const str 'ENOMSG'.
    pub const ENOMSG_STR: &str = "ENOMSG";

    /// The const str 'EIDRM'.
    pub const EIDRM_STR: &str = "EIDRM";

    // /// The const str 'ECHRNG'.
    // pub const ECHRNG_STR: &str = "ECHRNG";

    // /// The const str 'EL2NSYNC'.
    // pub const EL2NSYNC_STR: &str = "EL2NSYNC";

    // /// The const str 'EL3HLT'.
    // pub const EL3HLT_STR: &str = "EL3HLT";

    // /// The const str 'EL3RST'.
    // pub const EL3RST_STR: &str = "EL3RST";

    // /// The const str 'ELNRNG'.
    // pub const ELNRNG_STR: &str = "ELNRNG";

    // /// The const str 'EUNATCH'.
    // pub const EUNATCH_STR: &str = "EUNATCH";

    // /// The const str 'ENOCSI'.
    // pub const ENOCSI_STR: &str = "ENOCSI";

    // /// The const str 'EL2HLT'.
    // pub const EL2HLT_STR: &str = "EL2HLT";

    // /// The const str 'EBADE'.
    // pub const EBADE_STR: &str = "EBADE";

    // /// The const str 'EBADR'.
    // pub const EBADR_STR: &str = "EBADR";

    // /// The const str 'EXFULL'.
    // pub const EXFULL_STR: &str = "EXFULL";

    // /// The const str 'ENOANO'.
    // pub const ENOANO_STR: &str = "ENOANO";

    // /// The const str 'EBADRQC'.
    // pub const EBADRQC_STR: &str = "EBADRQC";

    // /// The const str 'EBADSLT'.
    // pub const EBADSLT_STR: &str = "EBADSLT";

    // /// The const str 'EBFONT'.
    // pub const EBFONT_STR: &str = "EBFONT";

    /// The const str 'ENOSTR'.
    pub const ENOSTR_STR: &str = "ENOSTR";

    /// The const str 'ENODATA'.
    pub const ENODATA_STR: &str = "ENODATA";

    /// The const str 'ETIME'.
    pub const ETIME_STR: &str = "ETIME";

    /// The const str 'ENOSR'.
    pub const ENOSR_STR: &str = "ENOSR";

    // /// The const str 'ENONET'.
    // pub const ENONET_STR: &str = "ENONET";

    // /// The const str 'ENOPKG'.
    // pub const ENOPKG_STR: &str = "ENOPKG";

    /// The const str 'EREMOTE'.
    pub const EREMOTE_STR: &str = "EREMOTE";

    /// The const str 'ENOLINK'.
    pub const ENOLINK_STR: &str = "ENOLINK";

    // /// The const str 'EADV'.
    // pub const EADV_STR: &str = "EADV";

    // /// The const str 'ESRMNT'.
    // pub const ESRMNT_STR: &str = "ESRMNT";

    // /// The const str 'ECOMM'.
    // pub const ECOMM_STR: &str = "ECOMM";

    /// The const str 'EPROTO'.
    pub const EPROTO_STR: &str = "EPROTO";

    /// The const str 'EMULTIHOP'.
    pub const EMULTIHOP_STR: &str = "EMULTIHOP";

    // /// The const str 'EDOTDOT'.
    // pub const EDOTDOT_STR: &str = "EDOTDOT";

    /// The const str 'EBADMSG'.
    pub const EBADMSG_STR: &str = "EBADMSG";

    /// The const str 'EOVERFLOW'.
    pub const EOVERFLOW_STR: &str = "EOVERFLOW";

    // /// The const str 'ENOTUNIQ'.
    // pub const ENOTUNIQ_STR: &str = "ENOTUNIQ";

    // /// The const str 'EBADFD'.
    // pub const EBADFD_STR: &str = "EBADFD";

    // /// The const str 'EREMCHG'.
    // pub const EREMCHG_STR: &str = "EREMCHG";

    // /// The const str 'ELIBACC'.
    // pub const ELIBACC_STR: &str = "ELIBACC";

    // /// The const str 'ELIBBAD'.
    // pub const ELIBBAD_STR: &str = "ELIBBAD";

    // /// The const str 'ELIBSCN'.
    // pub const ELIBSCN_STR: &str = "ELIBSCN";

    // /// The const str 'ELIBMAX'.
    // pub const ELIBMAX_STR: &str = "ELIBMAX";

    // /// The const str 'ELIBEXEC'.
    // pub const ELIBEXEC_STR: &str = "ELIBEXEC";

    /// The const str 'EILSEQ'.
    pub const EILSEQ_STR: &str = "EILSEQ";

    // /// The const str 'ERESTART'.
    // pub const ERESTART_STR: &str = "ERESTART";

    // /// The const str 'ESTRPIPE'.
    // pub const ESTRPIPE_STR: &str = "ESTRPIPE";

    /// The const str 'EUSERS'.
    pub const EUSERS_STR: &str = "EUSERS";

    /// The const str 'ENOTSOCK'.
    pub const ENOTSOCK_STR: &str = "ENOTSOCK";

    /// The const str 'EDESTADDRREQ'.
    pub const EDESTADDRREQ_STR: &str = "EDESTADDRREQ";

    /// The const str 'EMSGSIZE'.
    pub const EMSGSIZE_STR: &str = "EMSGSIZE";

    /// The const str 'EPROTOTYPE'.
    pub const EPROTOTYPE_STR: &str = "EPROTOTYPE";

    /// The const str 'ENOPROTOOPT'.
    pub const ENOPROTOOPT_STR: &str = "ENOPROTOOPT";

    /// The const str 'EPROTONOSUPPORT'.
    pub const EPROTONOSUPPORT_STR: &str = "EPROTONOSUPPORT";

    /// The const str 'ESOCKTNOSUPPORT'.
    pub const ESOCKTNOSUPPORT_STR: &str = "ESOCKTNOSUPPORT";

    /// The const str 'EOPNOTSUPP'.
    pub const EOPNOTSUPP_STR: &str = "EOPNOTSUPP";

    /// The const str 'EPFNOSUPPORT'.
    pub const EPFNOSUPPORT_STR: &str = "EPFNOSUPPORT";

    /// The const str 'EAFNOSUPPORT'.
    pub const EAFNOSUPPORT_STR: &str = "EAFNOSUPPORT";

    /// The const str 'EADDRINUSE'.
    pub const EADDRINUSE_STR: &str = "EADDRINUSE";

    /// The const str 'EADDRNOTAVAIL'.
    pub const EADDRNOTAVAIL_STR: &str = "EADDRNOTAVAIL";

    /// The const str 'ENETDOWN'.
    pub const ENETDOWN_STR: &str = "ENETDOWN";

    /// The const str 'ENETUNREACH'.
    pub const ENETUNREACH_STR: &str = "ENETUNREACH";

    /// The const str 'ENETRESET'.
    pub const ENETRESET_STR: &str = "ENETRESET";

    /// The const str 'ECONNABORTED'.
    pub const ECONNABORTED_STR: &str = "ECONNABORTED";

    /// The const str 'ECONNRESET'.
    pub const ECONNRESET_STR: &str = "ECONNRESET";

    /// The const str 'ENOBUFS'.
    pub const ENOBUFS_STR: &str = "ENOBUFS";

    /// The const str 'EISCONN'.
    pub const EISCONN_STR: &str = "EISCONN";

    /// The const str 'ENOTCONN'.
    pub const ENOTCONN_STR: &str = "ENOTCONN";

    /// The const str 'ESHUTDOWN'.
    pub const ESHUTDOWN_STR: &str = "ESHUTDOWN";

    /// The const str 'ETOOMANYREFS'.
    pub const ETOOMANYREFS_STR: &str = "ETOOMANYREFS";

    /// The const str 'ETIMEDOUT'.
    pub const ETIMEDOUT_STR: &str = "ETIMEDOUT";

    /// The const str 'ECONNREFUSED'.
    pub const ECONNREFUSED_STR: &str = "ECONNREFUSED";

    /// The const str 'EHOSTDOWN'.
    pub const EHOSTDOWN_STR: &str = "EHOSTDOWN";

    /// The const str 'EHOSTUNREACH'.
    pub const EHOSTUNREACH_STR: &str = "EHOSTUNREACH";

    /// The const str 'EALREADY'.
    pub const EALREADY_STR: &str = "EALREADY";

    /// The const str 'EINPROGRESS'.
    pub const EINPROGRESS_STR: &str = "EINPROGRESS";

    /// The const str 'ESTALE'.
    pub const ESTALE_STR: &str = "ESTALE";

    // /// The const str 'EUCLEAN'.
    // pub const EUCLEAN_STR: &str = "EUCLEAN";

    // /// The const str 'ENOTNAM'.
    // pub const ENOTNAM_STR: &str = "ENOTNAM";

    // /// The const str 'ENAVAIL'.
    // pub const ENAVAIL_STR: &str = "ENAVAIL";

    // /// The const str 'EISNAM'.
    // pub const EISNAM_STR: &str = "EISNAM";

    // /// The const str 'EREMOTEIO'.
    // pub const EREMOTEIO_STR: &str = "EREMOTEIO";

    /// The const str 'EOTHER'.
    pub const EOTHER_STR: &str = "EOTHER";
}
use errno::*;

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
            //ErrNo::ChRng => ECHRNG_STR,
            //ErrNo::L2NSync => EL2NSYNC_STR,
            //ErrNo::L3Hlt => EL3HLT_STR,
            //ErrNo::L3Rst => EL3RST_STR,
            //ErrNo::LNRng => ELNRNG_STR,
            //ErrNo::Unatch => EUNATCH_STR,
            //ErrNo::NoCSI => ENOCSI_STR,
            //ErrNo::L2Hlt => EL2HLT_STR,
            //ErrNo::BadE => EBADE_STR,
            //ErrNo::BadR => EBADR_STR,
            //ErrNo::XFull => EXFULL_STR,
            //ErrNo::NoAno => ENOANO_STR,
            //ErrNo::BadRqC => EBADRQC_STR,
            //ErrNo::BadSlt => EBADSLT_STR,
            //ErrNo::BFont => EBFONT_STR,
            ErrNo::NoStr => ENOSTR_STR,
            ErrNo::NoData => ENODATA_STR,
            ErrNo::Time => ETIME_STR,
            ErrNo::NoSR => ENOSR_STR,
            //ErrNo::NoNet => ENONET_STR,
            //ErrNo::NoPkg => ENOPKG_STR,
            ErrNo::Remote => EREMOTE_STR,
            ErrNo::NoLink => ENOLINK_STR,
            //ErrNo::Adv => EADV_STR,
            //ErrNo::SrMnt => ESRMNT_STR,
            //ErrNo::Comm => ECOMM_STR,
            ErrNo::Proto => EPROTO_STR,
            ErrNo::MultiHop => EMULTIHOP_STR,
            //ErrNo::DotDot => EDOTDOT_STR,
            ErrNo::BadMsg => EBADMSG_STR,
            ErrNo::Overflow => EOVERFLOW_STR,
            //ErrNo::NotUniq => ENOTUNIQ_STR,
            //ErrNo::BadFD => EBADFD_STR,
            //ErrNo::RemChg => EREMCHG_STR,
            //ErrNo::LibAcc => ELIBACC_STR,
            //ErrNo::LibBad => ELIBBAD_STR,
            //ErrNo::LibScn => ELIBSCN_STR,
            //ErrNo::LibMax => ELIBMAX_STR,
            //ErrNo::LibExec => ELIBEXEC_STR,
            ErrNo::IlSeq => EILSEQ_STR,
            //ErrNo::Restart => ERESTART_STR,
            //ErrNo::StrPipe => ESTRPIPE_STR,
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
            //ErrNo::UClean => EUCLEAN_STR,
            //ErrNo::NotNam => ENOTNAM_STR,
            //ErrNo::NAvail => ENAVAIL_STR,
            //ErrNo::IsNam => EISNAM_STR,
            //ErrNo::RemoteIO => EREMOTEIO_STR,
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
            //ECHRNG_STR => Self::ChRng,
            //EL2NSYNC_STR => Self::L2NSync,
            //EL3HLT_STR => Self::L3Hlt,
            //EL3RST_STR => Self::L3Rst,
            //ELNRNG_STR => Self::LNRng,
            //EUNATCH_STR => Self::Unatch,
            //ENOCSI_STR => Self::NoCSI,
            //EL2HLT_STR => Self::L2Hlt,
            //EBADE_STR => Self::BadE,
            //EBADR_STR => Self::BadR,
            //EXFULL_STR => Self::XFull,
            //ENOANO_STR => Self::NoAno,
            //EBADRQC_STR => Self::BadRqC,
            //EBADSLT_STR => Self::BadSlt,
            //EBFONT_STR => Self::BFont,
            ENOSTR_STR => Self::NoStr,
            ENODATA_STR => Self::NoData,
            ETIME_STR => Self::Time,
            ENOSR_STR => Self::NoSR,
            //ENONET_STR => Self::NoNet,
            //ENOPKG_STR => Self::NoPkg,
            EREMOTE_STR => Self::Remote,
            ENOLINK_STR => Self::NoLink,
            //EADV_STR => Self::Adv,
            //ESRMNT_STR => Self::SrMnt,
            //ECOMM_STR => Self::Comm,
            EPROTO_STR => Self::Proto,
            EMULTIHOP_STR => Self::MultiHop,
            //EDOTDOT_STR => Self::DotDot,
            EBADMSG_STR => Self::BadMsg,
            EOVERFLOW_STR => Self::Overflow,
            //ENOTUNIQ_STR => Self::NotUniq,
            //EBADFD_STR => Self::BadFD,
            //EREMCHG_STR => Self::RemChg,
            //ELIBACC_STR => Self::LibAcc,
            //ELIBBAD_STR => Self::LibBad,
            //ELIBSCN_STR => Self::LibScn,
            //ELIBMAX_STR => Self::LibMax,
            //ELIBEXEC_STR => Self::LibExec,
            EILSEQ_STR => Self::IlSeq,
            //ERESTART_STR => Self::Restart,
            //ESTRPIPE_STR => Self::StrPipe,
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
            //EUCLEAN_STR => Self::UClean,
            //ENOTNAM_STR => Self::NotNam,
            //ENAVAIL_STR => Self::NAvail,
            //EISNAM_STR => Self::IsNam,
            //EREMOTEIO_STR => Self::RemoteIO,
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
