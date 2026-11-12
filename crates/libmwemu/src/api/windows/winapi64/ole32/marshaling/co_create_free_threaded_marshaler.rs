use crate::emu;
use crate::winapi::helper;

pub fn CoCreateFreeThreadedMarshaler(emu: &mut emu::Emu) {
    let punk_outer = emu.regs().rcx;
    let ppunk_marshal = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://IMarshal");
    if ppunk_marshal != 0 {
        emu.maps.write_qword(ppunk_marshal, hndl);
    }
    log_red!(emu, "ole32!CoCreateFreeThreadedMarshaler =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
