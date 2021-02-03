# rust-http-clients-smoke-test

To build with Address Sanitizer, run

```bash
env RUSTFLAGS="-Z sanitizer=address" cargo +nightly build --target x86_64-unknown-linux-gnu --release
```

To build with debug assertions and overflow checks, run

```bash
env RUSTFLAGS="-C debug-assertions -C overflow-checks" cargo build --release
```

To build with all of the above, run
```
env RUSTFLAGS="-Z sanitizer=address -C debug-assertions -C overflow-checks" cargo +nightly build --target x86_64-unknown-linux-gnu --release
```
