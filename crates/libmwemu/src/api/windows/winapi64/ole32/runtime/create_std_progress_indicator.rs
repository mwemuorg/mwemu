use crate::emu;
use crate::winapi::helper;

pub fn CreateStdProgressIndicator(emu: &mut emu::Emu) {
    let hwnd_parent = emu.regs().rcx;
    let psz_title = emu.regs().rdx;
    let pp_ibsc = emu.regs().r8;
    let hndl = helper::handler_create("ole32://IBindStatusCallback");
    if pp_ibsc != 0 {
        emu.maps.write_qword(pp_ibsc, hndl);
    }
    log_red!(emu, "ole32!CreateStdProgressIndicator =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
