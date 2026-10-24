use crate::emu;

pub fn EncodePointer(emu: &mut emu::Emu) {
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!EncodePointer ptr: 0x{:x}", ptr);

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr;
}
