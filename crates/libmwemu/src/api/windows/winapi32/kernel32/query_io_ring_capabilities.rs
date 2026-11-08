use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryIoRingCapabilities(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;

    log_red!(
        emu,
        "kernel32!QueryIoRingCapabilities param1:0x{:x}",
        param1
    );
    if arg(emu, 0) as u64 != 0 && emu.maps.is_mapped(arg(emu, 0) as u64) {
        emu.maps.write_dword(arg(emu, 0) as u64, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
