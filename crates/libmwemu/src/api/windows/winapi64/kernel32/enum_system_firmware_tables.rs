use crate::emu;

pub fn EnumSystemFirmwareTables(emu: &mut emu::Emu) {
    let firmware_type = emu.regs().rcx as u32;
    let buffer_ptr = emu.regs().rdx;
    let buffer_size = emu.regs().r8 as u32;
    log_red!(
        emu,
        "kernel32!EnumSystemFirmwareTables type:0x{:x} buf:0x{:x} size:{}",
        firmware_type,
        buffer_ptr,
        buffer_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
