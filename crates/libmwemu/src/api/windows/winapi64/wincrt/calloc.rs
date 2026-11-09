use super::allocate_memory;
use crate::emu;

pub(super) fn calloc(emu: &mut emu::Emu) {
    let nmemb = emu.regs().rcx;
    let size = emu.regs().rdx;
    let total = nmemb.saturating_mul(size);
    if total == 0 {
        emu.regs_mut().rax = 0;
        return;
    }
    let base = match allocate_memory(emu, total) {
        Some(a) => a,
        None => {
            log::error!("wincrt!calloc out of memory (size={})", total);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    for i in 0..total {
        let _ = emu.maps.write_byte(base + i, 0);
    }
    log_red!(
        emu,
        "wincrt!calloc nmemb:{} size:{} =0x{:x}",
        nmemb,
        size,
        base
    );
    emu.regs_mut().rax = base;
}
