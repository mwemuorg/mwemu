use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn ExitThread(emu: &mut Emu) {
    trace::api(
        emu,
        "kernelbase!ExitThread",
        "exit_code",
        false,
        threads::exit_thread,
    );
}
