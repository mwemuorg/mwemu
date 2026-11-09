use crate::emu;
use crate::winapi::helper;

pub fn CreateStreamOnHGlobal(emu: &mut emu::Emu) {
    let hglobal = emu.regs().rcx;
    let f_delete = emu.regs().rdx;
    let ppstm = emu.regs().r8;
    let hndl = helper::handler_create("ole32://IStream");
    if ppstm != 0 {
        emu.maps.write_qword(ppstm, hndl);
    }
    log_red!(emu, "ole32!CreateStreamOnHGlobal =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
