use crate::emu;
use crate::winapi::helper;

pub fn CreateAntiMoniker(emu: &mut emu::Emu) {
    let ppmk = emu.regs().rcx;
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    log_red!(
        emu,
        "ole32!CreateAntiMoniker ppmk:0x{:x} =0x{:x}",
        ppmk,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
