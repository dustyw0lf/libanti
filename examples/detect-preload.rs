use linux_antidbg::detect::preload::is_preloaded_procfs;

fn main() {
    if is_preloaded_procfs().unwrap() {
        println!("Preloaded");
    } else {
        println!("Normal");
    }
}
