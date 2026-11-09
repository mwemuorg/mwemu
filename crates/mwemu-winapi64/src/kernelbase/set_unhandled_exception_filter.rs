use crate::emu;

pub(super) fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = emu.regs().rcx;

    log::trace!(
        "{}** {} kernelbase!SetUnhandledExceptionFilter 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr1,
        emu.colors.nc
    );

    emu.set_uef(ptr1);
    emu.regs_mut().rax = 0;
}
