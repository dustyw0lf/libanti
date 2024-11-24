# Linux Anti Debugging

This repo contains example code for anti debugging techniques.

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