use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let find_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let file = emu.maps.read_wide_string(file_ptr);

    log_red!(emu, "kernel32!FindFirstFileW file: {}", file);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = helper::handler_create(&file);
}
