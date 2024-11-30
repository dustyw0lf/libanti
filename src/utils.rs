use crate::error::{Error, Result};
use libloading::Library;
use std::sync::{Once, OnceLock};

static LIBC_INIT: Once = Once::new();
static mut LIBC: Option<Library> = None;
static LIBC_INIT_ERROR: OnceLock<String> = OnceLock::new();

pub(crate) fn get_libc() -> Result<&'static Library> {
    unsafe {
        LIBC_INIT.call_once(|| match Library::new("libc.so.6") {
            Ok(lib) => LIBC = Some(lib),
            Err(e) => {
                let _ = LIBC_INIT_ERROR.set(e.to_string());
            }
        });

        if let Some(err) = LIBC_INIT_ERROR.get() {
            return Err(Error::Other(err.clone()));
        }

        LIBC.as_ref()
            .ok_or_else(|| Error::Other("failed to initialize libc".to_string()))
    }
}
