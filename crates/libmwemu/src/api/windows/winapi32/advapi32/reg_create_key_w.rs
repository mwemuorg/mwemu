use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!RegCreateKeyW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.regs_mut().rax = 0;
    for _ in 0..9 {
        emu.stack_pop32(false);
    }
}
