# FIRST_HOUR_PLAYTEST.md

Run this on a clean machine. No Ra-Thor checkout.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
