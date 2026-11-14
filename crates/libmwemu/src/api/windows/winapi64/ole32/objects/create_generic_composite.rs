use crate::emu;
use crate::winapi::helper;

pub fn CreateGenericComposite(emu: &mut emu::Emu) {
    let pmk_first = emu.regs().rcx;
    let pmk_rest = emu.regs().rdx;
    let ppmk_composite = emu.regs().r8;
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk_composite != 0 {
        emu.maps.write_qword(ppmk_composite, hndl);
    }
    log_red!(emu, "ole32!CreateGenericComposite =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
