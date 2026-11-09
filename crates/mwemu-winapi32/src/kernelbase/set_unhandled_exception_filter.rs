use crate::emu;

pub(super) fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);

    log_red!(emu, "kernelbase!SetUnhandledExceptionFilter 0x{:x}", ptr1);

    emu.set_uef(ptr1 as u64);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
