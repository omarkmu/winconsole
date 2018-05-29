# winconsole
This crate provides a wrapper for console-related functions in the Windows API.  
Check out the documentation [here](https://docs.rs/winconsole/0.4.0/x86_64-pc-windows-msvc/winconsole/).

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies]
winconsole = "0.4"
```
Then, add the following to your code:
```rust
extern crate winconsole;
```

There are two optional features: `input` and `serde`.  
* `input` - Includes input-related functions.
* `serde` - Support for [serde](https://serde.rs/).

Additional features must be added to `Cargo.toml`:
```toml
[dependencies.winconsole]
version = "0.4"
features = ["input", "serde"]
```