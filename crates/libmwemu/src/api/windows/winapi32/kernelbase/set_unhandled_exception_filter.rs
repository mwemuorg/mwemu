use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let ptr1 = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernelbase!SetUnhandledExceptionFilter 0x{:x}", ptr1);

    emu.set_uef(ptr1 as u64);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
