use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "66:1"]
        pub fn errors_set_immediate(errors: *mut errors_t, immediate: bool);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
    }
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
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "330:1"]
        pub fn limit_passes(opt: *mut pass_opt_t, pass: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "335:1"]
        pub fn pass_name(pass: pass_id) -> *const libc::c_char;
        #[c2rust::src_loc = "339:1"]
        pub fn pass_next(pass: pass_id) -> pass_id;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/options/options.h:1"]
pub mod options_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct opt_arg_t {
        pub long_opt: *const libc::c_char,
        pub short_opt: libc::c_char,
        pub flag: u32,
        pub id: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:16"]
    pub struct opt_state_t {
        pub args: *const opt_arg_t,
        pub argc: *mut libc::c_int,
        pub argv: *mut *mut libc::c_char,
        pub arg_val: *mut libc::c_char,
        pub opt_start: *mut libc::c_char,
        pub opt_end: *mut libc::c_char,
        pub match_type: libc::c_int,
        pub idx: libc::c_int,
        pub remove: libc::c_int,
    }
    extern "C" {
        #[c2rust::src_loc = "89:1"]
        pub fn ponyint_opt_next(s: *mut opt_state_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/options/options.h:1"]
pub mod options_options_h {
    #[c2rust::src_loc = "12:9"]
    pub type ponyc_opt_process_t = libc::c_uint;
    #[c2rust::src_loc = "16:3"]
    pub const CONTINUE: ponyc_opt_process_t = 2;
    #[c2rust::src_loc = "15:3"]
    pub const EXIT_255: ponyc_opt_process_t = 1;
    #[c2rust::src_loc = "14:3"]
    pub const EXIT_0: ponyc_opt_process_t = 0;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "135:1"]
        pub fn atoi(_: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.h:5"]
pub mod bnfprint_h {
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn print_grammar(antlr: bool, clean: bool);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:6"]
pub mod package_h {
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn package_add_paths(paths: *const libc::c_char, opt: *mut pass_opt_t);
        #[c2rust::src_loc = "61:1"]
        pub fn package_add_safe(paths: *const libc::c_char, opt: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.h:7"]
pub mod buildflagset_h {
    extern "C" {
        #[c2rust::src_loc = "84:1"]
        pub fn define_build_flag(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/plugin/plugin.h:11"]
pub mod plugin_h {
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "70:1"]
        pub fn plugin_load(opt: *mut pass_opt_t, paths: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "72:1"]
        pub fn plugin_print_help(opt: *mut pass_opt_t);
        #[c2rust::src_loc = "74:1"]
        pub fn plugin_parse_options(
            opt: *mut pass_opt_t,
            argc: *mut libc::c_int,
            argv: *mut *mut libc::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:16"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::bnfprint_h::print_grammar;
use self::buildflagset_h::define_build_flag;
use self::error_h::errors_set_immediate;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::options_h::{opt_arg_t, opt_state_t, ponyint_opt_next};
pub use self::options_options_h::{ponyc_opt_process_t, CONTINUE, EXIT_0, EXIT_255};
use self::package_h::{package_add_paths, package_add_safe};
pub use self::pass_h::{
    limit_passes, magic_package_t, pass_id, pass_name, pass_next, pass_opt_t, plugins_t,
    verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR,
    PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ,
    PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR,
    PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL,
    VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::plugin_h::{plugin_load, plugin_parse_options, plugin_print_help};
use self::stdio_h::printf;
use self::stdlib_h::atoi;
use self::string_h::strlen;

#[c2rust::src_loc = "66:3"]
pub const OPT_ANTLRRAW: C2RustUnnamed = 39;
#[c2rust::src_loc = "65:3"]
pub const OPT_ANTLR: C2RustUnnamed = 38;
#[c2rust::src_loc = "64:3"]
pub const OPT_BNF: C2RustUnnamed = 37;
#[c2rust::src_loc = "62:3"]
pub const OPT_LLVM_ARGS: C2RustUnnamed = 36;
#[c2rust::src_loc = "61:3"]
pub const OPT_LINT_LLVM: C2RustUnnamed = 35;
#[c2rust::src_loc = "60:3"]
pub const OPT_EXTFUN: C2RustUnnamed = 34;
#[c2rust::src_loc = "59:3"]
pub const OPT_CHECKTREE: C2RustUnnamed = 33;
#[c2rust::src_loc = "58:3"]
pub const OPT_FILENAMES: C2RustUnnamed = 32;
#[c2rust::src_loc = "57:3"]
pub const OPT_VERIFY: C2RustUnnamed = 31;
#[c2rust::src_loc = "56:3"]
pub const OPT_IMMERR: C2RustUnnamed = 30;
#[c2rust::src_loc = "55:3"]
pub const OPT_WIDTH: C2RustUnnamed = 29;
#[c2rust::src_loc = "54:3"]
pub const OPT_TRACE: C2RustUnnamed = 28;
#[c2rust::src_loc = "53:3"]
pub const OPT_ASTPACKAGE: C2RustUnnamed = 27;
#[c2rust::src_loc = "52:3"]
pub const OPT_AST: C2RustUnnamed = 26;
#[c2rust::src_loc = "51:3"]
pub const OPT_PASSES: C2RustUnnamed = 25;
#[c2rust::src_loc = "50:3"]
pub const OPT_VERBOSE: C2RustUnnamed = 24;
#[c2rust::src_loc = "48:3"]
pub const OPT_PLUGIN: C2RustUnnamed = 23;
#[c2rust::src_loc = "47:3"]
pub const OPT_LINK_LDCMD: C2RustUnnamed = 22;
#[c2rust::src_loc = "46:3"]
pub const OPT_LINKER: C2RustUnnamed = 21;
#[c2rust::src_loc = "45:3"]
pub const OPT_LINK_ARCH: C2RustUnnamed = 20;
#[c2rust::src_loc = "44:3"]
pub const OPT_STATS: C2RustUnnamed = 19;
#[c2rust::src_loc = "43:3"]
pub const OPT_TRIPLE: C2RustUnnamed = 18;
#[c2rust::src_loc = "42:3"]
pub const OPT_FEATURES: C2RustUnnamed = 17;
#[c2rust::src_loc = "41:3"]
pub const OPT_CPU: C2RustUnnamed = 16;
#[c2rust::src_loc = "40:3"]
pub const OPT_ABI: C2RustUnnamed = 15;
#[c2rust::src_loc = "39:3"]
pub const OPT_SAFE: C2RustUnnamed = 14;
#[c2rust::src_loc = "37:3"]
pub const OPT_DOCS_PUBLIC: C2RustUnnamed = 13;
#[c2rust::src_loc = "36:3"]
pub const OPT_DOCS: C2RustUnnamed = 12;
#[c2rust::src_loc = "35:3"]
pub const OPT_NOPIC: C2RustUnnamed = 11;
#[c2rust::src_loc = "34:3"]
pub const OPT_PIC: C2RustUnnamed = 10;
#[c2rust::src_loc = "33:3"]
pub const OPT_STATIC: C2RustUnnamed = 9;
#[c2rust::src_loc = "32:3"]
pub const OPT_RUNTIMEBC: C2RustUnnamed = 8;
#[c2rust::src_loc = "31:3"]
pub const OPT_BIN_NAME: C2RustUnnamed = 7;
#[c2rust::src_loc = "30:3"]
pub const OPT_OUTPUT: C2RustUnnamed = 6;
#[c2rust::src_loc = "29:3"]
pub const OPT_PATHS: C2RustUnnamed = 5;
#[c2rust::src_loc = "28:3"]
pub const OPT_STRIP: C2RustUnnamed = 4;
#[c2rust::src_loc = "27:3"]
pub const OPT_BUILDFLAG: C2RustUnnamed = 3;
#[c2rust::src_loc = "26:3"]
pub const OPT_DEBUG: C2RustUnnamed = 2;
#[c2rust::src_loc = "25:3"]
pub const OPT_HELP: C2RustUnnamed = 1;
#[c2rust::src_loc = "24:3"]
pub const OPT_VERSION: C2RustUnnamed = 0;
#[c2rust::src_loc = "22:1"]
pub type C2RustUnnamed = libc::c_uint;
#[c2rust::src_loc = "69:18"]
static mut std_args: [opt_arg_t; 41] = [
    {
        let mut init = opt_arg_t {
            long_opt: b"version\0" as *const u8 as *const libc::c_char,
            short_opt: 'v' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_VERSION as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"help\0" as *const u8 as *const libc::c_char,
            short_opt: 'h' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_HELP as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"debug\0" as *const u8 as *const libc::c_char,
            short_opt: 'd' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_DEBUG as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"define\0" as *const u8 as *const libc::c_char,
            short_opt: 'D' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_BUILDFLAG as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"strip\0" as *const u8 as *const libc::c_char,
            short_opt: 's' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_STRIP as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"path\0" as *const u8 as *const libc::c_char,
            short_opt: 'p' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_PATHS as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"output\0" as *const u8 as *const libc::c_char,
            short_opt: 'o' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_OUTPUT as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"bin-name\0" as *const u8 as *const libc::c_char,
            short_opt: 'b' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_BIN_NAME as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"runtimebc\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_RUNTIMEBC as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"static\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_STATIC as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"pic\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_PIC as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"nopic\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_NOPIC as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"docs\0" as *const u8 as *const libc::c_char,
            short_opt: 'g' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_DOCS as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"docs-public\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_DOCS_PUBLIC as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"safe\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 1 as libc::c_int) as u32,
            id: OPT_SAFE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"abi\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_ABI as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"cpu\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_CPU as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"features\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_FEATURES as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"triple\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_TRIPLE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"stats\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_STATS as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"link-arch\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_LINK_ARCH as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"linker\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_LINKER as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"link-ldcmd\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_LINK_LDCMD as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"plugin\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_PLUGIN as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"verbose\0" as *const u8 as *const libc::c_char,
            short_opt: 'V' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_VERBOSE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"pass\0" as *const u8 as *const libc::c_char,
            short_opt: 'r' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_PASSES as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ast\0" as *const u8 as *const libc::c_char,
            short_opt: 'a' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_AST as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"astpackage\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_ASTPACKAGE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"trace\0" as *const u8 as *const libc::c_char,
            short_opt: 't' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_TRACE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"width\0" as *const u8 as *const libc::c_char,
            short_opt: 'w' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_WIDTH as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"immerr\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_IMMERR as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"verify\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_VERIFY as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"files\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_FILENAMES as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"checktree\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_CHECKTREE as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"extfun\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_EXTFUN as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"lint-llvm\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_LINT_LLVM as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"llvm-args\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_LLVM_ARGS as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"bnf\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_BNF as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"antlr\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_ANTLR as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"antlrraw\0" as *const u8 as *const libc::c_char,
            short_opt: '\0' as i32 as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_ANTLRRAW as libc::c_int as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: 0 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: 4294967295 as libc::c_uint,
            id: 4294967295 as libc::c_uint,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn ponyc_opt_std_args() -> *mut opt_arg_t {
    return std_args.as_mut_ptr();
}
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn usage() {
    printf(
        b"%s\n%s\n%s\n%s\n%s\n%s\0" as *const u8 as *const libc::c_char,
        b"ponyc [OPTIONS] <package directory>\n\0" as *const u8 as *const libc::c_char,
        b"The package directory defaults to the current directory.\n\0" as *const u8
            as *const libc::c_char,
        b"Options:\n  --version, -v    Print the version of the compiler and exit.\n  --help, -h       Print this help text and exit.\n  --debug, -d      Don't optimise the output.\n  --define, -D     Define the specified build flag.\n    =name\n  --strip, -s      Strip debug info.\n  --path, -p       Add an additional search path.\n    =path          Used to find packages and libraries.\n  --output, -o     Write output to this directory.\n    =path          Defaults to the current directory.\n  --bin-name, -b   Name of executable binary.\n    =name          Defaults to name of the directory.\n  --runtimebc      Compile with the LLVM bitcode file for the runtime.\n  --static         Compile a static binary (experimental, Linux-only).\n  --pic            Compile using position independent code.\n  --nopic          Don't compile using position independent code.\n  --docs, -g       Generate code documentation.\n  --docs-public    Generate code documentation for public types only.\n\0"
            as *const u8 as *const libc::c_char,
        b"Rarely needed options:\n  --safe           Allow only the listed packages to use C FFI.\n    =package       With no packages listed, only builtin is allowed.\n  --abi            Set the target ABI.\n    =name          Default is the host ABI.\n  --cpu            Set the target CPU.\n    =name          Default is the host CPU.\n  --features       CPU features to enable or disable.\n    =+this,-that   Use + to enable, - to disable.\n                   Defaults to detecting all CPU features from the host.\n  --triple         Set the target triple.\n    =name          Defaults to the host triple.\n  --stats          Print some compiler stats.\n  --link-arch      Set the linking architecture.\n    =name          Default is the host architecture.\n  --linker         Set the linker command to use.\n    =name          Default is the compiler used to compile ponyc.\n  --link-ldcmd     Set the ld command to use.\n    =name          Default is `gold` on linux and system default otherwise.\n  --plugin         Use the specified plugin(s).\n    =name\n  --define, -D     Set a compile time definition.\n  --llvm-args      Pass LLVM-specific arguments.\n\0"
            as *const u8 as *const libc::c_char,
        b"Debugging options:\n  --verbose, -V    Verbosity level.\n    =0             Only print errors.\n    =1             Print info on compiler stages.\n    =2             More detailed compilation information.\n    =3             External tool command lines.\n    =4             Very low-level detail.\n  --pass, -r       Restrict phases.\n    =parse\n    =syntax\n    =sugar\n    =scope\n    =import\n    =name\n    =flatten\n    =traits\n    =docs\n    =refer\n    =expr\n    =completeness\n    =verify\n    =final\n    =serialise\n    =reach\n    =paint\n    =ir            Output LLVM IR.\n    =bitcode       Output LLVM bitcode.\n    =asm           Output assembly.\n    =obj           Output an object file.\n    =all           The default: generate an executable.\n  --ast, -a        Output an abstract syntax tree for the whole program.\n  --astpackage     Output an abstract syntax tree for the main package.\n  --trace, -t      Enable parse trace.\n  --width, -w      Width to target when printing the AST.\n    =columns       Defaults to the terminal width.\n  --immerr         Report errors immediately rather than deferring.\n  --checktree      Verify AST well-formedness.\n  --verify         Verify LLVM IR.\n  --extfun         Set function default linkage to external.\n  --files          Print source file names as each is processed.\n  --bnf            Print out the Pony grammar as human readable BNF.\n  --antlr          Print out the Pony grammar as an ANTLR file.\n  --lint-llvm      Run the LLVM linting pass on generated IR.\n\0"
            as *const u8 as *const libc::c_char,
        b"Runtime options for Pony programs (not for use with ponyc):\n  --ponymaxthreads Use N scheduler threads. Defaults to the number of\n                   cores (not hyperthreads) available.\n                   This can't be larger than the number of cores available.\n  --ponyminthreads Minimum number of active scheduler threads allowed.\n                   Defaults to 0, meaning that all scheduler threads are\n                   allowed to be suspended when no work is available.\n                   This can't be larger than --ponymaxthreads if provided,\n                   or the physical cores available\n  --ponynoscale    Don't scale down the scheduler threads.\n                   See --ponymaxthreads on how to specify the number of threads\n                   explicitly. Can't be used with --ponyminthreads.\n  --ponysuspendthreshold\n                   Amount of idle time before a scheduler thread suspends\n                   itself to minimize resource consumption (max 1000 ms,\n                   min 1 ms).\n                   Defaults to 1 ms.\n  --ponycdinterval Run cycle detection every N ms (max 1000 ms, min 10 ms).\n                   Defaults to 100 ms.\n  --ponygcinitial  Defer garbage collection until an actor is using at\n                   least 2^N bytes. Defaults to 2^14.\n  --ponygcfactor   After GC, an actor will next be GC'd at a heap memory\n                   usage N times its current value. This is a floating\n                   point value. Defaults to 2.0.\n  --ponynoyield    Do not yield the CPU when no work is available.\n  --ponynoblock    Do not send block messages to the cycle detector.\n  --ponypin        Pin scheduler threads to CPU cores. The ASIO thread\n                   can also be pinned if `--ponypinasio` is set.\n  --ponypinasio    Pin the ASIO thread to a CPU the way scheduler\n                   threads are pinned to CPUs. Requires `--ponypin` to\n                   be set to have any effect.\n  --ponyprintstatsinterval\n                   Print actor stats before an actor is destroyed and\n                   print scheduler stats every X seconds. Defaults to -1 (never).\n  --ponyversion    Print the version of the compiler and exit.\n  --ponyhelp       Print the runtime usage options and exit.\n\nNOTE: These can be programmatically overridden. See the docstring in the\n      `RuntimeOptions` struct in the `builtin` package.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn print_passes() {
    printf(b"  \0" as *const u8 as *const libc::c_char);
    let mut cur_len: size_t = 2 as libc::c_int as size_t;
    let mut p: pass_id = PASS_PARSE;
    while (p as libc::c_uint) < PASS_ALL as libc::c_int as libc::c_uint {
        let mut name: *const libc::c_char = pass_name(p);
        let mut len: size_t = (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if cur_len.wrapping_add(len) < 80 as libc::c_int as libc::c_ulong {
            printf(b"%s,\0" as *const u8 as *const libc::c_char, name);
            cur_len = (cur_len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        } else {
            printf(b"\n  %s,\0" as *const u8 as *const libc::c_char, name);
            cur_len = len.wrapping_add(2 as libc::c_int as libc::c_ulong);
        }
        p = pass_next(p);
    }
    let mut name_0: *const libc::c_char = pass_name(PASS_ALL);
    if cur_len.wrapping_add(strlen(name_0)) < 80 as libc::c_int as libc::c_ulong {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, name_0);
    } else {
        printf(b"\n%s\n\0" as *const u8 as *const libc::c_char, name_0);
    };
}
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn special_opt_processing(mut opt: *mut pass_opt_t) -> ponyc_opt_process_t {
    (*opt).pic = 1 as libc::c_int != 0;
    define_build_flag(b"scheduler_scaling_pthreads\0" as *const u8 as *const libc::c_char);
    let ref mut fresh0 = (*opt).llvm_args;
    *fresh0 = 0 as *const libc::c_char;
    return CONTINUE;
}
#[no_mangle]
#[c2rust::src_loc = "263:1"]
pub unsafe extern "C" fn ponyc_opt_process(
    mut s: *mut opt_state_t,
    mut opt: *mut pass_opt_t,
    mut print_program_ast: *mut bool,
    mut print_package_ast: *mut bool,
) -> ponyc_opt_process_t {
    let mut exit_code: ponyc_opt_process_t = CONTINUE;
    let mut id: libc::c_int = 0;
    *print_program_ast = 0 as libc::c_int != 0;
    *print_package_ast = 0 as libc::c_int != 0;
    exit_code = special_opt_processing(opt);
    if exit_code as libc::c_uint != CONTINUE as libc::c_int as libc::c_uint {
        return exit_code;
    }
    let mut wants_help: bool = 0 as libc::c_int != 0;
    loop {
        id = ponyint_opt_next(s);
        if !(id != -(1 as libc::c_int)) {
            break;
        }
        match id {
            0 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"UNKNOWN\0" as *const u8 as *const libc::c_char,
                );
                printf(
                    b"Defaults: pic=%s\n\0" as *const u8 as *const libc::c_char,
                    if (*opt).pic as libc::c_int != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
                return EXIT_0;
            }
            1 => {
                wants_help = 1 as libc::c_int != 0;
            }
            2 => {
                (*opt).release = 0 as libc::c_int != 0;
            }
            3 => {
                define_build_flag((*s).arg_val);
            }
            4 => {
                (*opt).strip_debug = 1 as libc::c_int != 0;
            }
            5 => {
                package_add_paths((*s).arg_val, opt);
            }
            6 => {
                let ref mut fresh1 = (*opt).output;
                *fresh1 = (*s).arg_val;
            }
            7 => {
                let ref mut fresh2 = (*opt).bin_name;
                *fresh2 = (*s).arg_val;
            }
            8 => {
                (*opt).runtimebc = 1 as libc::c_int != 0;
            }
            9 => {
                (*opt).staticbin = 1 as libc::c_int != 0;
            }
            10 => {
                (*opt).pic = 1 as libc::c_int != 0;
            }
            11 => {
                (*opt).pic = 0 as libc::c_int != 0;
            }
            12 => {
                (*opt).docs = 1 as libc::c_int != 0;
                (*opt).docs_private = 1 as libc::c_int != 0;
            }
            13 => {
                (*opt).docs = 1 as libc::c_int != 0;
                (*opt).docs_private = 0 as libc::c_int != 0;
            }
            14 => {
                if !package_add_safe((*s).arg_val, opt) {
                    printf(
                        b"Error adding safe packages: %s\n\0" as *const u8 as *const libc::c_char,
                        (*s).arg_val,
                    );
                    exit_code = EXIT_255;
                }
            }
            15 => {
                let ref mut fresh3 = (*opt).abi;
                *fresh3 = (*s).arg_val;
            }
            16 => {
                let ref mut fresh4 = (*opt).cpu;
                *fresh4 = (*s).arg_val;
            }
            17 => {
                let ref mut fresh5 = (*opt).features;
                *fresh5 = (*s).arg_val;
            }
            18 => {
                let ref mut fresh6 = (*opt).triple;
                *fresh6 = (*s).arg_val;
            }
            19 => {
                (*opt).print_stats = 1 as libc::c_int != 0;
            }
            20 => {
                let ref mut fresh7 = (*opt).link_arch;
                *fresh7 = (*s).arg_val;
            }
            21 => {
                let ref mut fresh8 = (*opt).linker;
                *fresh8 = (*s).arg_val;
            }
            22 => {
                let ref mut fresh9 = (*opt).link_ldcmd;
                *fresh9 = (*s).arg_val;
            }
            23 => {
                if !plugin_load(opt, (*s).arg_val) {
                    printf(
                        b"Error loading plugins: %s\n\0" as *const u8 as *const libc::c_char,
                        (*s).arg_val,
                    );
                    exit_code = EXIT_255;
                }
            }
            26 => {
                *print_program_ast = 1 as libc::c_int != 0;
            }
            27 => {
                *print_package_ast = 1 as libc::c_int != 0;
            }
            28 => {
                (*opt).parse_trace = 1 as libc::c_int != 0;
            }
            29 => {
                (*opt).ast_print_width = atoi((*s).arg_val) as size_t;
            }
            30 => {
                errors_set_immediate((*opt).check.errors, 1 as libc::c_int != 0);
            }
            31 => {
                (*opt).verify = 1 as libc::c_int != 0;
            }
            34 => {
                (*opt).extfun = 1 as libc::c_int != 0;
            }
            32 => {
                (*opt).print_filenames = 1 as libc::c_int != 0;
            }
            33 => {
                (*opt).check_tree = 1 as libc::c_int != 0;
            }
            35 => {
                (*opt).lint_llvm = 1 as libc::c_int != 0;
            }
            36 => {
                let ref mut fresh10 = (*opt).llvm_args;
                *fresh10 = (*s).arg_val;
            }
            37 => {
                print_grammar(0 as libc::c_int != 0, 1 as libc::c_int != 0);
                return EXIT_0;
            }
            38 => {
                print_grammar(1 as libc::c_int != 0, 1 as libc::c_int != 0);
                return EXIT_0;
            }
            39 => {
                print_grammar(1 as libc::c_int != 0, 0 as libc::c_int != 0);
                return EXIT_0;
            }
            24 => {
                let mut v: libc::c_int = atoi((*s).arg_val);
                if v >= 0 as libc::c_int && v <= 4 as libc::c_int {
                    (*opt).verbosity = v as verbosity_level;
                } else {
                    printf(
                        b"Verbosity must be 0..4, %d is invalid\n\0" as *const u8
                            as *const libc::c_char,
                        v,
                    );
                    exit_code = EXIT_255;
                }
            }
            25 => {
                if !limit_passes(opt, (*s).arg_val) {
                    printf(
                        b"Invalid pass=%s it should be one of:\n\0" as *const u8
                            as *const libc::c_char,
                        (*s).arg_val,
                    );
                    print_passes();
                    exit_code = EXIT_255;
                }
            }
            -2 => return EXIT_255,
            _ => {
                printf(
                    b"BUG: unprocessed option id %d\n\0" as *const u8 as *const libc::c_char,
                    id,
                );
                return EXIT_255;
            }
        }
    }
    if !plugin_parse_options(opt, (*s).argc, (*s).argv) {
        exit_code = EXIT_255;
    }
    if wants_help {
        usage();
        plugin_print_help(opt);
        if exit_code as libc::c_uint != EXIT_255 as libc::c_int as libc::c_uint {
            exit_code = EXIT_0;
        }
        return exit_code;
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < *(*s).argc {
        if *(*((*s).argv).offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            printf(
                b"Unrecognised option: %s\n\0" as *const u8 as *const libc::c_char,
                *((*s).argv).offset(i as isize),
            );
            exit_code = EXIT_255;
        }
        i += 1;
    }
    exit_code
}
