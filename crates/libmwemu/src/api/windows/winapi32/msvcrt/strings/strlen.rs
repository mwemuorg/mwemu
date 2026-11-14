use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strlen(emu: &mut emu::Emu) {
    let s_ptr = WinApiAbi::arg(emu, 0);
    let s = emu.maps.read_string(s_ptr);
    let len = s.len();
    log_red!(emu, "msvcrt!strlen '{}' => {}", s, len);

    emu.regs_mut().rax = len as u64;
}
