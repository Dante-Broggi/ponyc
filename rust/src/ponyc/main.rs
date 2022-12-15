use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:1"]
pub mod _stdio_h {
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:16"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "63:1"]
        pub fn errors_get_count(errors: *mut errors_t) -> usize;
        #[c2rust::src_loc = "72:1"]
        pub fn errors_print(errors: *mut errors_t);
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
        #[c2rust::src_loc = "13:1"]
        pub fn stringtab_init();
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
    use super::stddef_h::size_t;
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
    use super::stddef_h::size_t;
    use super::stringtab_h::strlist_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "395:1"]
        pub fn generate_passes(program: *mut ast_t, options: *mut pass_opt_t) -> bool;
        #[c2rust::src_loc = "351:1"]
        pub fn pass_opt_done(options: *mut pass_opt_t);
        #[c2rust::src_loc = "347:1"]
        pub fn pass_opt_init(options: *mut pass_opt_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/options/options.h:7"]
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
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_opt_init(
            args: *const opt_arg_t,
            s: *mut opt_state_t,
            argc: *mut libc::c_int,
            argv: *mut *mut libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/options/options.h:7"]
pub mod options_options_h {
    #[c2rust::src_loc = "12:9"]
    pub type ponyc_opt_process_t = libc::c_uint;
    #[c2rust::src_loc = "16:3"]
    pub const CONTINUE: ponyc_opt_process_t = 2;
    #[c2rust::src_loc = "15:3"]
    pub const EXIT_255: ponyc_opt_process_t = 1;
    #[c2rust::src_loc = "14:3"]
    pub const EXIT_0: ponyc_opt_process_t = 0;
    use super::options_h::{opt_arg_t, opt_state_t};
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn ponyc_opt_std_args() -> *mut opt_arg_t;
        #[c2rust::src_loc = "19:1"]
        pub fn ponyc_opt_process(
            s: *mut opt_state_t,
            opt: *mut pass_opt_t,
            print_program_ast: *mut bool,
            print_package_ast: *mut bool,
        ) -> ponyc_opt_process_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/ttycom.h:18"]
pub mod ttycom_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct winsize {
        pub ws_row: libc::c_ushort,
        pub ws_col: libc::c_ushort,
        pub ws_xpixel: libc::c_ushort,
        pub ws_ypixel: libc::c_ushort,
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:1"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "41:7"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ponyc.h:1"]
pub mod ponyc_h {
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn ponyc_shutdown(options: *mut pass_opt_t);
        #[c2rust::src_loc = "9:1"]
        pub fn ponyc_init(options: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::_stdio_h::FILE;
    use super::stddef_h::size_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "154:1"]
        pub fn ast_fprint(fp: *mut FILE, ast: *mut ast_t, width: usize);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:4"]
pub mod package_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "87:1"]
        pub fn program_load(path: *const libc::c_char, opt: *mut pass_opt_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:15"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "83:7"]
        pub fn strncat(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/ioctl.h:18"]
pub mod ioctl_h {
    extern "C" {
        #[c2rust::src_loc = "97:1"]
        pub fn ioctl(_: libc::c_int, _: libc::c_ulong, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:19"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "463:1"]
        pub fn isatty(_: libc::c_int) -> libc::c_int;
    }
}
use self::_malloc_h::calloc;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__int64_t, __uint32_t};
use self::ast_h::{ast_child, ast_fprint, ast_free};
use self::error_h::{errors_get_count, errors_print, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::ioctl_h::ioctl;
pub use self::options_h::{opt_arg_t, opt_state_t, ponyint_opt_init};
pub use self::options_options_h::{
    ponyc_opt_process, ponyc_opt_process_t, ponyc_opt_std_args, CONTINUE, EXIT_0, EXIT_255,
};
use self::package_h::program_load;
pub use self::pass_h::{
    generate_passes, magic_package_t, pass_id, pass_opt_done, pass_opt_init, pass_opt_t, plugins_t,
    verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR,
    PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ,
    PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR,
    PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL,
    VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyc_h::{ponyc_init, ponyc_shutdown};
pub use self::stddef_h::size_t;
use self::stdio_h::{__stderrp, printf};
use self::string_h::{strlen, strncat};
use self::stringtab_h::{stringtab_init, strlist_t};
use self::symtab_h::ast_t;
pub use self::sys__types_h::__darwin_off_t;
pub use self::ttycom_h::winsize;
use self::unistd_h::isatty;
#[c2rust::src_loc = "22:1"]
unsafe extern "C" fn get_width() -> usize {
    let mut width: usize = 80 as libc::c_int as usize;
    if libc::isatty(1 as libc::c_int) != 0 {
        let mut ws: winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if libc::ioctl(
            1 as libc::c_int,
            0x40000000 as libc::c_int as u32 as libc::c_ulong
                | (::core::mem::size_of::<winsize>()
                    & 0x1fff as libc::c_int as libc::c_ulong)
                    << 16 as libc::c_int
                | (('t' as i32) << 8 as libc::c_int) as libc::c_ulong
                | 104 as libc::c_int as libc::c_ulong,
            &mut ws as *mut winsize,
        ) == 0 as libc::c_int
        {
            if ws.ws_col as libc::c_ulong > width {
                width = ws.ws_col as usize;
            }
        }
    }
    width
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn compile_package(
    mut path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
    mut print_program_ast: bool,
    mut print_package_ast: bool,
) -> bool {
    let mut program: *mut ast_t = program_load(path, opt);
    if program.is_null() {
        return 0 as libc::c_int != 0;
    }
    if print_program_ast {
        ast_fprint(__stderrp, program, (*opt).ast_print_width);
    }
    if print_package_ast {
        ast_fprint(__stderrp, ast_child(program), (*opt).ast_print_width);
    }
    let mut ok: bool = generate_passes(program, opt);
    ast_free(program);
    ok
}
#[c2rust::src_loc = "72:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    stringtab_init();
    let mut opt: pass_opt_t = pass_opt_t {
        limit: PASS_PARSE,
        program_pass: PASS_PARSE,
        release: false,
        library: false,
        runtimebc: false,
        staticbin: false,
        pic: false,
        print_stats: false,
        verify: false,
        extfun: false,
        strip_debug: false,
        print_filenames: false,
        check_tree: false,
        lint_llvm: false,
        docs: false,
        docs_private: false,
        verbosity: VERBOSITY_QUIET,
        ast_print_width: 0,
        allow_test_symbols: false,
        parse_trace: false,
        package_search_paths: 0 as *mut strlist_t,
        safe_packages: 0 as *mut strlist_t,
        magic_packages: 0 as *mut magic_package_t,
        argv0: 0 as *const libc::c_char,
        all_args: 0 as *const libc::c_char,
        output: 0 as *const libc::c_char,
        bin_name: 0 as *const libc::c_char,
        link_arch: 0 as *mut libc::c_char,
        linker: 0 as *mut libc::c_char,
        link_ldcmd: 0 as *mut libc::c_char,
        llvm_args: 0 as *const libc::c_char,
        triple: 0 as *mut libc::c_char,
        abi: 0 as *mut libc::c_char,
        cpu: 0 as *mut libc::c_char,
        features: 0 as *mut libc::c_char,
        check: typecheck_t {
            frame: 0 as *mut typecheck_frame_t,
            stats: typecheck_stats_t {
                names_count: 0,
                default_caps_count: 0,
                heap_alloc: 0,
                stack_alloc: 0,
            },
            errors: 0 as *mut errors_t,
        },
        plugins: 0 as *mut plugins_t,
        data: 0 as *mut libc::c_void,
    };
    pass_opt_init(&mut opt);
    opt.release = 1 as libc::c_int != 0;
    opt.output = b".\0" as *const u8 as *const libc::c_char;
    opt.ast_print_width = get_width();
    opt.argv0 = *argv.offset(0 as libc::c_int as isize);
    let mut args_size: usize = 1 as libc::c_int as usize;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        args_size = (args_size as libc::c_ulong).wrapping_add(
            (libc::strlen(*argv.offset(i as isize))).wrapping_add(1),
        ) as usize as usize;
        i += 1;
    }
    opt.all_args = libc::calloc(
        args_size,
        ::core::mem::size_of::<libc::c_char>(),
    ) as *const libc::c_char;
    let mut size_left: usize = args_size;
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 < argc {
        strncat(
            opt.all_args as *mut libc::c_char,
            *argv.offset(i_0 as isize),
            size_left,
        );
        strncat(
            opt.all_args as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        );
        size_left = (size_left as libc::c_ulong).wrapping_sub(
            (libc::strlen(*argv.offset(i_0 as isize))).wrapping_add(1),
        ) as usize as usize;
        i_0 += 1;
    }
    let mut exit_code: ponyc_opt_process_t = EXIT_0;
    let mut print_program_ast: bool = false;
    let mut print_package_ast: bool = false;
    let mut s: opt_state_t = opt_state_t {
        args: 0 as *const opt_arg_t,
        argc: 0 as *mut libc::c_int,
        argv: 0 as *mut *mut libc::c_char,
        arg_val: 0 as *mut libc::c_char,
        opt_start: 0 as *mut libc::c_char,
        opt_end: 0 as *mut libc::c_char,
        match_type: 0,
        idx: 0,
        remove: 0,
    };
    ponyint_opt_init(ponyc_opt_std_args(), &mut s, &mut argc, argv);
    exit_code = ponyc_opt_process(
        &mut s,
        &mut opt,
        &mut print_program_ast,
        &mut print_package_ast,
    );
    if exit_code as libc::c_uint == EXIT_255 as libc::c_int as libc::c_uint {
        errors_print(opt.check.errors);
        pass_opt_done(&mut opt);
        return -(1 as libc::c_int);
    } else {
        if exit_code as libc::c_uint == EXIT_0 as libc::c_int as libc::c_uint {
            pass_opt_done(&mut opt);
            return 0 as libc::c_int;
        }
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    if ponyc_init(&mut opt) {
        if argc == 1 as libc::c_int {
            ok = (ok as libc::c_int
                & compile_package(
                    b".\0" as *const u8 as *const libc::c_char,
                    &mut opt,
                    print_program_ast,
                    print_package_ast,
                ) as libc::c_int)
                != 0;
        } else {
            let mut i_1: libc::c_int = 1 as libc::c_int;
            while i_1 < argc {
                ok = (ok as libc::c_int
                    & compile_package(
                        *argv.offset(i_1 as isize),
                        &mut opt,
                        print_program_ast,
                        print_package_ast,
                    ) as libc::c_int)
                    != 0;
                i_1 += 1;
            }
        }
    }
    if !ok && errors_get_count(opt.check.errors) == 0 {
        printf(b"Error: internal failure not reported\n\0" as *const u8 as *const libc::c_char);
    }
    ponyc_shutdown(&mut opt);
    pass_opt_done(&mut opt);
    return if ok as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
