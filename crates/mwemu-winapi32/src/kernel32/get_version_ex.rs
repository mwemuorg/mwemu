use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
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

pub fn GetVersionExW(emu: &mut emu::Emu) {
    let version_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!GetVersionExW 0x{:x}", version_info_ptr);

    let os_version_info = structures::OsVersionInfoExW::new();
    os_version_info.save(version_info_ptr, &mut emu.maps);

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
