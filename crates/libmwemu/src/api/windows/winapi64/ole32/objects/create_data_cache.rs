use crate::emu;
use crate::winapi::helper;

pub fn CreateDataCache(emu: &mut emu::Emu) {
    let punk_outer = emu.regs().rcx;
    let rclsid = emu.regs().rdx;
    let riid = emu.regs().r8;
    let ppv = emu.regs().r9;
    let hndl = helper::handler_create("ole32://IOleCache");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "ole32!CreateDataCache outer:0x{:x} clsid:0x{:x} =0x{:x}",
        punk_outer,
        rclsid,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
