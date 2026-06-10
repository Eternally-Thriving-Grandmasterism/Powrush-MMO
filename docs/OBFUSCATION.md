# Powrush-MMO Client-Side Obfuscation Guide (v18.9)

This document describes the light, sovereignty-friendly obfuscation layer applied to the client.

## Philosophy

We use **light obfuscation** only. Heavy commercial obfuscators are avoided because they conflict with sovereignty and add unnecessary complexity.

The primary protection comes from:
- Server-authoritative simulation
- Behavioral + telemetry bot detection
- Rate limiting and validation

Obfuscation is a secondary layer to raise the bar for casual reverse engineering.

## Applied Techniques

### 1. Release Build Optimizations

The root `Cargo.toml` includes an optimized release profile:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

This removes debug symbols and applies aggressive optimizations, making reverse engineering significantly harder.

### 2. WASM Optimization Pipeline (Web Builds)

For WebAssembly builds, run the following after `wasm-pack` or `trunk build`:

```bash
# Install tools (once)
wasm-opt --version || cargo install wasm-opt
wasm-strip --version || cargo install wasm-strip

# Optimize and strip
wasm-opt -Oz --strip-debug --strip-producers \
    pkg/powrush_mmo_bg.wasm -o pkg/powrush_mmo_bg.wasm

wasm-strip pkg/powrush_mmo_bg.wasm
```

This significantly reduces binary size and removes a lot of metadata.

### 3. Lightweight String Obfuscation

We use the `obfstr` crate for sensitive strings (endpoints, keys, internal identifiers).

Example usage:

```rust
use obfstr::obfstr;

let endpoint = obfstr!("https://api.example.com/telemetry");
let secret = obfstr!("my-telemetry-key");
```

Strings are obfuscated at compile time and only decrypted at runtime when needed.

## How to Build with Obfuscation

```bash
# Native
cargo build --release

# Web
trunk build --release
# Then run the wasm-opt + wasm-strip commands above
```

## Disabling Obfuscation

If you want fully readable binaries (e.g. for sovereign debugging or modding), simply build with:

```bash
cargo build --profile dev
```

Or remove `strip = true` from the release profile.

## Limitations

Obfuscation is **not** a replacement for server-authoritative design and behavioral detection. Serious actors will still attempt to bypass client protections.

## Future Enhancements (Optional)

- More advanced string obfuscation
- Selective control-flow obfuscation on critical paths
- Runtime integrity checks (only if needed)
```
