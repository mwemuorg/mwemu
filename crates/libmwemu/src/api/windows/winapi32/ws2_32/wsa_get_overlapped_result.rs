use crate::emu;

pub fn WSAGetOverlappedResult(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let transfer_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let wait = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
