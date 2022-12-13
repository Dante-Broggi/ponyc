use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:1"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut libc::c_int;
    }
}
use self::errno_h::__error;
#[no_mangle]
#[c2rust::src_loc = "6:1"]
pub unsafe extern "C" fn pony_os_clear_errno() {
    *__error() = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "11:1"]
pub unsafe extern "C" fn pony_os_errno() -> libc::c_int {
    return *__error();
}
