use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unwind.h:5"]
pub mod unwind_h {
    #[c2rust::src_loc = "32:9"]
    pub type _Unwind_Reason_Code = libc::c_uint;
    #[c2rust::src_loc = "42:3"]
    pub const _URC_CONTINUE_UNWIND: _Unwind_Reason_Code = 8;
    #[c2rust::src_loc = "41:3"]
    pub const _URC_INSTALL_CONTEXT: _Unwind_Reason_Code = 7;
    #[c2rust::src_loc = "40:3"]
    pub const _URC_HANDLER_FOUND: _Unwind_Reason_Code = 6;
    #[c2rust::src_loc = "39:3"]
    pub const _URC_END_OF_STACK: _Unwind_Reason_Code = 5;
    #[c2rust::src_loc = "38:3"]
    pub const _URC_NORMAL_STOP: _Unwind_Reason_Code = 4;
    #[c2rust::src_loc = "37:3"]
    pub const _URC_FATAL_PHASE1_ERROR: _Unwind_Reason_Code = 3;
    #[c2rust::src_loc = "36:3"]
    pub const _URC_FATAL_PHASE2_ERROR: _Unwind_Reason_Code = 2;
    #[c2rust::src_loc = "35:3"]
    pub const _URC_FOREIGN_EXCEPTION_CAUGHT: _Unwind_Reason_Code = 1;
    #[c2rust::src_loc = "34:3"]
    pub const _URC_OK: _Unwind_Reason_Code = 0;
    #[c2rust::src_loc = "33:3"]
    pub const _URC_NO_REASON: _Unwind_Reason_Code = 0;
    #[c2rust::src_loc = "48:9"]
    pub type _Unwind_Action = libc::c_uint;
    #[c2rust::src_loc = "53:3"]
    pub const _UA_END_OF_STACK: _Unwind_Action = 16;
    #[c2rust::src_loc = "52:3"]
    pub const _UA_FORCE_UNWIND: _Unwind_Action = 8;
    #[c2rust::src_loc = "51:3"]
    pub const _UA_HANDLER_FRAME: _Unwind_Action = 4;
    #[c2rust::src_loc = "50:3"]
    pub const _UA_CLEANUP_PHASE: _Unwind_Action = 2;
    #[c2rust::src_loc = "49:3"]
    pub const _UA_SEARCH_PHASE: _Unwind_Action = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:8"]
    pub struct _Unwind_Exception {
        pub exception_class: u64,
        pub exception_cleanup:
            Option<unsafe extern "C" fn(_Unwind_Reason_Code, *mut _Unwind_Exception) -> ()>,
        pub private_1: libc::uintptr_t,
        pub private_2: libc::uintptr_t,
    }
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "56:16"]
        pub type _Unwind_Context;
        #[c2rust::src_loc = "169:1"]
        pub fn _Unwind_RaiseException(
            exception_object: *mut _Unwind_Exception,
        ) -> _Unwind_Reason_Code;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/lang/lsda.h:5"]
pub mod lsda_h {
    #[c2rust::src_loc = "16:1"]
    pub type exception_context_t = _Unwind_Context;
    use super::_uintptr_t_h::uintptr_t;
    use super::unwind_h::_Unwind_Context;
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_lsda_scan(context: *mut exception_context_t, lp: *mut libc::uintptr_t) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "131:7"]
        pub fn abort() -> !;
    }
}
pub use self::_uintptr_t_h::uintptr_t;
pub use self::lsda_h::{exception_context_t, ponyint_lsda_scan};
use self::stdlib_h::abort;
pub use self::unwind_h::{
    _Unwind_Action, _Unwind_Context, _Unwind_Exception, _Unwind_RaiseException,
    _Unwind_Reason_Code, _UA_CLEANUP_PHASE, _UA_END_OF_STACK, _UA_FORCE_UNWIND, _UA_HANDLER_FRAME,
    _UA_SEARCH_PHASE, _URC_CONTINUE_UNWIND, _URC_END_OF_STACK, _URC_FATAL_PHASE1_ERROR,
    _URC_FATAL_PHASE2_ERROR, _URC_FOREIGN_EXCEPTION_CAUGHT, _URC_HANDLER_FOUND,
    _URC_INSTALL_CONTEXT, _URC_NORMAL_STOP, _URC_NO_REASON, _URC_OK,
};
#[thread_local]
#[c2rust::src_loc = "19:53"]
static mut exception: _Unwind_Exception = _Unwind_Exception {
    exception_class: 0,
    exception_cleanup: None,
    private_1: 0,
    private_2: 0,
};
#[thread_local]
#[c2rust::src_loc = "20:38"]
static mut landing_pad: libc::uintptr_t = 0;
#[c2rust::src_loc = "22:1"]
unsafe extern "C" fn exception_cleanup(
    mut _reason: _Unwind_Reason_Code,
    mut _exception_0: *mut _Unwind_Exception,
) {
}
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn pony_error() {
    exception.exception_class = 0x506f6e7900000000 as libc::c_long as u64;
    exception.exception_cleanup = Some(
        exception_cleanup
            as unsafe extern "C" fn(_Unwind_Reason_Code, *mut _Unwind_Exception) -> (),
    );
    _Unwind_RaiseException(&mut exception);
    libc::abort();
}
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn set_registers(
    mut _exception_0: *mut _Unwind_Exception,
    mut _context: *mut _Unwind_Context,
) {
    libc::abort();
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn ponyint_personality_v0(
    mut version: libc::c_int,
    mut actions: _Unwind_Action,
    mut ex_class: u64,
    mut exception_0: *mut _Unwind_Exception,
    mut context: *mut _Unwind_Context,
) -> _Unwind_Reason_Code {
    if version != 1 as libc::c_int || exception_0.is_null() || context.is_null() {
        return _URC_FATAL_PHASE1_ERROR;
    }
    if ex_class != 0x506f6e7900000000 as libc::c_long as libc::c_ulonglong {
        return _URC_CONTINUE_UNWIND;
    }
    if actions as libc::c_uint & _UA_SEARCH_PHASE as libc::c_int as libc::c_uint != 0 {
        if !ponyint_lsda_scan(context, &mut landing_pad) {
            return _URC_CONTINUE_UNWIND;
        }
        return _URC_HANDLER_FOUND;
    }
    if actions as libc::c_uint & _UA_CLEANUP_PHASE as libc::c_int as libc::c_uint != 0 {
        if actions as libc::c_uint & _UA_HANDLER_FRAME as libc::c_int as libc::c_uint == 0 {
            return _URC_CONTINUE_UNWIND;
        }
        set_registers(exception_0, context);
        return _URC_INSTALL_CONTEXT;
    }
    _URC_FATAL_PHASE1_ERROR
}
