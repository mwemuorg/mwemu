use crate::emu;
use crate::winapi::helper;

pub fn CreateStdDispatch(emu: &mut emu::Emu) {
    let punk_outer = emu.regs().rcx;
    let pv_this = emu.regs().rdx;
    let ptinfo = emu.regs().r8;
    let ppunk_std_disp = emu.regs().r9;
    let hndl = helper::handler_create("oleaut32://IDispatch");
    if ppunk_std_disp != 0 {
        emu.maps.write_qword(ppunk_std_disp, hndl);
    }
    log_red!(emu, "oleaut32!CreateStdDispatch =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
