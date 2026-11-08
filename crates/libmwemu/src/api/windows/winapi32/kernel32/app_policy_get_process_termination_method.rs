use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AppPolicyGetProcessTerminationMethod(emu: &mut emu::Emu) {
    let token = arg(emu, 0) as u64;
    let p_policy = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!AppPolicyGetProcessTerminationMethod token:0x{:x} pPolicy:0x{:x}",
        token,
        p_policy
    );

    if p_policy != 0 {
        emu.maps.write_dword(p_policy, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
