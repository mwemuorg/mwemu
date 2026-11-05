use crate::emu;

pub fn DecodePointer(emu: &mut emu::Emu) {
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!DecodePointer ptr: 0x{:x}", ptr);

    emu.stack_pop32(false);
    emu.set_last_error(0);
    emu.regs_mut().rax = ptr;
}
