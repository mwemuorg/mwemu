use crate::emu;
use crate::winapi::helper;
use crate::windows::peb;

const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u64 = 0x04;

pub fn GetModuleHandleExW(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let out_ptr = emu.regs().r8;

    let base = if name_ptr == 0 {
        let name = emu.cfg.exe_name.clone();
        peb::peb64::get_module_base(&name, emu).unwrap_or_else(|| helper::handler_create(&name))
    } else if flags & GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS != 0 {
        let pe_name = emu
            .maps
            .get_addr_name(name_ptr)
            .and_then(|map_name| map_name.split('.').next().map(|n| format!("{}.pe", n)));
        match pe_name {
            Some(ref pn) => emu.maps.get_mem2(pn).map(|m| m.get_base()).unwrap_or(0),
            None => 0,
        }
    } else {
        let name = emu.maps.read_wide_string(name_ptr).to_lowercase();
        emu.maps
            .get_mem2(&name.replace(".dll", ".pe"))
            .map(|m| m.get_base())
            .unwrap_or(0)
    };

    log_red!(
        emu,
        "kernel32!GetModuleHandleExW flags: 0x{:x} -> 0x{:x}",
        flags,
        base
    );

    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, base);
    }
    emu.regs_mut().rax = if base != 0 { 1 } else { 0 };
}
