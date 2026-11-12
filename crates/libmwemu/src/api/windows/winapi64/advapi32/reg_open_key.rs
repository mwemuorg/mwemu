use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyA", false);
}

fn reg_open_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let result_ptr = emu.regs().r8;
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("registry://{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
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

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyW", true);
}
