use crate::emu;

pub(super) fn crt_strstr(emu: &mut emu::Emu) {
    let haystack = emu.regs().rcx;
    let needle = emu.regs().rdx;
    let h = emu.maps.read_string(haystack);
    let n = emu.maps.read_string(needle);
    let result = match h.find(&n) {
        Some(pos) => haystack + pos as u64,
        None => 0,
    };
    log_red!(emu, "wincrt!strstr '{}' '{}' => 0x{:x}", h, n, result);
    emu.regs_mut().rax = result;
}
