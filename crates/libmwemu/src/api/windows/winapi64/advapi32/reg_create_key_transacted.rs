use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn RegCreateKeyTransactedA(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedA", false);
}

fn reg_create_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegCreateKeyTransactedW(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedW", true);
}
