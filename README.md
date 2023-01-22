
# A (fixed) demo showing how to use component model with Wasmtime

Nix (flake) users:
```bash
direnv allow
# or
nix develop
```

Others:
Make sure you have wasm32-unknown-unknown installed

## Running this demo 
```bash 
cargo build -p guest --target wasm32-unknown-unknown
cargo run -p host
```

## Details
For more details, please read the comments in the source code. 

Or you can refer to [blog post](https://blog.mediosz.club/2022/11/17/how-to-use-wit-bindgen/).
however its outdated.
