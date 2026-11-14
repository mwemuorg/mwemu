use crate::emu;

pub fn wcsstr(emu: &mut emu::Emu) {
    let haystack_ptr = emu.regs().rcx;
    let needle_ptr = emu.regs().rdx;
    let haystack = emu.maps.read_wide_string(haystack_ptr);
    let needle = emu.maps.read_wide_string(needle_ptr);
    let result = match haystack.find(&needle) {
        Some(pos) => haystack_ptr + (pos * 2) as u64,
        None => 0,
    };
    log_red!(
        emu,
        "msvcrt!wcsstr '{}' '{}' => 0x{:x}",
        haystack,
        needle,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
