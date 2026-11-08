use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::structures;

pub fn GetVersionExA(emu: &mut emu::Emu) {
    let ptr = arg(emu, 0) as u64;
    let struct_size = emu.maps.read_dword(ptr).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!GetVersionExA 0x{:x} size: {}",
        ptr,
        struct_size
    );
    match struct_size {
        148 => structures::OsVersionInfoA::new().save(ptr, &mut emu.maps),
        156 => structures::OsVersionInfoExA::new().save(ptr, &mut emu.maps),
        _ => {
            log::warn!(
                "GetVersionExA: unexpected dwOSVersionInfoSize {}",
                struct_size
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
            return;
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
