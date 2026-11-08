use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!RegCreateKeyW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
