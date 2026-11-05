use crate::emu;

pub fn DecodePointer(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!DecodePointer ptr: 0x{:x}", ptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = ptr;
}
