use std::sync::Once;

use libloading::Library;

static INIT_LIBC: Once = Once::new();
static mut LIBC: Option<Library> = None;

pub(crate) fn get_libc() -> Result<&'static Library, Box<dyn std::error::Error>> {
    unsafe {
        INIT_LIBC.call_once(|| {
            LIBC = Some(Library::new("libc.so.6").unwrap());
        });

        LIBC.as_ref().ok_or_else(|| "Failed to load library".into())
    }
}
