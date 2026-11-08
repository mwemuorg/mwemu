use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryProtectedPolicy(emu: &mut emu::Emu) {
    let policy_guid = arg(emu, 0) as u64;
    let p_policy_value = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!QueryProtectedPolicy guid:0x{:x} pValue:0x{:x}",
        policy_guid,
        p_policy_value
    );
    if p_policy_value != 0 {
        emu.maps.write_qword(p_policy_value, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
