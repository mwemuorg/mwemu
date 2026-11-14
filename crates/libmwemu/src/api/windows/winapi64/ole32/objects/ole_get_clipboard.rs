use crate::emu;
use crate::winapi::helper;

pub fn OleGetClipboard(emu: &mut emu::Emu) {
    let ppdata_obj = emu.regs().rcx;
    log_red!(emu, "ole32!OleGetClipboard ppDataObj:0x{:x}", ppdata_obj);
    if ppdata_obj != 0 {
        let handle = helper::handler_create("OleClipboardData");
        emu.maps.write_qword(ppdata_obj, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
