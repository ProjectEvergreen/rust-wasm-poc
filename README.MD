# rust-wasm-poc
A playground for things related to Rust, WASM and the web


# To build:
`wasm-pack build`
# To test:
`wasm-pack test`

------------------------------
When running `wasm-pack build`  creates:
rust_wasm_poc_bg.d.ts,
rust_wasm_poc_bg.wasm,
rust_wasm_poc.d.ts, and 
rust_wasm_poc.js

```
{
  "name": "rust-wasm-poc",
  "collaborators": [
    "aimeelynnramirez"
  ],
  "version": "0.1.0",
  "files": [
    "rust_wasm_poc_bg.wasm",
    "rust_wasm_poc.js",
    "rust_wasm_poc.d.ts"
  ],
  "module": "rust_wasm_poc.js",
  "types": "rust_wasm_poc.d.ts",
  "sideEffects": "false"
}
```