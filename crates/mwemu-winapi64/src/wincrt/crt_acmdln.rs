use super::allocate_memory;
use crate::emu;

pub(super) fn crt_acmdln(emu: &mut emu::Emu, api: &str) {
    let p = match allocate_memory(emu, 64) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    if api.contains("wcmdln") || api.contains("_w") {
        emu.maps.write_wide_string(p, "program.exe");
    } else {
        emu.maps.write_string(p, "program.exe");
    }
    log_red!(emu, "wincrt!{} => 0x{:x}", api, p);
    emu.regs_mut().rax = p;
}
