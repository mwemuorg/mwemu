use crate::emu;

pub fn GetOverlappedResult(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let overlapped = emu.regs().rdx;
    let bytes_transferred = emu.regs().r8;
    let b_wait = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetOverlappedResult h:0x{:x} overlapped:0x{:x} bytes:0x{:x} wait:{}",
        h_file,
        overlapped,
        bytes_transferred,
        b_wait
    );
    if bytes_transferred != 0 {
        emu.maps.write_dword(bytes_transferred, 0);
    }
    emu.regs_mut().rax = 1;
}
