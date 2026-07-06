use crate::emu;

pub fn DecodePointer(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!DecodePointer cannot read the pointer") as u64;

    log_red!(emu, "kernel32!DecodePointer ptr: 0x{:x}", ptr);

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr;
}
