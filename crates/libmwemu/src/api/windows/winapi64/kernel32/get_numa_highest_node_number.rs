use crate::emu;

pub fn GetNumaHighestNodeNumber(emu: &mut emu::Emu) {
    let out_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetNumaHighestNodeNumber ptr:0x{:x}", out_ptr);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
