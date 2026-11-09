use crate::emu;

pub fn EscapeCommFunction(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let func = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!EscapeCommFunction hFile:0x{:x} func:{}",
        h_file,
        func
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
