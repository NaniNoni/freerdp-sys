# freerdp-sys

Rust FFI bindings to the [freerdp3 library](https://github.com/FreeRDP/FreeRDP).

## Building
`freerdp-sys` requires you to have freerdp3 installed on your system (only for building).
You can install it on Ubuntu 24.04 like so:
```bash
sudo apt install freerdp3-dev
```

## Roadmap
1. Add ability to build from source using the `cmake` crate.
2. Add ability to specify build type using cargo features.
3. Add test suite
