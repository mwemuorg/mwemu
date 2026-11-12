use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExA", false);
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExW", true);
}

fn reg_query_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size_ptr = WinApiAbi::arg(emu, 5);
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!{api} `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if data_ptr != 0 {
        strings::write_string(emu, data_ptr, "", wide);
    }
    if wide {
        if data_size_ptr != 0 {
            emu.maps.write_dword(data_size_ptr, 2);
        }
    } else {
        if data_size_ptr != 0 {
            emu.maps.write_dword(data_size_ptr, 1);
        }
    }

    emu.regs_mut().rax = 0;
}
