import { add } from "https://wasmbyexample.dev/examples/hello-world/demo/rust/pkg/hello_world_bg.wasm";

const result = add(24, 24);

console.log(`It works! ${result}`);
