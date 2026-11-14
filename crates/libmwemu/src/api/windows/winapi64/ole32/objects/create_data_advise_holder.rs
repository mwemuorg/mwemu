use crate::emu;
use crate::winapi::helper;

pub fn CreateDataAdviseHolder(emu: &mut emu::Emu) {
    let pp_dah = emu.regs().rcx;
    let hndl = helper::handler_create("ole32://IDataAdviseHolder");
    if pp_dah != 0 {
        emu.maps.write_qword(pp_dah, hndl);
    }
    log_red!(emu, "ole32!CreateDataAdviseHolder =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
