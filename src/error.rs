use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error reading from procfs
    ProcFs(io::Error),
    /// Error parsing procfs content
    ProcFsParse(String),
    /// Error loading dynamic library
    LibLoad(libloading::Error),
    /// Error accessing CPU information
    CpuId(String),
    /// Error accessing filesystem
    Fs(io::Error),
    /// General system call error
    Syscall(String),
    /// Generic error with message
    Other(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ProcFs(e) => Some(e),
            Error::LibLoad(e) => Some(e),
            Error::Fs(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProcFs(e) => write!(f, "procfs error: {}", e),
            Error::ProcFsParse(e) => write!(f, "procfs parse error: {}", e),
            Error::LibLoad(e) => write!(f, "library loading error: {}", e),
            Error::CpuId(e) => write!(f, "CPU ID error: {}", e),
            Error::Fs(e) => write!(f, "filesystem error: {}", e),
            Error::Syscall(e) => write!(f, "syscall error: {}", e),
            Error::Other(e) => write!(f, "{}", e),
        }
    }
}

// Convert regular io::Error to Fs variant
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Fs(err)
    }
}

impl From<libloading::Error> for Error {
    fn from(err: libloading::Error) -> Self {
        Error::LibLoad(err)
    }
}
