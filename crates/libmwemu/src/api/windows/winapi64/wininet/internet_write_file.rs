use crate::emu;

pub fn InternetWriteFile(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buffer = emu.regs().rdx;
    let bytes_to_write = emu.regs().r8 as u32;
    let bytes_written_ptr = emu.regs().r9;
    log_red!(
        emu,
        "wininet!InternetWriteFile handle:0x{:x} buf:0x{:x} sz:{}",
        handle,
        buffer,
        bytes_to_write
    );
    if bytes_written_ptr != 0 {
        emu.maps.write_dword(bytes_written_ptr, bytes_to_write);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
