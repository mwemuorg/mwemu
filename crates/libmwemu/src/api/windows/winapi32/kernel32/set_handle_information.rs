use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetHandleInformation(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let mask = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!SetHandleInformation handle:0x{:x} mask:0x{:x} flags:0x{:x}",
        handle,
        mask,
        flags
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
