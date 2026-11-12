use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strstr(emu: &mut emu::Emu) {
    let haystack_ptr = WinApiAbi::arg(emu, 0);
    let needle_ptr = WinApiAbi::arg(emu, 1);
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
