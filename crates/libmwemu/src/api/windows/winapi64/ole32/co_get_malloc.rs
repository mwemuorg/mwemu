use crate::emu;
use crate::winapi::helper;

pub fn CoGetMalloc(emu: &mut emu::Emu) {
    let _context = emu.regs().rcx;
    let ppv = emu.regs().rdx;
    let hndl = helper::handler_create("com_malloc");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(emu, "ole32!CoGetMalloc => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
