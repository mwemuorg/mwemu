use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CheckTokenCapability(emu: &mut emu::Emu) {
    let token_handle = arg(emu, 0) as u64;
    let capability_sid = arg(emu, 1) as u64;
    let has_capability = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!CheckTokenCapability token:0x{:x} sid:0x{:x} hasCap:0x{:x}",
        token_handle,
        capability_sid,
        has_capability
    );
    if has_capability != 0 {
        emu.maps.write_dword(has_capability, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
