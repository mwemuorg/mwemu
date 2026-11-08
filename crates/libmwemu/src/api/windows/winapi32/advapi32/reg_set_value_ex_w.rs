use crate::emu;

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let value_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_size = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    let data_str = if value_type == 1 || value_type == 2 {
        emu.maps.read_wide_string(data_ptr)
    } else {
        format!("binary[{}]", data_size)
    };
    log_red!(
        emu,
        "advapi32!RegSetValueExW hkey:0x{:x} '{}' type:{} data:'{}'",
        hkey,
        name,
        value_type,
        data_str
    );
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
