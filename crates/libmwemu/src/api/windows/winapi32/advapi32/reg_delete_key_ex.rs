use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExA", false);
}

fn reg_delete_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub fn RegDeleteKeyExW(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExW", true);
}
