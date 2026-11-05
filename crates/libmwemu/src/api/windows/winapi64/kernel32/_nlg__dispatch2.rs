use crate::emu;

pub fn _NLG_Dispatch2(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_NLG_Dispatch2");
    emu.set_last_error(0);
}
