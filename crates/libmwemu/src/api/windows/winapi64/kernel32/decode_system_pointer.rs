use crate::emu;

pub fn DecodeSystemPointer(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!DecodeSystemPointer ptr:0x{:x}", ptr);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ptr;
}
