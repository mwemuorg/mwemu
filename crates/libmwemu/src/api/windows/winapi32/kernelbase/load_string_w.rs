use crate::emu;

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let id = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let buff = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0);
    let len = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);

    log_red!(emu, "kernelbase!LoadStringW {} 0x{}", id, buff);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
