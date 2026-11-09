use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn EnumSystemFirmwareTables(emu: &mut emu::Emu) {
    let firmware_type = arg(emu, 0) as u64 as u32;
    let buffer_ptr = arg(emu, 1) as u64;
    let buffer_size = arg(emu, 2) as u64 as u32;
    log_red!(
        emu,
        "kernel32!EnumSystemFirmwareTables type:0x{:x} buf:0x{:x} size:{}",
        firmware_type,
        buffer_ptr,
        buffer_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
