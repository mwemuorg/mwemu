use super::allocate_memory;
use crate::emu;

pub(super) fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let env_ptr = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    emu.maps.write_qword(env_ptr, 0);
    log_red!(
        emu,
        "wincrt!_get_initial_narrow_environment => 0x{:x}",
        env_ptr
    );
    emu.regs_mut().rax = env_ptr;
}
