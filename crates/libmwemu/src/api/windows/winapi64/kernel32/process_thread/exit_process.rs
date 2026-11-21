use crate::emu::Emu;
use crate::winapi::common::threads;

pub fn ExitProcess(emu: &mut Emu) {
    threads::exit_process(emu, "ExitProcess");
}
