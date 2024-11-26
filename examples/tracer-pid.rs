use std::fs;

fn main() {
    if is_tracer_attached().unwrap() {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}

fn is_tracer_attached() -> Result<bool, Box<dyn std::error::Error>> {
    let status = fs::read_to_string("/proc/self/status").unwrap();

    for line in status.lines() {
        if line.contains("TracerPid") {
            let status = line
                .split_whitespace()
                .last()
                .ok_or_else(|| "Error getting tracer pid")?
                .parse::<isize>()?;
            if status != 0 {
                return Ok(true);
            }
        };
    }

    Ok(false)
}
