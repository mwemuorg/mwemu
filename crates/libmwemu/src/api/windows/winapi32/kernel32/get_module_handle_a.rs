use crate::emu;
use crate::winapi::helper;
use crate::windows::peb;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let mod_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetModuleHandleA cannot read mod_name_ptr") as u64;

    let mod_name: String;

    if mod_name_ptr == 0 {
        let caller_rip = emu.regs().rip;
        let exe_name = emu.cfg.exe_name.clone();
        mod_name = match emu.maps.get_addr_name(caller_rip) {
            Some(n) => n,
            None => {
                log::trace!(
                    "kernel32!GetModuleHandleA called from weird place 0x{:x}",
                    caller_rip
                );
                &exe_name
            }
        }
        .to_string();

        let base = match peb::peb64::get_module_base(&mod_name, emu) {
            Some(b) => b,
            None => helper::handler_create(&mod_name),
        };

        log_red!(emu, "kernel32!GetModuleHandleA `{}` {:x}", mod_name, base);

        emu.regs_mut().rax = base;
    } else {
        mod_name = emu.maps.read_string(mod_name_ptr).to_lowercase();
        let mod_mem = match emu.maps.get_mem2(mod_name.as_str()) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };

        emu.regs_mut().rax = mod_mem.get_base();
    }

    log_red!(emu, "kernel32!GetModuleHandleA '{}'", mod_name);

    emu.stack_pop32(false);
}
