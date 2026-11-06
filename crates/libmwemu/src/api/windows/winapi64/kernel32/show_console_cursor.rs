use crate::emu;

pub fn ShowConsoleCursor(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let show = emu.regs().rdx != 0;

    log_red!(
        emu,
        "kernel32!ShowConsoleCursor h:0x{:x} show:{}",
        handle,
        show
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
