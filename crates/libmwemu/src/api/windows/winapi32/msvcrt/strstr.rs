use crate::emu;

pub fn strstr(emu: &mut emu::Emu) {
    let haystack_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let needle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let haystack = emu.maps.read_string(haystack_ptr);
    let needle = emu.maps.read_string(needle_ptr);
    let found = haystack.find(&needle);
    let result = match found {
        Some(pos) => haystack_ptr + pos as u64,
        None => 0,
    };
    log_red!(
        emu,
        "msvcrt!strstr '{}' '{}' => 0x{:x}",
        haystack,
        needle,
        result
    );

    emu.regs_mut().rax = result;
}
