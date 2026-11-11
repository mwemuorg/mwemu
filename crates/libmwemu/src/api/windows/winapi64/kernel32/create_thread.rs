use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn CreateThread(emu: &mut Emu) {
    trace::api(
        emu,
        "kernel32!CreateThread",
        "attributes stack_size start parameter flags thread_id",
        true,
        threads::create_thread,
    );
}
