use crate::emu;
use crate::winapi::helper;
use crate::windows::peb;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let module_name_ptr = emu.regs().rcx;
    let module_name: String;
    let base;

    if module_name_ptr == 0 {
        module_name = emu.cfg.exe_name.clone();
        base = match peb::peb64::get_module_base(&module_name, emu) {
            Some(b) => b,
            None => helper::handler_create(&module_name),
        };
    } else {
        module_name = emu.maps.read_string(module_name_ptr);

        let mod_mem = match emu
            .maps
            .get_mem2(&module_name.to_lowercase().replace(".dll", ".pe"))
        {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        base = mod_mem.get_base();
    }

    log_red!(
        emu,
        "kernel32!GetModuleHandleA `{}` {:x}",
        module_name,
        base
    );

    emu.regs_mut().rax = base;
}
