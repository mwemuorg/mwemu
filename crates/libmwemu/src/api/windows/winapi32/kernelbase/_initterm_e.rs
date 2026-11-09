use crate::emu;

pub(super) fn _initterm_e(emu: &mut emu::Emu) {
    let ptr1 = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let ptr2 = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    log_red!(emu, "kernelbase!_initterm_e 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
