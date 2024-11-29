use std::{env, fs};

pub fn is_preloaded_procfs() -> Result<bool, Box<dyn std::error::Error>> {
    let exe = env::current_exe()?.display().to_string();
    let exe = exe
        .split("/")
        .last()
        .ok_or_else(|| "Failed to get binary name")?;

    // allowed names
    let maps = [
        exe,
        "linux-vdso.so.1",
        "libgcc_s.so.1",
        "libc.so.6",
        "ld-linux-x86-64.so.2",
        "[heap]",
        "[stack]",
        "[vvar]",
        "[vdso]",
    ];

    for line in fs::read_to_string("/proc/self/maps")?.lines() {
        // Get name from the map
        let name = match line.split_whitespace().nth(5) {
            Some(res) => res,
            // Skip lines that don't have a 6th element
            None => continue,
        };

        let name = name
            .split("/")
            .last()
            .ok_or_else(|| format!("Error splitting map: {}", line))?;

        if !maps.contains(&name) {
            // println!("{}", name);
            return Ok(true);
        };
    }

    Ok(false)
}
