use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn wcsncpy(emu: &mut emu::Emu) {
    let dst = WinApiAbi::arg(emu, 0);
    let src = WinApiAbi::arg(emu, 1);
    let n = WinApiAbi::arg(emu, 2) as usize;
    let s = emu.maps.read_wide_string(src);
    let truncated: String = s.chars().take(n).collect();
    if dst != 0 {
        emu.maps.write_wide_string(dst, &truncated);
    }
    log_red!(
        emu,
        "msvcrt!wcsncpy dst:0x{:x} '{}' n:{}",
        dst,
        truncated,
        n
    );

    emu.regs_mut().rax = dst;
}
