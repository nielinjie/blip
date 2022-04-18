import * as wasm from 'blip-front-wasm';

export const compute = function (item:object):string {
  return wasm.compute(item);
};