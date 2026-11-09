use crate::emu;

pub fn WPUCompleteOverlappedRequest(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let bytes = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
        sock,
        error,
        bytes
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
