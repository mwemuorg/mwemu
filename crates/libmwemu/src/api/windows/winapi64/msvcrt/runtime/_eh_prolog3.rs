use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _EH_prolog3(emu: &mut emu::Emu, api: &str) {
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, emu.regs().rbp);
    }
    log_red!(emu, "msvcrt!{} (64-bit)", api);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
