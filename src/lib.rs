#![cfg(windows)]
#![warn(missing_docs)]
/*!
 This crate provides a wrapper for console-related functions in the Windows API.

 # Usage
 Add the following to `Cargo.toml`:
 ```toml
 [dependencies]
 winconsole = "0.4"
 ```
 Then, add the following to your code:

 ```rust
 extern crate winconsole;
 ```
 In order to use features related to input, the `input` feature is required.  
 Add this to `Cargo.toml`:
 ```toml
 [dependencies.winconsole]
 version = "0.4"
 features = ["input"]
 ```
 */
extern crate winapi;
extern crate cgmath;
#[macro_use] extern crate lazy_static;
extern crate rgb;

#[macro_use] mod macros;

/// Contains console-related functions, structs, and enums.
pub mod console;
/// Contains various error types.
pub mod errors;
/// Contains input-related functions, structs, and enums.
#[cfg(feature = "input")]
pub mod input;
