use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegDeleteKeyTransactedA(emu: &mut emu::Emu) {
    reg_delete_key_transacted(emu, "RegDeleteKeyTransactedA", false);
}

fn reg_delete_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}

pub fn RegDeleteKeyTransactedW(emu: &mut emu::Emu) {
    reg_delete_key_transacted(emu, "RegDeleteKeyTransactedW", true);
}
