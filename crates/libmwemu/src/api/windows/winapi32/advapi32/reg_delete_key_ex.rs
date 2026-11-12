use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExA", false);
}

fn reg_delete_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub fn RegDeleteKeyExW(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExW", true);
}
