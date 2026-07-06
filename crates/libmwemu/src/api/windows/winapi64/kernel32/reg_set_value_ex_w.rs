use crate::emu;
use crate::windows::constants;

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let value_name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let value_type = emu.regs().r9;

    // Stack params
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 32).unwrap_or(0);
    let _data_size = emu.maps.read_qword(emu.regs().rsp + 40).unwrap_or(0);

    let value_name = emu.maps.read_wide_string(value_name_ptr);

    log_red!(
        emu,
        "kernel32!RegSetValueExW `{}` type: {} data: 0x{:x}",
        value_name,
        value_type,
        data_ptr
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
