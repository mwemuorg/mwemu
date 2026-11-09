use crate::emu;
use crate::winapi::helper;

/// kernelbase API ////
pub(super) fn GetModuleHandleW(emu: &mut emu::Emu) {
    let mod_name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let mod_name: String;

    if mod_name_ptr == 0 {
        mod_name = "self".to_string();
        emu.regs_mut().rax = match emu.maps.get_base() {
            Some(base) => base,
            None => helper::handler_create(&mod_name),
        }
    } else {
        mod_name = emu.maps.read_wide_string(mod_name_ptr).to_lowercase();
        let mod_mem = match emu.maps.get_mem2(&mod_name) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        emu.regs_mut().rax = mod_mem.get_base();
    }

    log_red!(emu, "kernel32!GetModuleHandleW '{}'", mod_name);

    emu.stack_pop32(false);
}
