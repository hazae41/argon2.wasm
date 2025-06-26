# argon2.wasm

WebAssembly port of Argon2

```bash
npm i @hazae41/argon2.wasm
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/argon2.wasm)

## Features
- Reproducible building
- Pre-bundled and streamed
- Zero-copy memory slices

## Modules
- argon2

## Algorithms
- Argon2

## Usage

```typescript
import { Argon2, Argon2Deriver, Memory } from "@hazae41/argon2.wasm";

// Wait for WASM to load
await Argon2.initBundled();

using pass = new Memory(crypto.getRandomValues(new Uint8Array(256)))
using salt = new Memory(crypto.getRandomValues(new Uint8Array(32)))

using deriver = new Argon2Deriver("argon2d", 19, 16384, 12, 2)
using derived = deriver.derive(pass, salt)

console.log(derived.bytes)
```

## Building

### Unreproducible building

You need to install [Rust](https://www.rust-lang.org/tools/install)

Then, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
cargo install wasm-pack
```

Finally, do a clean install and build

```bash
npm ci && npm run build
```

### Reproducible building

You can build the exact same bytecode using Docker, just be sure you're on a `linux/amd64` host

```bash
docker compose up --build
```

Then check that all the files are the same using `npm diff`

```bash
npm diff
```

If the output is empty then the bytecode is the same as the one I commited

### Automated checks

Each time I release a new version on GitHub, the GitHub's CI clones the GitHub repository, reproduces the build, and throws an error if the NPM release is different. If a version is present on NPM but not on GitHub, do not use it!
