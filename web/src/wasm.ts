import init, * as wasm from 'em-wasm';

let initialized = false;

export async function ensureInit() {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

export { wasm };
