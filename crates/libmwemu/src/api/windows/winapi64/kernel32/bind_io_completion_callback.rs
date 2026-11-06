
use crate::emu;
use crate::windows::constants;
use crate::winapi::helper;
pub fn BindIoCompletionCallback(emu: &mut emu::Emu) {
    let _FileHandle = emu.regs().rcx as u32;
    let _Function = emu.regs().rdx as u32;
    let _Flags = emu.regs().r8 as u32;
    log_red!(emu, "kernel32!BindIoCompletionCallback");
    let handle = helper::handler_create("BindIoCompletionCallback");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
