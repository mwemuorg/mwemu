use crate::emu;
use crate::winapi::helper;

pub(super) fn fopen(emu: &mut emu::Emu) {
    let filepath_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let mode_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if filepath_ptr == 0 || mode_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!fopen null ptrs");
        return;
    }

    let filepath = emu.maps.read_string(filepath_ptr);
    let mode = emu.maps.read_string(mode_ptr);

    log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);

    emu.regs_mut().rax = helper::handler_create(&filepath);
}
