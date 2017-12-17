//! Error handling functionality.

use std::ptr;
use std::ffi::CStr;

use lib;

error_chain! {
    errors {
        Vpx(msg: &'static str) {
            description("an error occurred within libvpx"),
            display("libvpx: {}", msg),
        }
    }
}

pub unsafe fn ok(err: lib::vpx_codec_err_t) -> bool {
    err == lib::vpx_codec_err_t::VPX_CODEC_OK
}

pub unsafe fn libvpx(err: lib::vpx_codec_err_t) -> Error {
    let ptr = lib::vpx_codec_err_to_string(err);

    if ptr == ptr::null() {
        return Error::from(ErrorKind::Vpx("unknown error"));
    }

    let msg = CStr::from_ptr(ptr).to_str().unwrap_or("invalid error message (not utf8)");
    Error::from(ErrorKind::Vpx(msg))
}
