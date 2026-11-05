use crate::emu;

pub fn InitializeEnclave(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let lp_address = emu.regs().rdx;
    let lp_info = emu.regs().r8;
    let info_length = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!InitializeEnclave hProc:0x{:x} addr:0x{:x} info:0x{:x} len:{}",
        h_process,
        lp_address,
        lp_info,
        info_length
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
