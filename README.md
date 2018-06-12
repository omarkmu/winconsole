# winconsole
This crate provides a wrapper for console-related functions in the Windows API.  
Check out the documentation [here](https://omarkmu.github.io/docs/winconsole/).

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies]
winconsole = "0.6"
```
Then, add the following to your code:
```rust
extern crate winconsole;
```

---

There are a few optional features:
* `input` - Includes input-related functions.
* `serde` - Support for [serde](https://serde.rs/).
* `window` - Includes window-related functions.

These features must be added to `Cargo.toml`:
```toml
[dependencies.winconsole]
version = "0.6"
features = ["input", "serde"]
```