use crate::emu;

pub fn GetDllDirectoryA(emu: &mut emu::Emu) {
    let buf_len = emu.regs().rcx as u32;
    let buf_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetDllDirectoryA len:{} ptr:0x{:x}",
        buf_len,
        buf_ptr
    );
    if buf_ptr != 0 && buf_len >= 1 {
        emu.maps.write_byte(buf_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
