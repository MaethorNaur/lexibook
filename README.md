# Lexibook

Current release: 0.2.3

## CLI

### Build

```bash
cargo build --bins --release
```

### Install

```bash
cargo install --force --path cli
```

## FFI

### Build

```bash
cargo build --lib --release
```

## WASM

### Build

```bash
cd wasm
wasm-pack build
cd public
yarn install
npm run start
```
