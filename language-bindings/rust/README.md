WAMR Rust binding: Embedding WAMR in Rust guideline
===============================================

This Rust libray use [rust-bindgen](https://rust-lang.github.io/rust-bindgen/) to genrate rust bindings by consuming the runtime APIs of the WAMR project which are defined in [core/iwasm/include/wasm_export.h](../../core/iwasm/include/wasm_export.h). The API details are available in the header files.

## Installation
Install Rust toolchain following the directions from https://www.rust-lang.org/tools/install.


### Installing from the source code

1. build C++ iwasm library according to [Build WAMR vmcore (iwasm)](../../doc/build_wamr.md)
2. `cargo build` in this folder to build the package, which builds the WAMR runtime library firstly and then builds the Rust binding library.

## Supported APIs

All the embedding APIs supported are defined under folder [wamr](./wamr).

### Runtime APIs
Checkout https://docs.rs/wamr/latest

## Sample codes

**TBD**

```Rust
```

More samples can be found in [test.go](./samples/test.rs)
