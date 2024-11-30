use std::sync::{Once, OnceLock};

use libloading::Library;

use crate::error::{Error, InitError, Result};

static LIBC_INIT: Once = Once::new();
static mut LIBC: Option<Library> = None;
static LIBC_INIT_ERROR: OnceLock<String> = OnceLock::new();

pub(crate) fn get_libc() -> Result<&'static Library> {
    let lib = "libc.so.6";
    unsafe {
        LIBC_INIT.call_once(|| match Library::new(lib) {
            Ok(lib) => LIBC = Some(lib),
            Err(err) => {
                let _ = LIBC_INIT_ERROR.set(err.to_string());
            }
        });

        if let Some(err) = LIBC_INIT_ERROR.get() {
            return Err(Error::Init(InitError::LibraryInit(err)));
        }

        LIBC.as_ref()
            .ok_or_else(|| Error::Init(InitError::LibraryInit(lib)))
    }
}
