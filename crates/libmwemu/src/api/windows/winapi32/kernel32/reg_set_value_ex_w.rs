use crate::emu;
use crate::windows::constants;

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    let hKey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let value_name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let value_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_size = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let value_name = emu.maps.read_wide_string(value_name_ptr);

    log_red!(
        emu,
        "kernel32!RegSetValueExW `{}` type: {} data: 0x{:x}",
        value_name,
        value_type,
        data_ptr
    );
    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
