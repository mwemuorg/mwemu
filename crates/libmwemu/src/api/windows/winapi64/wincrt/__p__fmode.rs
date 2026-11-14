use super::allocate_memory;
use crate::emu;

pub(super) fn __p__fmode(emu: &mut emu::Emu) {
    // int * __p__fmode(void)
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__fmode alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}
