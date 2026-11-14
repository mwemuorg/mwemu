use crate::emu;
use crate::winapi::helper;

pub fn CreateObjrefMoniker(emu: &mut emu::Emu) {
    let punk = emu.regs().rcx;
    let ppmk = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    log_red!(emu, "ole32!CreateObjrefMoniker =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
