//! # vpx
//! Rust wrapper for the libvpx (VP8/VP9) codec library.

#[macro_use]
extern crate error_chain;

extern crate vpx_sys as lib;

pub mod error;
pub mod decoder;

pub use decoder::Decoder;

/// Represents a clear video image.
pub struct Image {
    img: *mut lib::vpx_image_t,
}

impl Image {
    /// Return the width of the image in pixels.
    pub fn width(&self) -> u32 {
        unsafe {
            (*self.img).w as u32
        }
    }

    /// Return the height of the image in pixels.
    pub fn height(&self) -> u32 {
        unsafe {
            (*self.img).h as u32
        }
    }
}
