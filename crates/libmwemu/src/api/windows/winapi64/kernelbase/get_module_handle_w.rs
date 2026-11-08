use crate::emu;
use crate::winapi::helper;

pub fn GetModuleHandleW(emu: &mut emu::Emu) {
    let mod_name_ptr = emu.regs().rcx;
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
                log_red!(emu, "kernelbase!GetModuleHandleW '{}' not found", mod_name);
                return;
            }
        };
        emu.regs_mut().rax = mod_mem.get_base();
    }
    log_red!(
        emu,
        "kernelbase!GetModuleHandleW '{}' =0x{:x}",
        mod_name,
        emu.regs().rax
    );
}
