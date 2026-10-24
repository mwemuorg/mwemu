use crate::emu;
use crate::winapi::helper;
use crate::windows::peb;

pub fn GetModuleHandleW(emu: &mut emu::Emu) {
    let module_name_ptr = emu.regs().rcx;
    let (module_name, base) = if module_name_ptr == 0 {
        let name = emu.cfg.exe_name.clone();
        let b = match peb::peb64::get_module_base(&name, emu) {
            Some(b) => b,
            None => helper::handler_create(&name),
        };
        (name, b)
    } else {
        let name = emu.maps.read_wide_string(module_name_ptr).to_lowercase();
        let mod_mem = match emu
            .maps
            .get_mem2(&name.to_lowercase().replace(".dll", ".pe"))
        {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        (name, mod_mem.get_base())
    };

    log_red!(
        emu,
        "kernel32!GetModuleHandleW `{}` 0x{:x}",
        module_name,
        base
    );

    emu.regs_mut().rax = base;
}
