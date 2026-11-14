use crate::emu;
use crate::winapi::helper;

pub fn OleSetClipboard(emu: &mut emu::Emu) {
    let pdata_obj = emu.regs().rcx;
    log_red!(emu, "ole32!OleSetClipboard pDataObj:0x{:x}", pdata_obj);
    if pdata_obj != 0 {
        let _ = helper::handler_create("OleClipboard");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
