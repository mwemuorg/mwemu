use crate::emu;
use crate::winapi::helper;

pub fn CreateILockBytesOnHGlobal(emu: &mut emu::Emu) {
    let hglobal = emu.regs().rcx;
    let f_delete = emu.regs().rdx;
    let pplkbyt = emu.regs().r8;
    let hndl = helper::handler_create("ole32://ILockBytes");
    if pplkbyt != 0 {
        emu.maps.write_qword(pplkbyt, hndl);
    }
    log_red!(emu, "ole32!CreateILockBytesOnHGlobal =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
