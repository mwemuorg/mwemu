use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);

    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
