use crate::emu;
use crate::winapi::helper;

pub fn CreateBindCtx(emu: &mut emu::Emu) {
    let reserved = emu.regs().rcx;
    let ppbc = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://IBindCtx");
    if ppbc != 0 {
        emu.maps.write_qword(ppbc, hndl);
    }
    log_red!(emu, "ole32!CreateBindCtx =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
