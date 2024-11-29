use std::sync::Once;

use libloading::Library;

static INIT: Once = Once::new();
static mut LIBC: Option<Library> = None;

pub(crate) fn get_lib(lib: &str) -> Result<&'static Library, Box<dyn std::error::Error>> {
    unsafe {
        INIT.call_once(|| {
            LIBC = Some(Library::new(lib).unwrap());
        });

        LIBC.as_ref().ok_or_else(|| "Failed to load library".into())
    }
}
