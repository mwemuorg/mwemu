use crate::emu;
use crate::winapi::helper;

pub fn CreateItemMoniker(emu: &mut emu::Emu) {
    let lpsz_delim = emu.regs().rcx;
    let lpsz_item = emu.regs().rdx;
    let ppmk = emu.regs().r8;
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    log_red!(emu, "ole32!CreateItemMoniker =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
