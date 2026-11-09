use crate::emu;

pub(super) fn crt_wcslen(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let ws = emu.maps.read_wide_string(ptr);
    log_red!(emu, "wincrt!wcslen '{}' => {}", ws, ws.len());
    emu.regs_mut().rax = ws.len() as u64;
}
