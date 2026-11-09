use crate::emu;

pub fn StringLengthWorkerA(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;

    let s = emu.maps.read_string(psz);
    let slen = s.len();

    log_red!(
        emu,
        "kernel32!StringLengthWorkerA psz:0x{:x} len:{}",
        psz,
        slen
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = slen as u64;
}
