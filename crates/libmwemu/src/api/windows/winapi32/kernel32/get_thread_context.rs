use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn GetThreadContext(emu: &mut Emu) {
    trace::api(
        emu,
        "kernel32!GetThreadContext",
        "thread context",
        true,
        threads::get_thread_context,
    );
}
