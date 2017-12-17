//! # vpx
//! Rust wrapper for the libvpx (VP8/VP9) codec library.

#[macro_use]
extern crate error_chain;

extern crate vpx_sys as lib;

pub mod error;
pub mod image;
pub mod decoder;

pub use decoder::Decoder;
