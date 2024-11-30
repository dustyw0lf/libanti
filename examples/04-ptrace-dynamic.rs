use std::ffi::{c_uint, c_void};

use libanti::debug::get_ptrace;

fn main() {
    let ptrace = get_ptrace().unwrap();

    if unsafe { ptrace(0 as *const c_uint, 0, 0 as *mut c_void, 0 as *mut c_void) } == -1 {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
