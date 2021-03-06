//! VPX Decoder functionality.

use std::ptr;
use std::mem;

use lib;
use super::Codec;
use image::Image;
use error::{self, Result};

/// Decoder object (context).
pub struct Decoder {
    ctx: lib::vpx_codec_ctx_t,
}

impl Decoder {
    /// Create a new VPX Decoder.
    pub fn new(codec: Codec) -> Result<Decoder> {
        unsafe {
            let mut ctx: lib::vpx_codec_ctx_t = mem::uninitialized();

            let iface = match codec {
                Codec::VP8 => lib::vpx_codec_vp8_dx(),
                Codec::VP9 => lib::vpx_codec_vp9_dx(),
            };

            let err = lib::vpx_codec_dec_init_ver(
                &mut ctx,
                iface,
                ptr::null_mut(),
                0,
                lib::VPX_DECODER_ABI_VERSION as i32
            );

            if !error::ok(err) {
                bail!(error::libvpx(err));
            }

            Ok(Decoder {
                ctx: ctx,
            })
        }
    }

    /// Decode the specified VPX data. Returns an iterator over the available frames that have been
    /// decoded.
    pub fn decode(&mut self, data: Vec<u8>) -> Result<Images> {
        unsafe {
            let ptr = data.as_ptr();
            let size = data.len() as u32;
            let err = lib::vpx_codec_decode(&mut self.ctx, ptr, size, ptr::null_mut(), 0);

            if !error::ok(err) {
                bail!(error::libvpx(err));
            }

            let frames = Images {
                dec: self,
                iter: ptr::null_mut(),
            };

            Ok(frames)
        }
    }
}

pub struct Images<'a> {
    dec: &'a mut Decoder,
    iter: lib::vpx_codec_iter_t,
}

impl<'a> ::std::iter::Iterator for Images<'a> {
    type Item = Image;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let img = lib::vpx_codec_get_frame(&mut self.dec.ctx, &mut self.iter);

            if img == ptr::null_mut() {
                None
            } else {
                Some(Image {
                    img: img,
                })
            }
        }
    }
}
