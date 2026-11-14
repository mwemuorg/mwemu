use crate::emu;
use crate::winapi::helper;

pub fn OleIsCurrentClipboard(emu: &mut emu::Emu) {
    let pdata_obj = emu.regs().rcx;
    log_red!(
        emu,
        "ole32!OleIsCurrentClipboard pDataObj:0x{:x}",
        pdata_obj
    );
    let result = if helper::handler_exist(pdata_obj) {
        0u64
    } else {
        1u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
