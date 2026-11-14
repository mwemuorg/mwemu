use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn __deregister_frame_info(emu: &mut emu::Emu) {
    let p1 = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "libgcc!__deregister_frame_info {:x}", p1);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
