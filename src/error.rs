use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error accessing the filsystem
    Io(io::Error),
    /// Error parsing procfs content
    ProcFsParse(String),
    /// Error loading dynamic library
    LibLoad(libloading::Error),
    /// Error accessing CPU information
    CpuId(String),
    /// General system call error
    Syscall(String),
    /// Generic error with message
    Other(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::LibLoad(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "filesystem error: {}", e),
            Error::ProcFsParse(e) => write!(f, "procfs parse error: {}", e),
            Error::LibLoad(e) => write!(f, "library loading error: {}", e),
            Error::CpuId(e) => write!(f, "CPU ID error: {}", e),
            Error::Syscall(e) => write!(f, "syscall error: {}", e),
            Error::Other(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<libloading::Error> for Error {
    fn from(err: libloading::Error) -> Self {
        Error::LibLoad(err)
    }
}
