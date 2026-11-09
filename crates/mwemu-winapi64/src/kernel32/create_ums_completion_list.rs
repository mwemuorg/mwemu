use crate::emu;
use crate::winapi::helper;
pub fn CreateUmsCompletionList(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!CreateUmsCompletionList handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let handle = helper::handler_create("CreateUmsCompletionList");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
