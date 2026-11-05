use crate::emu;

pub fn _local_unwind(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_local_unwind");
    emu.set_last_error(0);
}
