#![cfg(windows)]
#![warn(missing_docs)]
/*!
 This crate provides a wrapper for console-related functions in the Windows API.

 # Usage
 Add the following to `Cargo.toml`:
 ```toml
 [dependencies]
 winconsole = "0.5"
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
 version = "0.5"
 features = ["input", "serde"] 
 ```
 */
extern crate winapi;
extern crate cgmath;
#[macro_use]
extern crate lazy_static;
extern crate rgb;
#[cfg(feature = "serde")] #[macro_use]
extern crate serde;

#[macro_use]
mod macros;

/// Contains console-related functions, structs, and enums.
pub mod console;
/// Contains various error types.
pub mod errors;
/// Contains input-related functions, structs, and enums.
#[cfg(feature = "input")]
pub mod input;
