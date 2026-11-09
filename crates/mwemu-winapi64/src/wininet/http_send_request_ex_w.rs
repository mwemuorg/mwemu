use crate::emu;

pub fn HttpSendRequestExW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buffers_in = emu.regs().rdx;
    let buffers_out = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "wininet!HttpSendRequestExW handle:0x{:x} buf_in:0x{:x} flags:0x{:x}",
        handle,
        buffers_in,
        flags
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
