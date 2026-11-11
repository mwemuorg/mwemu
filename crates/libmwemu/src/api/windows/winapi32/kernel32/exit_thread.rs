use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn ExitThread(emu: &mut Emu) {
    trace::api(
        emu,
        "kernel32!ExitThread",
        "exit_code",
        false,
        threads::exit_thread,
    );
}
