use crate::emu;

pub fn DebugBreakProcess(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!DebugBreakProcess handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
