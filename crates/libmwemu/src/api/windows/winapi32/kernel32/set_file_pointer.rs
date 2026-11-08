use crate::emu;

pub fn SetFilePointer(emu: &mut emu::Emu) {
    let hfile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dist_low = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
    let dist_high_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let method = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!SetFilePointer hFile:0x{:x} dist:{} high_ptr:0x{:x} method:{}",
        hfile,
        dist_low,
        dist_high_ptr,
        method
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    // Stub: just return the distance as the new pointer if high isn't used much
    emu.regs_mut().rax = dist_low as u64;
}
