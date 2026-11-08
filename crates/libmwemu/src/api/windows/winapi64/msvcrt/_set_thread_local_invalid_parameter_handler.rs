use super::allocate_memory;
use crate::emu;

pub fn _set_thread_local_invalid_parameter_handler(emu: &mut emu::Emu) {
    let handler = emu.regs().rcx;
    log_red!(
        emu,
        "msvcrt!_set_thread_local_invalid_parameter_handler handler:0x{:x}",
        handler
    );
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, handler);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous handler (NULL)
}
