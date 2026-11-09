use crate::emu;
use crate::winapi::helper;

pub fn OpenRegStream(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "shell32!OpenRegStream hkey:0x{:x} '{}'", hkey, subkey);
    let hndl = helper::handler_create("shell32://OpenRegStream");
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
