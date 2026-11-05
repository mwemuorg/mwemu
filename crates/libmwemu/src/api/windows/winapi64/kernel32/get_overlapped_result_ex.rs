use crate::emu;

pub fn GetOverlappedResultEx(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let overlapped = emu.regs().rdx;
    let bytes_transferred = emu.regs().r8;
    let timeout = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetOverlappedResultEx h:0x{:x} overlapped:0x{:x} bytes:0x{:x} timeout:{}",
        h_file,
        overlapped,
        bytes_transferred,
        timeout
    );
    if bytes_transferred != 0 {
        emu.maps.write_dword(bytes_transferred, 0);
    }
    emu.regs_mut().rax = 1;
}
