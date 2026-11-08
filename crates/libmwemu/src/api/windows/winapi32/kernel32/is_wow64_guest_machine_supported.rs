use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsWow64GuestMachineSupported(emu: &mut emu::Emu) {
    let machine = arg(emu, 0) as u64;
    let p_supported = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!IsWow64GuestMachineSupported machine:0x{:x} pSupported:0x{:x}",
        machine,
        p_supported
    );
    if p_supported != 0 {
        emu.maps.write_dword(p_supported, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
