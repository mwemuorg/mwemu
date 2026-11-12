use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _class = emu.regs().r9;
    let _options = WinApiAbi::arg(emu, 4);
    let _sam = WinApiAbi::arg(emu, 5);
    let _security = WinApiAbi::arg(emu, 6);
    let result_ptr = WinApiAbi::arg(emu, 7);
    let disp_ptr = WinApiAbi::arg(emu, 8);
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    if disp_ptr != 0 {
        emu.maps.write_dword(disp_ptr, 2); // REG_CREATED_NEW_KEY
    }
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}
