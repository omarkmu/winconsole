# winconsole
This crate provides a wrapper for console-related functions in the Windows API.

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies]
winconsole = "0.1"
```
Then, add the following to your code:
```rust
extern crate winconsole;
```
In order to use features related to input, the `input` feature is required.  
Add this to `Cargo.toml`:
```toml
[dependencies.winconsole]
version = "0.1"
features = ["input"]
```
