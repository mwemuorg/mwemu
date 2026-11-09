use super::allocate_memory;
use crate::emu;

pub(super) fn __p__commode(emu: &mut emu::Emu) {
    // int * __p__commode(void)
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__commode alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}
