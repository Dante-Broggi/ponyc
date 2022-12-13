use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:1"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = libc::c_int;
}
pub use self::_int32_t_h::int32_t;
#[no_mangle]
#[c2rust::src_loc = "10:1"]
pub unsafe extern "C" fn ponyint_wnohang() -> int32_t {
    return 0x1 as libc::c_int;
}
