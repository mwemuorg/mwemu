use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn wcstombs(emu: &mut emu::Emu, api: &str) {
    let dst = WinApiAbi::arg(emu, 0);
    let src = WinApiAbi::arg(emu, 1);
    let n = WinApiAbi::arg(emu, 2) as usize;
    let ws = emu.maps.read_wide_string(src);
    let copy_len = ws.len().min(n);
    for (i, b) in ws.as_bytes()[..copy_len].iter().enumerate() {
        emu.maps.write_byte(dst + i as u64, *b);
    }
    if copy_len < n {
        emu.maps.write_byte(dst + copy_len as u64, 0);
    }
    log_red!(emu, "msvcrt!{} '{}' => {}", api, ws, copy_len);
    emu.regs_mut().rax = copy_len as u64;
}
