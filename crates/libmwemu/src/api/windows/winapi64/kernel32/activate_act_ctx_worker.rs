
use crate::emu;
use crate::windows::constants;
use crate::winapi::helper;
pub fn ActivateActCtxWorker(emu: &mut emu::Emu) {
    let _Handle = emu.regs().rcx as u32;
    let _Cookie = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!ActivateActCtxWorker");
    let handle = helper::handler_create("ActivateActCtxWorker");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
