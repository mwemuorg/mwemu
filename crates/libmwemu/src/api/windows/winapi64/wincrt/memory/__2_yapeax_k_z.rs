use crate::emu;
use crate::winapi::helper;

pub fn __2_YAPEAX_K_Z(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 0;
}
