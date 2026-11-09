use crate::emu;

pub fn GetCommTimeouts(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let timeouts_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCommTimeouts h:0x{:x} ptr:0x{:x}",
        h_file,
        timeouts_ptr
    );
    if timeouts_ptr != 0 {
        for i in 0..5u64 {
            emu.maps.write_dword(timeouts_ptr + i * 4, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
