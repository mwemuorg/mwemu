use crate::emu;
use crate::winapi::helper;

pub fn GetActiveObject(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let pv_reserved = emu.regs().rdx;
    let ppunk = emu.regs().r8;
    let hndl = helper::handler_create("oleaut32://IUnknown");
    if ppunk != 0 {
        emu.maps.write_qword(ppunk, hndl);
    }
    log_red!(emu, "oleaut32!GetActiveObject =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
