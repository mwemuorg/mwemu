use crate::emu;
use crate::winapi::helper;

pub fn LoadRegTypeLib(emu: &mut emu::Emu) {
    let rguid = emu.regs().rcx;
    let w_ver_major = emu.regs().rdx as u16;
    let w_ver_minor = emu.regs().r8 as u16;
    let lcid = emu.regs().r9 as u32;
    let pptlib = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let hndl = helper::handler_create("oleaut32://ITypeLib");
    if pptlib != 0 {
        emu.maps.write_qword(pptlib, hndl);
    }
    log_red!(emu, "oleaut32!LoadRegTypeLib =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
