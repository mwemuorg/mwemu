use crate::emu;
use crate::winapi::helper;

pub fn CreateErrorInfo(emu: &mut emu::Emu) {
    let pp_errinfo = emu.regs().rcx;
    let hndl = helper::handler_create("oleaut32://ICreateErrorInfo");
    if pp_errinfo != 0 {
        emu.maps.write_qword(pp_errinfo, hndl);
    }
    log_red!(emu, "oleaut32!CreateErrorInfo =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
