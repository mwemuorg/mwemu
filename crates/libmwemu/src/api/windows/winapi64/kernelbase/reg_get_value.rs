use crate::emu;

pub fn RegGetValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    let type_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let size_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegGetValueA hkey:0x{:x} '{}\\{}' flags:0x{:x}",
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
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    let type_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let size_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_wide_string(subkey_ptr)
    } else {
        String::new()
    };
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegGetValueW hkey:0x{:x} '{}\\{}' flags:0x{:x}",
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
