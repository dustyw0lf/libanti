use libanti::ptrace::syscall_ptrace;

fn main() {
    if unsafe { syscall_ptrace(0, 0, 0, 0) } == -1 {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
