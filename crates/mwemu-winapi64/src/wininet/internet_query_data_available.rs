use crate::emu;

pub fn InternetQueryDataAvailable(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let bytes_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetQueryDataAvailable handle:0x{:x}",
        handle
    );
    if bytes_ptr != 0 {
        emu.maps.write_dword(bytes_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
