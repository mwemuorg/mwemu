use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetIoRingInfo(emu: &mut emu::Emu) {
    let io_ring = arg(emu, 0) as u64;
    let info_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetIoRingInfo ring:0x{:x} info:0x{:x}",
        io_ring,
        info_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80070057u64;
    stdcall_ret(emu, 2);
}
