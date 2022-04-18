export let wasmAdapter: any = undefined;

import('blip-front-wasm')
  .then((a) => (wasmAdapter = a))
  .catch((e) => console.error('Error importing `blip-front-wasm`:', e));
