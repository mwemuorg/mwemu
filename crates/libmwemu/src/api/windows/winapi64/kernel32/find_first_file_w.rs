use crate::emu;

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let find_data = emu.regs().rdx;

    let file = emu.maps.read_wide_string(file_ptr);
    log_red!(emu, "kernel32!FindFirstFileW file: {}", file);
    emu.regs_mut().rax = 1;
}
