use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size_ptr = WinApiAbi::arg(emu, 5);
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExA `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if data_ptr != 0 {
        emu.maps.write_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 1);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size_ptr = WinApiAbi::arg(emu, 5);
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExW `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if data_ptr != 0 {
        emu.maps.write_wide_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 2);
    }
    emu.regs_mut().rax = 0;
}
