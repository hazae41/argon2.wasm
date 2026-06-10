# argon2.wasm

WebAssembly port of Argon2

```bash
npm install @hazae41/argon2-wasm
```

[**📦 NPM**](https://www.npmjs.com/package/@hazae41/argon2-wasm)

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
import { load, Memory, Argon2Deriver } from "@hazae41/argon2-wasm";

await load();

using pass = new Memory(crypto.getRandomValues(new Uint8Array(256)))
using salt = new Memory(crypto.getRandomValues(new Uint8Array(32)))

using deriver = new Argon2Deriver("argon2d", 19, 16384, 12, 2)
using derived = deriver.derive(pass, salt)

console.log(derived.bytes)
```

## Building

### Reproducible building

You can build the exact same bytecode

```bash
npm run compile && npm run prepack
```

Then check that all the files are the same using `npm diff`

```bash
npm diff
```

If the output is empty then the bytecode is the same as the one I published on NPM.

### Automated checks

Each time I release a new version on GitHub, the GitHub's CI clones the GitHub repository, reproduces the build, and throws an error if the NPM release is different. If a version is present on NPM but not on GitHub, do not use it!
