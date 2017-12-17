//! VPX Decoder functionality.

use std::ptr;
use std::mem;

use lib;
use error::{self, Result};

/// Decoder object (context).
pub struct Decoder {
    ctx: lib::vpx_codec_ctx_t,
}

impl Decoder {
    /// Create a new VPX Decoder.
    pub fn new() -> Result<Decoder> {
        unsafe {
            let mut ctx: lib::vpx_codec_ctx_t = mem::uninitialized();

            let err = lib::vpx_codec_dec_init_ver(
                &mut ctx,
                lib::vpx_codec_vp8_dx(),
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
}
