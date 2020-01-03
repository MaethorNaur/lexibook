# Lexibook

## CLI

### Build

```bash
cargo build --bins --release
```

### Install

```bash
cargo install --bins --path .
```

## FFI

### Build

```bash
cargo build --lib --release
```

## WASM

### Build

```bash
wasm-pack build -- --features wasm --no-default-features
cd example
yarn install
npm run start
```
