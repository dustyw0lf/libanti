use core::arch::asm;
use std::ffi::{c_int, c_long, c_uint, c_void};
use std::fs;
use std::sync::{Once, OnceLock};

use libloading::Symbol;

use crate::error::{Error, Result};
use crate::utils::get_libc;

pub fn is_traced() -> Result<bool> {
    let status = fs::read_to_string("/proc/self/status").unwrap();

    for line in status.lines() {
        if line.contains("TracerPid") {
            let status = line
                .split_whitespace()
                .last()
                .ok_or_else(|| Error::ProcFsParse("missing TracerPid value".to_string()))?
                .parse::<isize>()
                .map_err(|_| Error::ProcFsParse("invalid TracerPid value".to_string()))?;
            if status != 0 {
                return Ok(true);
            }
        };
    }

    Ok(false)
}

type PtraceFn = unsafe extern "C" fn(
    request: *const c_uint,
    pid: c_int,
    addr: *mut c_void,
    data: *mut c_void,
) -> c_long;

static PTRACE_INIT: Once = Once::new();
static mut PTRACE: Option<PtraceFn> = None;
static PTRACE_INIT_ERROR: OnceLock<String> = OnceLock::new();

/// Detects if a debugger is present by dynamically resolving and calling ptrace.
///
/// Returns `Ok(true)` if a debugger is detected, `Ok(false)` if no debugger is present,
/// and `Err` if ptrace resolution fails.
pub fn is_ptraced_dynamic() -> Result<bool> {
    let lib = get_libc()?;

    let ptrace = unsafe {
        PTRACE_INIT.call_once(|| match lib.get::<Symbol<PtraceFn>>(b"ptrace\0") {
            // Double dereference:
            // first * get &PtraceFn from Symbol<PtraceFn>
            // second * gets the actual function pointer from &PtraceFn
            Ok(sym) => PTRACE = Some(**sym),
            Err(e) => {
                let _ = PTRACE_INIT_ERROR.set(format!("failed to resolve ptrace: {}", e));
            }
        });

        if let Some(err) = PTRACE_INIT_ERROR.get() {
            return Err(Error::Other(err.clone()));
        }

        PTRACE.ok_or_else(|| Error::Other("failed to initialize ptrace".to_string()))?
    };

    let res = unsafe { ptrace(0 as *const c_uint, 0, 0 as *mut c_void, 0 as *mut c_void) };

    // If the process was already being traced, return true
    // If the process wasn't already being traced, return false
    Ok(res != 0)
}

unsafe fn syscall_ptrace(request: usize, pid: usize, addr: usize, data: usize) -> isize {
    // Based on
    // https://github.com/jasonwhite/syscalls/blob/main/src/syscall/x86_64.rs
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") 101 as usize => ret, // ptrace syscall number
        in("rdi") request,                    // arg 1
        in("rsi") pid,                        // arg 2
        in("rdx") addr,                       // arg 3
        in("r10") data,                       // arg 4
        out("rcx") _,                         // rcx is used to store old rip
        out("r11") _,                         // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret as isize
}

/// Detects if a debugger is present by calling the ptrace syscall directly.
///
/// Returns `Ok(true)` if a debugger is detected and `Ok(false)` if no debugger is present.
/// The function is currently infallible, but returns `Result` for consistency with other functions.
pub fn is_ptraced_syscall() -> Result<bool> {
    let res = unsafe { syscall_ptrace(0, 0, 0, 0) };

    // If the process was already being traced, return true
    // If the process wasn't already being traced, return false
    Ok(res != 0)
}
