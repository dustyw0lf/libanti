use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error accessing the filsystem
    Io(io::Error),
    /// Error parsing procfs content
    ProcFsParse(String),
    /// Error accessing CPU information
    CpuId(CpuIdError),
    /// Error resolving functions at runtime
    Init(InitError),
}

#[derive(Debug)]
pub enum CpuIdError {
    MissingBrandString,
}

impl fmt::Display for CpuIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuIdError::MissingBrandString => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
pub enum InitError {
    /// Failure to resolve function
    FunctionResolution(&'static str),
    /// Failure to initialize library
    LibraryInit(&'static str),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitError::FunctionResolution(func) => {
                write!(f, "failed to resolve function: {}", func)
            }
            InitError::LibraryInit(lib) => write!(f, "failed to initialize library: {}", lib),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::ProcFsParse(e) => write!(f, "{}", e),
            Error::CpuId(e) => write!(f, "{}", e),
            Error::Init(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
