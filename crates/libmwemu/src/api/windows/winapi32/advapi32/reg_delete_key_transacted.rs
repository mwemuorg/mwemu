use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegDeleteKeyTransactedA(emu: &mut emu::Emu) {
    reg_delete_key_transacted(emu, "RegDeleteKeyTransactedA", false);
}

fn reg_delete_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}

pub fn RegDeleteKeyTransactedW(emu: &mut emu::Emu) {
    reg_delete_key_transacted(emu, "RegDeleteKeyTransactedW", true);
}
