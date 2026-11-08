use crate::emu;
use crate::winapi::helper;

pub fn GetRunningObjectTable(emu: &mut emu::Emu) {
    let reserved = emu.regs().rcx;
    let pp_rot = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://IRunningObjectTable");
    if pp_rot != 0 {
        emu.maps.write_qword(pp_rot, hndl);
    }
    log_red!(emu, "ole32!GetRunningObjectTable =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
