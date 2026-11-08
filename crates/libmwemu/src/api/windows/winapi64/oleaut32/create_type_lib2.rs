use crate::emu;
use crate::winapi::helper;

pub fn CreateTypeLib2(emu: &mut emu::Emu) {
    let sys_kind = emu.regs().rcx;
    let sz_file = emu.regs().rdx;
    let pp_ctlib = emu.regs().r8;
    let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
    if pp_ctlib != 0 {
        emu.maps.write_qword(pp_ctlib, hndl);
    }
    log_red!(emu, "oleaut32!CreateTypeLib2 =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
