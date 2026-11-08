use crate::emu;

pub fn RegQueryValueExW_api(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let data_size_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
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
