use crate::emu;

pub fn WinHttpReadData(emu: &mut emu::Emu) {
    let h_request = emu.regs().rcx;
    let _buffer = emu.regs().rdx;
    let _bytes_to_read = emu.regs().r8 as u32;
    let bytes_read_ptr = emu.regs().r9;

    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", h_request);

    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0);
    }

    emu.regs_mut().rax = 1;
}
