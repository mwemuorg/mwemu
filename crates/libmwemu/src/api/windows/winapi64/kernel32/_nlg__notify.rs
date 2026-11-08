use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn handler(emu: &mut emu::Emu) {
    let __DWORD = emu.regs().rcx;

    log_red!(emu, "kernel32!_NLG_Notify");
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..1 {}
}
