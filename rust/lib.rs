#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]
#![feature(register_tool)]
#![feature(thread_local)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
    pub mod libponyc {
        pub mod ast {
            pub mod ast;
            pub mod bnfprint;
            pub mod error;
            pub mod frame;
            pub mod id;
            pub mod lexer;
            pub mod lexint;
            pub mod parser;
            pub mod parserapi;
            pub mod printbuf;
            pub mod source;
            pub mod stringtab;
            pub mod symtab;
            pub mod token;
            pub mod treecheck;
        } // mod ast
        pub mod codegen {
            pub mod codegen;
            pub mod genbox;
            pub mod gencall;
            pub mod gencontrol;
            pub mod gendesc;
            pub mod genexe;
            pub mod genexpr;
            pub mod genfun;
            pub mod genheader;
            pub mod genident;
            pub mod genmatch;
            pub mod genname;
            pub mod genobj;
            pub mod genoperator;
            pub mod genprim;
            pub mod genreference;
            pub mod genserialise;
            pub mod gentrace;
            pub mod gentype;
        } // mod codegen
        pub mod expr {
            pub mod array;
            pub mod call;
            pub mod control;
            pub mod ffi;
            pub mod lambda;
            pub mod literal;
            pub mod r#match;
            pub mod operator;
            pub mod postfix;
            pub mod reference;
        } // mod expr
        pub mod options {
            pub mod options;
        } // mod options
        pub mod pass {
            pub mod completeness;
            pub mod docgen;
            pub mod expr;
            pub mod finalisers;
            pub mod flatten;
            pub mod import;
            pub mod names;
            pub mod pass;
            pub mod refer;
            pub mod scope;
            pub mod serialisers;
            pub mod sugar;
            pub mod syntax;
            pub mod traits;
            pub mod verify;
        } // mod pass
        pub mod pkg {
            pub mod buildflagset;
            pub mod ifdef;
            pub mod package;
            pub mod platformfuns;
            pub mod program;
            pub mod r#use;
        } // mod pkg
        pub mod platform {
            pub mod paths;
            pub mod vcvars;
        } // mod platform
        pub mod plugin {
            pub mod plugin;
        } // mod plugin
        pub mod ponyc;
        pub mod ponydoc;
        pub mod r#type {
            pub mod alias;
            pub mod assemble;
            pub mod cap;
            pub mod compattype;
            pub mod lookup;
            pub mod matchtype;
            pub mod reify;
            pub mod safeto;
            pub mod sanitise;
            pub mod subtype;
            pub mod typeparam;
            pub mod viewpoint;
        } // mod r#type
        pub mod reach {
            pub mod paint;
            pub mod reach;
            pub mod subtype;
        } // mod reach
        pub mod verify {
            pub mod call;
            pub mod control;
            pub mod fun;
            pub mod r#type;
        } // mod verify
    } // mod libponyc
    pub mod libponyrt {
        pub mod actor {
            pub mod actor;
            pub mod messageq;
        } // mod actor
        pub mod asio {
            pub mod asio;
            pub mod emscripten;
            pub mod epoll;
            pub mod event;
            pub mod iocp;
            pub mod kqueue;
        } // mod asio
        pub mod ds {
            pub mod fun;
            pub mod hash;
            pub mod list;
            pub mod stack;
        } // mod ds
        pub mod gc {
            pub mod actormap;
            pub mod cycle;
            pub mod delta;
            pub mod gc;
            pub mod objectmap;
            pub mod serialise;
            pub mod trace;
        } // mod gc
        pub mod lang {
            pub mod directory;
            pub mod errno;
            pub mod io;
            pub mod lsda;
            pub mod paths;
            pub mod posix_except;
            pub mod process;
            pub mod socket;
            pub mod ssl;
            pub mod stat;
            pub mod stdfd;
            pub mod time;
            pub mod win_except;
        } // mod lang
        pub mod mem {
            pub mod alloc;
            pub mod heap;
            pub mod pagemap;
            pub mod pool;
        } // mod mem
        pub mod options {
            pub mod options;
        } // mod options
        pub mod platform {
            pub mod ponyassert;
            pub mod threads;
        } // mod platform
        pub mod sched {
            pub mod cpu;
            pub mod mpmcq;
            pub mod mutemap;
            pub mod scheduler;
            pub mod start;
            pub mod systematic_testing;
        } // mod sched
    } // mod libponyrt
    pub mod ponyc {
        pub mod main;
    } // mod ponyc
} // mod src
