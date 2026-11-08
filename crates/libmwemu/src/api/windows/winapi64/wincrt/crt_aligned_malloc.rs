use super::allocate_memory;
use crate::emu;

pub(super) fn crt_aligned_malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    let alignment = emu.regs().rdx;
    let base = allocate_memory(emu, size + alignment).unwrap_or(0);
    let aligned = if base == 0 {
        0
    } else {
        (base + alignment - 1) & !(alignment - 1)
    };
    log_red!(
        emu,
        "wincrt!_aligned_malloc size:{} align:{} =0x{:x}",
        size,
        alignment,
        aligned
    );
    emu.regs_mut().rax = aligned;
}
