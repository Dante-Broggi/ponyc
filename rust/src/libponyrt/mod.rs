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
    #[derive(Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
}
pub use pony_h::*;
