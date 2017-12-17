//! Image functionality.

use lib;

/// Represents a clear video image.
pub struct Image {
    pub(crate) img: *mut lib::vpx_image_t,
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
