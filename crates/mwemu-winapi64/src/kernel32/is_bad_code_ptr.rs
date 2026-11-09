use crate::emu;

pub fn IsBadCodePtr(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!IsBadCodePtr ptr:0x{:x}", ptr);
    let result = if ptr != 0 && emu.maps.is_mapped(ptr) {
        0u64
    } else {
        1u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
