# Manual Simulator

## 0) Prerequisites

- **Rust** (with `wasm32-unknown-unknown` target)
- **wasm-pack** (Rust → WASM bundler)
- **Bun** (JavaScript runtime; replaces Node here)

```bash
# Rust
curl https://sh.rustup.rs -sSf | sh
rustup target add wasm32-unknown-unknown

# wasm-pack
cargo install wasm-pack

# Bun
curl -fsSL https://bun.sh/install | bash
```
