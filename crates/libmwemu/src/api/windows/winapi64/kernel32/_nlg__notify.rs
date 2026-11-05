use crate::emu;

pub fn _NLG_Notify(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_NLG_Notify");
    emu.set_last_error(0);
}
