# Linux Anti Reversing

This library contains functions to detect attempts to tamper with a running process. Currently implemented without obfuscation:
1. Debugger detection:
    - Directly invoking [ptrace](https://man7.org/linux/man-pages/man2/ptrace.2.html) as a system call, and as such not calling it from libc.
    - Checking the `TracerPid` field in [/proc/self/status](https://man7.org/linux/man-pages/man5/proc_pid_status.5.html).
2. Preloaded libraries detection:
    - Checking [/proc/self/maps](https://man7.org/linux/man-pages/man5/proc_pid_maps.5.html) for unusual shared objects.
3. Virtual machine detection:
    - Checking the `cpuid` brand string for unusual CPUs.
    - Checking the `/dev/disk/by-path` directory for virtual disks. 


## Examples
build an example in either debug or release mode
```bash
cargo build [--release] --example <example>
```

Run an example with gdb. For a release build, change `debug` to `release`
```bash
gdb -ex 'run' ./target/debug/examples/<example>
```

Run an example with gdb while overwriting the `ptrace` wrapper function using a shared object and `LD_PRELOAD`
```bash
gdb -ex 'set exec-wrapper env "LD_PRELOAD=./assets/deptrace.so"' -ex 'run' ./target/debug/examples/<example>
```

## Acknowledgments
- [gdb vs. ptrace ...... fight!](https://sites.google.com/site/janbeck/cybersecurity-and-reverse-engineering-fun/gdb-vs-ptrace-fight) by Jan Bastian Beck.
- [Linux Anti Debugging](https://seblau.github.io/posts/linux-anti-debugging) by Sebastian Auberger.
- [Analysis of Anti-Analysis: Hiding Call To Ptrace](https://github.com/yellowbyte/analysis-of-anti-analysis/blob/develop/research/hiding_call_to_ptrace/hiding_call_to_ptrace.md) by Yu-Jye Tung.
- [Syscalls: Raw Linux system calls for Rust](https://github.com/jasonwhite/syscalls) by Jason White.
- [Anti Debugging For Noobs](https://web.archive.org/web/20201205023553/https://adhokshajmishraonline.in/anti-debugging-for-noobs.html) by Adhokshaj mishra.