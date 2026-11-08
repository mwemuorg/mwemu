use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryMemoryResourceNotification(emu: &mut emu::Emu) {
    let h_resource = arg(emu, 0) as u64;
    let p_resource_state = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!QueryMemoryResourceNotification hRes:0x{:x} pState:0x{:x}",
        h_resource,
        p_resource_state
    );
    if p_resource_state != 0 {
        emu.maps.write_dword(p_resource_state, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
