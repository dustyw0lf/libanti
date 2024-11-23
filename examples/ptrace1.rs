use libc::{ptrace, PTRACE_TRACEME};

fn main() {
    if unsafe { ptrace(PTRACE_TRACEME, 0, 0, 0) } == -1 {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
