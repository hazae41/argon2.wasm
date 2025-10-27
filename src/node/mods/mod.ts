export * from "../../wasm/pkg/argon2_wasm.js";

/* @ts-types="../../wasm/pkg/argon2_wasm.d.ts" */
import init, { type InitOutput } from "../../wasm/pkg/argon2_wasm.js";
import { data } from "../../wasm/pkg/argon2_wasm.wasm.js";

export async function initBundled(): Promise<InitOutput> {
  return await init({ module_or_path: data })
}
