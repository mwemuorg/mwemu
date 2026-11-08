use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetProtectedPolicy(emu: &mut emu::Emu) {
    let policy_guid = arg(emu, 0) as u64;
    let policy_value = arg(emu, 1) as u64;
    let p_old_value = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!SetProtectedPolicy guid:0x{:x} value:0x{:x} pOld:0x{:x}",
        policy_guid,
        policy_value,
        p_old_value
    );
    if p_old_value != 0 {
        emu.maps.write_qword(p_old_value, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
