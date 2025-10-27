# argon2.wasm

WebAssembly port of Argon2

```bash
npm i @hazae41/argon2-wasm
```

```bash
deno install jsr:@hazae41/argon2-wasm
```

[**📦 NPM**](https://www.npmjs.com/package/@hazae41/argon2-wasm) • [**📦 JSR**](https://jsr.io/@hazae41/argon2-wasm)

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
import { argon2Wasm } from "@hazae41/argon2-wasm";

// Wait for WASM to load
await argon2Wasm.initBundled();

using pass = new argon2Wasm.Memory(crypto.getRandomValues(new Uint8Array(256)))
using salt = new argon2Wasm.Memory(crypto.getRandomValues(new Uint8Array(32)))

using deriver = new argon2Wasm.Argon2Deriver("argon2d", 19, 16384, 12, 2)
using derived = deriver.derive(pass, salt)

console.log(derived.bytes)
```

## Building

### Reproducible building

You can build the exact same bytecode using Docker

```bash
npm run compile
```

Then check that all the files are the same using `npm diff`

```bash
npm diff
```

If the output is empty then the bytecode is the same as the one I published on NPM.

### Automated checks

Each time I release a new version on GitHub, the GitHub's CI clones the GitHub repository, reproduces the build, and throws an error if the NPM release is different. If a version is present on NPM but not on GitHub, do not use it!
