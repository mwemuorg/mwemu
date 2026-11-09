use crate::emu;
use crate::winapi::helper;

pub fn CreateDispTypeInfo(emu: &mut emu::Emu) {
    let pidata = emu.regs().rcx;
    let lcid = emu.regs().rdx;
    let pptinfo = emu.regs().r8;
    let hndl = helper::handler_create("oleaut32://ITypeInfo");
    if pptinfo != 0 {
        emu.maps.write_qword(pptinfo, hndl);
    }
    log_red!(emu, "oleaut32!CreateDispTypeInfo =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
