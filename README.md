# Linux Anti Debugging

This repo contains example code for anti debugging techniques.

## Examples
build an example using
```bash
cargo build --example <example>
```

Run an example with gdb
```bash
gdb -ex 'run' ./target/debug/examples/01-single-ptrace
```

Run an example with gdb while overwriting `ptrace` using shared object and `LD_PRELOAD`
```bash
gdb -ex 'set exec-wrapper env "LD_PRELOAD=./assets/deptrace.so"' -ex 'run' ./target/debug/examples/01-single-ptrace
```

## Acknowledgments
- [gdb vs. ptrace ...... fight!](https://sites.google.com/site/janbeck/cybersecurity-and-reverse-engineering-fun/gdb-vs-ptrace-fight) by Jan Bastian Beck.
- [Linux Anti Debugging](https://seblau.github.io/posts/linux-anti-debugging) by Sebastian Auberger.
- [Analysis of Anti-Analysis: Hiding Call To Ptrace](https://github.com/yellowbyte/analysis-of-anti-analysis/blob/develop/research/hiding_call_to_ptrace/hiding_call_to_ptrace.md) by Yu-Jye Tung.
- [Syscalls: Raw Linux system calls for Rust](https://github.com/jasonwhite/syscalls) by Jason White.