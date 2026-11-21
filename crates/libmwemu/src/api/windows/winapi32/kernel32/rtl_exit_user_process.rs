use crate::emu::Emu;
use crate::winapi::common::threads;

pub fn RtlExitUserProcess(emu: &mut Emu) {
    threads::exit_process(emu, "RtlExitUserProcess");
}
