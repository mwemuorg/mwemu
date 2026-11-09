use crate::emu;
use crate::winapi::helper;

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let _sam_desired = emu.regs().rcx;
    let phk_result = emu.regs().rdx;
    let hndl = helper::handler_create("registry://HKEY_CURRENT_USER");
    if phk_result != 0 {
        emu.maps.write_qword(phk_result, hndl);
    }
    log_red!(emu, "advapi32!RegOpenCurrentUser => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
