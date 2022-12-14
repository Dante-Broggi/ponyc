use ::libc;
#[no_mangle]
#[c2rust::src_loc = "10:1"]
pub unsafe extern "C" fn ponyint_wnohang() -> i32 {
    return 0x1 as libc::c_int;
}
