use crate::emu;

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let data_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    let value = if reg_type == 1 || reg_type == 2 {
        emu.maps.read_wide_string(data_ptr)
    } else {
        format!("0x{:x}", data_ptr)
    };
    log_red!(
        emu,
        "kernelbase!RegSetValueExW hkey:0x{:x} '{}' type:{} value:'{}' len:{}",
        hkey,
        name,
        reg_type,
        value,
        data_len
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
