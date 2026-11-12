use crate::emu;
use crate::winapi::helper;

pub fn CoSetMessageDispatcher(emu: &mut emu::Emu) {
    let dispatcher = emu.regs().rcx;
    log_red!(
        emu,
        "ole32!CoSetMessageDispatcher dispatcher:0x{:x}",
        dispatcher
    );
    if dispatcher != 0 {
        let _ = helper::handler_create("CoMessageDispatcher");
    }
    emu.set_last_error(0);
}
