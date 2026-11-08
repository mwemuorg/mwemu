use crate::emu;

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "advapi32!RegDeleteKeyW hkey:0x{:x} '{}'", hkey, subkey);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
