use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    #[c2rust::src_loc = "9:16"]
    pub use crate::libponyc::ast::ast::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/frame.h:1"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct typecheck_frame_t {
        pub package: *mut ast_t,
        pub module: *mut ast_t,
        pub type_0: *mut ast_t,
        pub constraint: *mut ast_t,
        pub provides: *mut ast_t,
        pub method: *mut ast_t,
        pub def_arg: *mut ast_t,
        pub method_body: *mut ast_t,
        pub method_params: *mut ast_t,
        pub method_type: *mut ast_t,
        pub ffi_type: *mut ast_t,
        pub local_type: *mut ast_t,
        pub as_type: *mut ast_t,
        pub the_case: *mut ast_t,
        pub pattern: *mut ast_t,
        pub loop_0: *mut ast_t,
        pub loop_cond: *mut ast_t,
        pub loop_body: *mut ast_t,
        pub loop_else: *mut ast_t,
        pub try_expr: *mut ast_t,
        pub recover: *mut ast_t,
        pub ifdef_cond: *mut ast_t,
        pub ifdef_clause: *mut ast_t,
        pub iftype_constraint: *mut ast_t,
        pub iftype_body: *mut ast_t,
        pub prev: *mut typecheck_frame_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct typecheck_stats_t {
        pub names_count: usize,
        pub default_caps_count: usize,
        pub heap_alloc: usize,
        pub stack_alloc: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct typecheck_t {
        pub frame: *mut typecheck_frame_t,
        pub stats: typecheck_stats_t,
        pub errors: *mut errors_t,
    }

    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.h:1"]
pub mod pass_h {
    #[c2rust::src_loc = "205:9"]
    pub type verbosity_level = libc::c_uint;
    #[c2rust::src_loc = "211:3"]
    pub const VERBOSITY_ALL: verbosity_level = 4;
    #[c2rust::src_loc = "210:3"]
    pub const VERBOSITY_TOOL_INFO: verbosity_level = 3;
    #[c2rust::src_loc = "209:3"]
    pub const VERBOSITY_INFO: verbosity_level = 2;
    #[c2rust::src_loc = "208:3"]
    pub const VERBOSITY_MINIMAL: verbosity_level = 1;
    #[c2rust::src_loc = "207:3"]
    pub const VERBOSITY_QUIET: verbosity_level = 0;
    #[c2rust::src_loc = "216:9"]
    pub type pass_id = libc::c_uint;
    #[c2rust::src_loc = "239:3"]
    pub const PASS_ALL: pass_id = 21;
    #[c2rust::src_loc = "238:3"]
    pub const PASS_OBJ: pass_id = 20;
    #[c2rust::src_loc = "237:3"]
    pub const PASS_ASM: pass_id = 19;
    #[c2rust::src_loc = "236:3"]
    pub const PASS_BITCODE: pass_id = 18;
    #[c2rust::src_loc = "235:3"]
    pub const PASS_LLVM_IR: pass_id = 17;
    #[c2rust::src_loc = "234:3"]
    pub const PASS_PAINT: pass_id = 16;
    #[c2rust::src_loc = "233:3"]
    pub const PASS_REACH: pass_id = 15;
    #[c2rust::src_loc = "232:3"]
    pub const PASS_SERIALISER: pass_id = 14;
    #[c2rust::src_loc = "231:3"]
    pub const PASS_FINALISER: pass_id = 13;
    #[c2rust::src_loc = "230:3"]
    pub const PASS_VERIFY: pass_id = 12;
    #[c2rust::src_loc = "229:3"]
    pub const PASS_COMPLETENESS: pass_id = 11;
    #[c2rust::src_loc = "228:3"]
    pub const PASS_EXPR: pass_id = 10;
    #[c2rust::src_loc = "227:3"]
    pub const PASS_REFER: pass_id = 9;
    #[c2rust::src_loc = "226:3"]
    pub const PASS_DOCS: pass_id = 8;
    #[c2rust::src_loc = "225:3"]
    pub const PASS_TRAITS: pass_id = 7;
    #[c2rust::src_loc = "224:3"]
    pub const PASS_FLATTEN: pass_id = 6;
    #[c2rust::src_loc = "223:3"]
    pub const PASS_NAME_RESOLUTION: pass_id = 5;
    #[c2rust::src_loc = "222:3"]
    pub const PASS_IMPORT: pass_id = 4;
    #[c2rust::src_loc = "221:3"]
    pub const PASS_SCOPE: pass_id = 3;
    #[c2rust::src_loc = "220:3"]
    pub const PASS_SUGAR: pass_id = 2;
    #[c2rust::src_loc = "219:3"]
    pub const PASS_SYNTAX: pass_id = 1;
    #[c2rust::src_loc = "218:3"]
    pub const PASS_PARSE: pass_id = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "274:16"]
    pub struct pass_opt_t {
        pub limit: pass_id,
        pub program_pass: pass_id,
        pub release: bool,
        pub library: bool,
        pub runtimebc: bool,
        pub staticbin: bool,
        pub pic: bool,
        pub print_stats: bool,
        pub verify: bool,
        pub extfun: bool,
        pub strip_debug: bool,
        pub print_filenames: bool,
        pub check_tree: bool,
        pub lint_llvm: bool,
        pub docs: bool,
        pub docs_private: bool,
        pub verbosity: verbosity_level,
        pub ast_print_width: usize,
        pub allow_test_symbols: bool,
        pub parse_trace: bool,
        pub package_search_paths: *mut strlist_t,
        pub safe_packages: *mut strlist_t,
        pub magic_packages: *mut magic_package_t,
        pub argv0: *const libc::c_char,
        pub all_args: *const libc::c_char,
        pub output: *const libc::c_char,
        pub bin_name: *const libc::c_char,
        pub link_arch: *mut libc::c_char,
        pub linker: *mut libc::c_char,
        pub link_ldcmd: *mut libc::c_char,
        pub llvm_args: *const libc::c_char,
        pub triple: *mut libc::c_char,
        pub abi: *mut libc::c_char,
        pub cpu: *mut libc::c_char,
        pub features: *mut libc::c_char,
        pub check: typecheck_t,
        pub plugins: *mut plugins_t,
        pub data: *mut libc::c_void,
    }

    use super::frame_h::typecheck_t;
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:2"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn target_is_bsd(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn target_is_freebsd(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn target_is_dragonfly(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "14:1"]
        pub fn target_is_openbsd(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "10:1"]
        pub fn target_is_linux(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "15:1"]
        pub fn target_is_macosx(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn target_is_windows(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "17:1"]
        pub fn target_is_posix(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn target_is_x86(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "19:1"]
        pub fn target_is_arm(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn target_is_lp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn target_is_llp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "26:1"]
        pub fn target_is_native128(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "27:1"]
        pub fn target_is_bigendian(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "28:1"]
        pub fn target_is_littleendian(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:3"]
pub mod ponyassert_h {

    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: usize,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:4"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::genopt_h::{
    target_is_arm, target_is_bigendian, target_is_bsd, target_is_dragonfly, target_is_freebsd,
    target_is_ilp32, target_is_linux, target_is_littleendian, target_is_llp64, target_is_lp64,
    target_is_macosx, target_is_native128, target_is_openbsd, target_is_posix, target_is_windows,
    target_is_x86,
};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::string_h::strcmp;

#[no_mangle]
#[c2rust::src_loc = "8:1"]
pub unsafe extern "C" fn os_is_target(
    mut attribute: *const libc::c_char,
    mut release: bool,
    mut out_is_target: *mut bool,
    mut options: *mut pass_opt_t,
) -> bool {
    if !attribute.is_null() {
    } else {
        ponyint_assert_fail(
            b"attribute != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/platformfuns.c\0"
                as *const u8 as *const libc::c_char,
            10 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"os_is_target\0")).as_ptr(),
        );
    };
    if !out_is_target.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_is_target != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/platformfuns.c\0"
                as *const u8 as *const libc::c_char,
            11 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"os_is_target\0")).as_ptr(),
        );
    };
    if !options.is_null() {
    } else {
        ponyint_assert_fail(
            b"options != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/platformfuns.c\0"
                as *const u8 as *const libc::c_char,
            12 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"os_is_target\0")).as_ptr(),
        );
    };
    if strcmp(attribute, b"bsd\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_bsd((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"freebsd\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_freebsd((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"dragonfly\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = target_is_dragonfly((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"openbsd\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_openbsd((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"linux\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_linux((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"osx\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_macosx((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"windows\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_windows((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"posix\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_posix((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"x86\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_x86((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"arm\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_arm((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"lp64\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_lp64((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"llp64\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_llp64((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"ilp32\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = target_is_ilp32((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"native128\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = target_is_native128((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(attribute, b"debug\0" as *const u8 as *const libc::c_char) == 0 {
        *out_is_target = !release;
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"bigendian\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = target_is_bigendian((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"littleendian\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = target_is_littleendian((*options).triple);
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"runtimestats\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = 0 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    if strcmp(
        attribute,
        b"runtimestatsmessages\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        *out_is_target = 0 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
