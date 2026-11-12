use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strrchr(emu: &mut emu::Emu) {
    let s_ptr = WinApiAbi::arg(emu, 0);
    let c = WinApiAbi::arg(emu, 1) as u32 as u8 as char;
    let s = emu.maps.read_string(s_ptr);
    let found = s.rfind(c);
    let result = match found {
        Some(pos) => s_ptr + pos as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!strrchr '{}' '{}' => 0x{:x}", s, c, result);

    emu.regs_mut().rax = result;
}
