use linux_antidbg::virt::{is_virt_cpu, is_virt_disk};

fn main() {
    if is_virt_cpu().unwrap() {
        println!("CPU: Virtual machine");
    } else {
        println!("CPU: Normal");
    }

    if is_virt_disk().unwrap() {
        println!("Disk: Virtual machine");
    } else {
        println!("Disk: Normal");
    }
}
