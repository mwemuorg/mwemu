use crate::emu;
use crate::winapi::helper;

pub fn LoadTypeLib(emu: &mut emu::Emu) {
    let sz_file = emu.regs().rcx;
    let pptlib = emu.regs().rdx;
    let hndl = helper::handler_create("oleaut32://ITypeLib");
    if pptlib != 0 {
        emu.maps.write_qword(pptlib, hndl);
    }
    let path = emu.maps.read_wide_string(sz_file);
    log_red!(emu, "oleaut32!LoadTypeLib '{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
