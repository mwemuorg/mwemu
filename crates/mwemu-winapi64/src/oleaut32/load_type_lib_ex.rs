use crate::emu;
use crate::winapi::helper;

pub fn LoadTypeLibEx(emu: &mut emu::Emu) {
    let sz_file = emu.regs().rcx;
    let reg_kind = emu.regs().rdx;
    let pptlib = emu.regs().r8;
    let hndl = helper::handler_create("oleaut32://ITypeLib");
    if pptlib != 0 {
        emu.maps.write_qword(pptlib, hndl);
    }
    let path = emu.maps.read_wide_string(sz_file);
    log_red!(emu, "oleaut32!LoadTypeLibEx '{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
