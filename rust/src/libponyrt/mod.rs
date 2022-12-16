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

// -- Top level contents

#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
mod pony_h {
    #[c2rust::src_loc = "30:16"]
    pub use crate::libponyrt::actor::actor::pony_actor_t;

    #[derive(Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }

    #[c2rust::src_loc = "74:1"]
    pub type pony_trace_fn = Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> ()>;

    #[c2rust::src_loc = "84:1"]
    pub type pony_serialise_fn = Option<
        unsafe extern "C" fn(
            *mut pony_ctx_t,
            *mut libc::c_void,
            *mut libc::c_void,
            usize,
            libc::c_int,
        ) -> (),
    >;

    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;

    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;

    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;

    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;

    use crate::libponyrt::sched::scheduler::pony_ctx_t;

    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:22"]
    pub struct _pony_type_t {
        pub id: u32,
        pub size: u32,
        pub field_count: u32,
        pub field_offset: u32,
        pub instance: *mut libc::c_void,
        pub trace: pony_trace_fn,
        pub serialise_trace: pony_trace_fn,
        pub serialise: pony_serialise_fn,
        pub deserialise: pony_trace_fn,
        pub custom_serialise_space: pony_custom_serialise_space_fn,
        pub custom_deserialise: pony_custom_deserialise_fn,
        pub dispatch: pony_dispatch_fn,
        pub final_0: pony_final_fn,
        pub event_notify: u32,
        pub traits: *mut *mut libc::uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
}
pub use pony_h::*;
