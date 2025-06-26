import { test } from "@hazae41/phobos";
import { Argon2Deriver, initBundled, Memory } from "./index.js";

function equals(a: Uint8Array, b: Uint8Array) {
  return Buffer.from(a).equals(Buffer.from(b))
}

test("argon2", async () => {
  await initBundled()

  using pass = new Memory(crypto.getRandomValues(new Uint8Array(256)))
  using salt = new Memory(crypto.getRandomValues(new Uint8Array(32)))

  using deriver = new Argon2Deriver("argon2d", 19, 16384, 12, 2)
  using derived = deriver.derive(pass, salt)

  console.log("derived", derived.bytes)
})