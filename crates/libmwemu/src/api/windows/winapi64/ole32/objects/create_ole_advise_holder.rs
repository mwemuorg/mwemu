use crate::emu;
use crate::winapi::helper;

pub fn CreateOleAdviseHolder(emu: &mut emu::Emu) {
    let pp_oah = emu.regs().rcx;
    let hndl = helper::handler_create("ole32://IOleAdviseHolder");
    if pp_oah != 0 {
        emu.maps.write_qword(pp_oah, hndl);
    }
    log_red!(emu, "ole32!CreateOleAdviseHolder =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
