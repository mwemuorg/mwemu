use crate::emu;

pub(super) fn __deregister_frame_info(emu: &mut emu::Emu) {
    let p1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "libgcc!__deregister_frame_info {:x}", p1);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
