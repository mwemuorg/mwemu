use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegGetValueA(emu: &mut emu::Emu) {
    reg_get_value(emu, "RegGetValueA", false);
}

fn reg_get_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    let type_ptr = WinApiAbi::arg(emu, 4);
    let data_ptr = WinApiAbi::arg(emu, 5);
    let size_ptr = WinApiAbi::arg(emu, 6);
    let subkey = if subkey_ptr != 0 {
        strings::read_text(emu, subkey_ptr, wide)
    } else {
        String::new()
    };
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}\\{}' flags:0x{:x}",
        hkey,
        subkey,
        value,
        flags
    );
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.regs_mut().rax = 2; // ERROR_FILE_NOT_FOUND
}

pub fn RegGetValueW(emu: &mut emu::Emu) {
    reg_get_value(emu, "RegGetValueW", true);
}
