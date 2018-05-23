#![cfg(windows)]
#![warn(missing_docs)]
/*!
 This crate provides a wrapper for console-related functions in the Windows API.

 # Usage
 Add the following to `Cargo.toml`:
 ```toml
 [dependencies]
 winconsole = "0.2"
 ```
 Then, add the following to your code:
 ```rust
 extern crate winconsole;
 ```
 In order to use features related to input, the `input` feature is required.  
 Add this to `Cargo.toml`:
 ```toml
 [dependencies.winconsole]
 version = "0.2"
 features = ["input"]
 ```
 */
extern crate winapi;
extern crate cgmath;
#[macro_use] extern crate lazy_static;
extern crate rgb;

#[macro_use] mod macros;

/// Contains console-related structs and enums.
pub mod console;
mod etc;
/// Contains various error types.
pub mod errors;
/// Contains input-related structs and enums.
#[cfg(feature = "input")]
pub mod input;
