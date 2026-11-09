use crate::emu;

pub fn WSAGetOverlappedResult(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let overlapped_ptr = emu.regs().rdx;
    let transfer_ptr = emu.regs().r8;
    let wait = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!WSAGetOverlappedResult sock:0x{:x} wait:{}",
        sock,
        wait
    );
    if transfer_ptr != 0 {
        emu.maps.write_dword(transfer_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
