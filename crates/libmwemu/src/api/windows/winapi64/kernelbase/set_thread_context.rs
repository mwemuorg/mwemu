use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn SetThreadContext(emu: &mut Emu) {
    trace::api(
        emu,
        "kernelbase!SetThreadContext",
        "thread context",
        true,
        threads::set_thread_context,
    );
}
