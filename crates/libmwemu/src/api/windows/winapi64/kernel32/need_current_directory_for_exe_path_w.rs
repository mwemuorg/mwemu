use crate::emu;

pub fn NeedCurrentDirectoryForExePathW(emu: &mut emu::Emu) {
    let exe_name_ptr = emu.regs().rcx;
    let exe_name = emu.maps.read_wide_string(exe_name_ptr);
    log_red!(
        emu,
        "kernel32!NeedCurrentDirectoryForExePathW '{}'",
        exe_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
