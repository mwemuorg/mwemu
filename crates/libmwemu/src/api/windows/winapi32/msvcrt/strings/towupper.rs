use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn towupper(emu: &mut emu::Emu) {
    let c = WinApiAbi::arg(emu, 0) as u32;
    let ch = char::from_u32(c).unwrap_or('\0');
    let upper = ch.to_uppercase().next().unwrap_or(ch);
    log_red!(emu, "msvcrt!towupper '{}' => '{}'", ch, upper);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = upper as u32 as u64;
}
