import wasm from "https://wasmbyexample.dev/examples/hello-world/demo/rust/pkg/hello_world_bg.wasm" with {
  type: "bytes",
};

const imports = {
  "./hello_world": { add },
};

const { instance } = await WebAssembly.instantiate(wasm, imports);

export function add(a, b) {
  return instance.exports.add(a, b);
}
