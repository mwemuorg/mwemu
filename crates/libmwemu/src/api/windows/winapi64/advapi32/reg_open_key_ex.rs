use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    reg_open_key_ex(emu, "RegOpenKeyExA", false);
}

fn reg_open_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _sam = emu.regs().r9;
    let result_ptr = WinApiAbi::arg(emu, 4);
    let subkey = if subkey_ptr != 0 {
        strings::read_text(emu, subkey_ptr, wide)
    } else {
        String::new()
    };
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
    emu.regs_mut().rax = 0;
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    reg_open_key_ex(emu, "RegOpenKeyExW", true);
}
