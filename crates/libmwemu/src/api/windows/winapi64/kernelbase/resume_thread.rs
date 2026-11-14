use crate::emu::Emu;
use crate::winapi::common::{threads, trace};

pub fn ResumeThread(emu: &mut Emu) {
    trace::api(
        emu,
        "kernelbase!ResumeThread",
        "thread",
        true,
        threads::resume_thread,
    );
}
