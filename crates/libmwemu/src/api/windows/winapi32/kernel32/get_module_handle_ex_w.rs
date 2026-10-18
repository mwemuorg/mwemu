use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::peb::peb32;

pub fn GetModuleHandleExW(emu: &mut emu::Emu) {
    const FROM_ADDRESS: u32 = 4; // GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
    let flags = arg(emu, 0);
    let name_ptr = arg(emu, 1) as u64;
    let out_ptr = arg(emu, 2) as u64;
    let base = if name_ptr == 0 {
        let exe = emu.cfg.exe_name.clone();
        peb32::get_module_base(&exe, emu).unwrap_or(emu.base)
    } else if flags & FROM_ADDRESS != 0 {
        emu.maps
            .get_addr_name(name_ptr)
            .and_then(|map| map.split('.').next().map(|n| format!("{}.pe", n)))
            .and_then(|pe| emu.maps.get_mem2(&pe).map(|m| m.get_base()))
            .unwrap_or(0)
    } else {
        let name = emu.maps.read_wide_string(name_ptr).to_lowercase();
        peb32::get_module_base(&name, emu).unwrap_or(0)
    };
    log_red!(
        emu,
        "kernel32!GetModuleHandleExW flags: 0x{:x} -> 0x{:x}",
        flags,
        base
    );
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, base as u32);
    }
    emu.regs_mut().rax = (base != 0) as u64;
    stdcall_ret(emu, 3);
}
