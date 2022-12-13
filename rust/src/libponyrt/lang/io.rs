use ::libc;
#[no_mangle]
#[c2rust::src_loc = "12:1"]
pub unsafe extern "C" fn pony_os_writev_max() -> libc::c_int {
    return 1024 as libc::c_int;
}
