use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegCreateKeyTransactedA(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedA", false);
}

fn reg_create_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 11);
}

pub fn RegCreateKeyTransactedW(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedW", true);
}
