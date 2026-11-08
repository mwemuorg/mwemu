use crate::emu;

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let value_type = emu.regs().r9;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let data_size = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
