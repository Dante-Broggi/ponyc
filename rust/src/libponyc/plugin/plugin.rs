use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "102:1"]
        pub fn errorf(
            errors: *mut errors_t,
            file: *const libc::c_char,
            fmt: *const libc::c_char,
            _: ...
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "15:1"]
    pub type map_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/list.h:1"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct list_t {
        pub data: *mut libc::c_void,
        pub next: *mut list_t,
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::fun_h::{cmp_fn, free_fn, map_fn};
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_list_pop(list: *mut list_t, data: *mut *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_list_push(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_list_append(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_list_next(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_list_index(list: *mut list_t, index: ssize_t) -> *mut list_t;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_list_data(list: *mut list_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_list_find(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "32:1"]
        pub fn ponyint_list_findindex(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> ssize_t;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_list_subset(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_list_equals(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_list_map(
            list: *mut list_t,
            f: map_fn,
            arg: *mut libc::c_void,
        ) -> *mut list_t;
        #[c2rust::src_loc = "40:1"]
        pub fn ponyint_list_reverse(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_list_length(list: *mut list_t) -> size_t;
        #[c2rust::src_loc = "44:1"]
        pub fn ponyint_list_free(list: *mut list_t, f: free_fn);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
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
        pub names_count: size_t,
        pub default_caps_count: size_t,
        pub heap_alloc: size_t,
        pub stack_alloc: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct typecheck_t {
        pub frame: *mut typecheck_frame_t,
        pub stats: typecheck_stats_t,
        pub errors: *mut errors_t,
    }
    use super::_size_t_h::size_t;
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
        pub ast_print_width: size_t,
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
    use super::_size_t_h::size_t;
    use super::frame_h::typecheck_t;
    use super::plugins_t;
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/plugin/plugin.h:1"]
pub mod plugin_h {
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type reach_t;
        #[c2rust::src_loc = "68:16"]
        pub type compile_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:4"]
pub mod package_h {
    #[c2rust::src_loc = "24:1"]
    pub type path_fn = Option<unsafe extern "C" fn(*const libc::c_char, *mut pass_opt_t) -> bool>;
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn handle_path_list(
            paths: *const libc::c_char,
            f: path_fn,
            opt: *mut pass_opt_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:6"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:7"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/dlfcn.h:10"]
pub mod dlfcn_h {
    extern "C" {
        #[c2rust::src_loc = "64:1"]
        pub fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
        #[c2rust::src_loc = "66:1"]
        pub fn dlopen(__path: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
        #[c2rust::src_loc = "67:1"]
        pub fn dlsym(
            __handle: *mut libc::c_void,
            __symbol: *const libc::c_char,
        ) -> *mut libc::c_void;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t};
use self::dlfcn_h::{dlclose, dlopen, dlsym};
use self::error_h::{errorf, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, map_fn};
pub use self::list_h::{
    list_t, ponyint_list_append, ponyint_list_data, ponyint_list_equals, ponyint_list_find,
    ponyint_list_findindex, ponyint_list_free, ponyint_list_index, ponyint_list_length,
    ponyint_list_map, ponyint_list_next, ponyint_list_pop, ponyint_list_push, ponyint_list_reverse,
    ponyint_list_subset,
};
pub use self::package_h::{handle_path_list, path_fn};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE,
    PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT,
    PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER,
    PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL,
    VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::plugin_h::{compile_t, reach_t};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::stdio_h::printf;
use self::stringtab_h::{stringtab, strlist_t};
use self::symtab_h::ast_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "69:22"]
pub struct plugins_t {
    pub contents: list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "39:16"]
pub struct plugin_t {
    pub handle: plugin_handle_t,
    pub path: *const libc::c_char,
    pub init_fn: plugin_init_fn,
    pub final_fn: plugin_final_fn,
    pub help_fn: plugin_help_fn,
    pub parse_options_fn: plugin_parse_options_fn,
    pub visit_ast_fn: plugin_visit_ast_fn,
    pub visit_reach_fn: plugin_visit_reach_fn,
    pub visit_compile_fn: plugin_visit_compile_fn,
    pub user_data: *mut libc::c_void,
}
#[c2rust::src_loc = "36:1"]
pub type plugin_visit_compile_fn =
    Option<unsafe extern "C" fn(*const compile_t, *mut pass_opt_t, *mut libc::c_void) -> ()>;
#[c2rust::src_loc = "34:1"]
pub type plugin_visit_reach_fn =
    Option<unsafe extern "C" fn(*const reach_t, *mut pass_opt_t, bool, *mut libc::c_void) -> ()>;
#[c2rust::src_loc = "32:1"]
pub type plugin_visit_ast_fn =
    Option<unsafe extern "C" fn(*const ast_t, *mut pass_opt_t, pass_id, *mut libc::c_void) -> ()>;
#[c2rust::src_loc = "30:1"]
pub type plugin_parse_options_fn = Option<
    unsafe extern "C" fn(
        *mut pass_opt_t,
        *mut libc::c_int,
        *mut *mut libc::c_char,
        *mut libc::c_void,
    ) -> bool,
>;
#[c2rust::src_loc = "29:1"]
pub type plugin_help_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char>;
#[c2rust::src_loc = "28:1"]
pub type plugin_final_fn = Option<unsafe extern "C" fn(*mut pass_opt_t, *mut libc::c_void) -> ()>;
#[c2rust::src_loc = "27:1"]
pub type plugin_init_fn =
    Option<unsafe extern "C" fn(*mut pass_opt_t, *mut *mut libc::c_void) -> bool>;
#[c2rust::src_loc = "15:1"]
pub type plugin_handle_t = *mut libc::c_void;
#[c2rust::src_loc = "68:1"]
pub type plugins_free_fn = Option<unsafe extern "C" fn(*mut plugin_t) -> ()>;
#[c2rust::src_loc = "68:1"]
pub type plugins_cmp_fn = Option<unsafe extern "C" fn(*mut plugin_t, *mut plugin_t) -> bool>;
#[c2rust::src_loc = "68:1"]
pub type plugins_map_fn =
    Option<unsafe extern "C" fn(*mut plugin_t, *mut libc::c_void) -> *mut plugin_t>;
#[thread_local]
#[c2rust::src_loc = "55:40"]
static mut opt_for_free: *mut pass_opt_t = 0 as *const pass_opt_t as *mut pass_opt_t;
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn plugin_free(mut p: *mut plugin_t) {
    if !opt_for_free.is_null() {
    } else {
        ponyint_assert_fail(
            b"opt_for_free != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/plugin/plugin.c\0" as *const u8
                as *const libc::c_char,
            59 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"plugin_free\0")).as_ptr(),
        );
    };
    if ((*p).final_fn).is_some() {
        ((*p).final_fn).expect("non-null function pointer")(opt_for_free, (*p).user_data);
    }
    dlclose((*p).handle);
    ponyint_pool_free(2 as libc::c_int as size_t, p as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn plugins_free(mut list: *mut plugins_t) {
    let mut free: plugins_free_fn = Some(plugin_free as unsafe extern "C" fn(*mut plugin_t) -> ());
    ponyint_list_free(
        list as *mut list_t,
        ::core::mem::transmute::<plugins_free_fn, free_fn>(free),
    );
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn plugins_findindex(
    mut list: *mut plugins_t,
    mut data: *mut plugin_t,
) -> ssize_t {
    let mut cmp: plugins_cmp_fn = None;
    return ponyint_list_findindex(
        list as *mut list_t,
        ::core::mem::transmute::<plugins_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn plugins_subset(mut a: *mut plugins_t, mut b: *mut plugins_t) -> bool {
    let mut cmp: plugins_cmp_fn = None;
    return ponyint_list_subset(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<plugins_cmp_fn, cmp_fn>(cmp),
    );
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn plugins_equals(mut a: *mut plugins_t, mut b: *mut plugins_t) -> bool {
    let mut cmp: plugins_cmp_fn = None;
    return ponyint_list_equals(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<plugins_cmp_fn, cmp_fn>(cmp),
    );
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn plugins_length(mut list: *mut plugins_t) -> size_t {
    return ponyint_list_length(list as *mut list_t);
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_map(
    mut list: *mut plugins_t,
    mut f: plugins_map_fn,
    mut arg: *mut libc::c_void,
) -> *mut plugins_t {
    return ponyint_list_map(
        list as *mut list_t,
        ::core::mem::transmute::<plugins_map_fn, map_fn>(f),
        arg,
    ) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_append(
    mut list: *mut plugins_t,
    mut data: *mut plugin_t,
) -> *mut plugins_t {
    return ponyint_list_append(list as *mut list_t, data as *mut libc::c_void) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_reverse(mut list: *mut plugins_t) -> *mut plugins_t {
    return ponyint_list_reverse(list as *mut list_t) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_pop(
    mut list: *mut plugins_t,
    mut data: *mut *mut plugin_t,
) -> *mut plugins_t {
    return ponyint_list_pop(list as *mut list_t, data as *mut *mut libc::c_void) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_index(
    mut list: *mut plugins_t,
    mut index: ssize_t,
) -> *mut plugins_t {
    return ponyint_list_index(list as *mut list_t, index) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_next(mut list: *mut plugins_t) -> *mut plugins_t {
    return ponyint_list_next(list as *mut list_t) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:22"]
pub unsafe extern "C" fn plugins_push(
    mut list: *mut plugins_t,
    mut data: *mut plugin_t,
) -> *mut plugins_t {
    return ponyint_list_push(list as *mut list_t, data as *mut libc::c_void) as *mut plugins_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:33"]
pub unsafe extern "C" fn plugins_find(
    mut list: *mut plugins_t,
    mut data: *mut plugin_t,
) -> *mut plugin_t {
    let mut cmp: plugins_cmp_fn = None;
    return ponyint_list_find(
        list as *mut list_t,
        ::core::mem::transmute::<plugins_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    ) as *mut plugin_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:33"]
pub unsafe extern "C" fn plugins_data(mut list: *mut plugins_t) -> *mut plugin_t {
    return ponyint_list_data(list as *mut list_t) as *mut plugin_t;
}
#[c2rust::src_loc = "71:1"]
unsafe extern "C" fn load_plugin(mut path: *const libc::c_char, mut opt: *mut pass_opt_t) -> bool {
    let mut handle: plugin_handle_t = dlopen(path, 0x1 as libc::c_int);
    if handle.is_null() {
        errorf(
            (*opt).check.errors,
            0 as *const libc::c_char,
            b"Couldn't find plugin %s\0" as *const u8 as *const libc::c_char,
            path,
        );
        return 0 as libc::c_int != 0;
    }
    let mut p: *mut plugin_t = ponyint_pool_alloc(2 as libc::c_int as size_t) as *mut plugin_t;
    let ref mut fresh0 = (*p).handle;
    *fresh0 = handle;
    let ref mut fresh1 = (*p).path;
    *fresh1 = stringtab(path);
    let ref mut fresh2 = (*p).user_data;
    *fresh2 = 0 as *mut libc::c_void;
    let ref mut fresh3 = (*p).init_fn;
    *fresh3 = ::core::mem::transmute::<*mut libc::c_void, plugin_init_fn>(dlsym(
        handle,
        b"pony_plugin_init\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh4 = (*p).final_fn;
    *fresh4 = ::core::mem::transmute::<*mut libc::c_void, plugin_final_fn>(dlsym(
        handle,
        b"pony_plugin_final\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh5 = (*p).help_fn;
    *fresh5 = ::core::mem::transmute::<*mut libc::c_void, plugin_help_fn>(dlsym(
        handle,
        b"pony_plugin_help\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh6 = (*p).parse_options_fn;
    *fresh6 = ::core::mem::transmute::<*mut libc::c_void, plugin_parse_options_fn>(dlsym(
        handle,
        b"pony_plugin_parse_options\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh7 = (*p).visit_ast_fn;
    *fresh7 = ::core::mem::transmute::<*mut libc::c_void, plugin_visit_ast_fn>(dlsym(
        handle,
        b"pony_plugin_visit_ast\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh8 = (*p).visit_reach_fn;
    *fresh8 = ::core::mem::transmute::<*mut libc::c_void, plugin_visit_reach_fn>(dlsym(
        handle,
        b"pony_plugin_visit_reach\0" as *const u8 as *const libc::c_char,
    ));
    let ref mut fresh9 = (*p).visit_compile_fn;
    *fresh9 = ::core::mem::transmute::<*mut libc::c_void, plugin_visit_compile_fn>(dlsym(
        handle,
        b"pony_plugin_visit_compile\0" as *const u8 as *const libc::c_char,
    ));
    if ((*p).init_fn).is_some()
        && !((*p).init_fn).expect("non-null function pointer")(opt, &mut (*p).user_data)
    {
        dlclose((*p).handle);
        ponyint_pool_free(2 as libc::c_int as size_t, p as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh10 = (*opt).plugins;
    *fresh10 = plugins_append((*opt).plugins, p);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn plugin_load(
    mut opt: *mut pass_opt_t,
    mut paths: *const libc::c_char,
) -> bool {
    return handle_path_list(
        paths,
        Some(load_plugin as unsafe extern "C" fn(*const libc::c_char, *mut pass_opt_t) -> bool),
        opt,
    );
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn plugin_print_help(mut opt: *mut pass_opt_t) {
    let mut plugins: *mut plugins_t = (*opt).plugins;
    while !plugins.is_null() {
        let mut plugin: *mut plugin_t = plugins_data(plugins);
        if ((*plugin).help_fn).is_some() {
            let mut str: *const libc::c_char =
                ((*plugin).help_fn).expect("non-null function pointer")((*plugin).user_data);
            printf(
                b"\nHelp for plugin %s:\n%s\n\0" as *const u8 as *const libc::c_char,
                (*plugin).path,
                str,
            );
        } else {
            printf(
                b"\nNo help for plugin %s\n\0" as *const u8 as *const libc::c_char,
                (*plugin).path,
            );
        }
        plugins = plugins_next(plugins);
    }
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn plugin_parse_options(
    mut opt: *mut pass_opt_t,
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut plugins: *mut plugins_t = (*opt).plugins;
    while !plugins.is_null() {
        let mut plugin: *mut plugin_t = plugins_data(plugins);
        if ((*plugin).parse_options_fn).is_some() {
            if !((*plugin).parse_options_fn).expect("non-null function pointer")(
                opt,
                argc,
                argv,
                (*plugin).user_data,
            ) {
                ok = 0 as libc::c_int != 0;
            }
        }
        plugins = plugins_next(plugins);
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn plugin_visit_ast(
    mut ast: *const ast_t,
    mut opt: *mut pass_opt_t,
    mut pass: pass_id,
) {
    let mut plugins: *mut plugins_t = (*opt).plugins;
    while !plugins.is_null() {
        let mut plugin: *mut plugin_t = plugins_data(plugins);
        if ((*plugin).visit_ast_fn).is_some() {
            ((*plugin).visit_ast_fn).expect("non-null function pointer")(
                ast,
                opt,
                pass,
                (*plugin).user_data,
            );
        }
        plugins = plugins_next(plugins);
    }
}
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub unsafe extern "C" fn plugin_visit_reach(
    mut r: *const reach_t,
    mut opt: *mut pass_opt_t,
    mut built_vtables: bool,
) {
    let mut plugins: *mut plugins_t = (*opt).plugins;
    while !plugins.is_null() {
        let mut plugin: *mut plugin_t = plugins_data(plugins);
        if ((*plugin).visit_reach_fn).is_some() {
            ((*plugin).visit_reach_fn).expect("non-null function pointer")(
                r,
                opt,
                built_vtables,
                (*plugin).user_data,
            );
        }
        plugins = plugins_next(plugins);
    }
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn plugin_visit_compile(mut c: *const compile_t, mut opt: *mut pass_opt_t) {
    let mut plugins: *mut plugins_t = (*opt).plugins;
    while !plugins.is_null() {
        let mut plugin: *mut plugin_t = plugins_data(plugins);
        if ((*plugin).visit_compile_fn).is_some() {
            ((*plugin).visit_compile_fn).expect("non-null function pointer")(
                c,
                opt,
                (*plugin).user_data,
            );
        }
        plugins = plugins_next(plugins);
    }
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn plugin_unload(mut opt: *mut pass_opt_t) {
    opt_for_free = opt;
    plugins_free((*opt).plugins);
    let ref mut fresh11 = (*opt).plugins;
    *fresh11 = 0 as *mut plugins_t;
    opt_for_free = 0 as *mut pass_opt_t;
}
