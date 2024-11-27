use std::ffi::{c_int, c_long, c_uint, c_void};
use std::sync::Once;

use libloading::{Library, Symbol};

static INIT: Once = Once::new();
static mut LIBC: Option<Library> = None;

fn main() {
    let not_ptrace = get_not_ptrace().unwrap();

    if unsafe { not_ptrace(0 as *const c_uint, 0, 0 as *mut c_void, 0 as *mut c_void) } == -1 {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}

fn get_lib() -> Result<&'static Library, Box<dyn std::error::Error>> {
    unsafe {
        INIT.call_once(|| {
            LIBC = Some(Library::new("libc.so.6").unwrap());
        });

        LIBC.as_ref().ok_or_else(|| "Failed to load library".into())
    }
}

type NotPtraceFn = unsafe extern "C" fn(
    request: *const c_uint,
    pid: c_int,
    addr: *mut c_void,
    data: *mut c_void,
) -> c_long;

pub fn get_not_ptrace() -> Result<NotPtraceFn, Box<dyn std::error::Error>> {
    let lib = get_lib()?;
    unsafe {
        let not_ptrace: Symbol<NotPtraceFn> = lib.get(b"ptrace\0")?;
        Ok(*not_ptrace.into_raw())
    }
}
