use super::allocate_memory;
use crate::emu;

pub(super) fn __p__environ(emu: &mut emu::Emu) {
    // char *** __p__environ(void)
    // Return a pointer to a NULL-terminated environment pointer list (empty env).
    let envp = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__environ alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_qword(envp, 0);
    emu.regs_mut().rax = envp;
}
