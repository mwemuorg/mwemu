use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHEnumerateUnreadMailAccountsW(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "shell32!SHEnumerateUnreadMailAccountsW '{}'  p1:0x{:x}",
        s,
        p1
    );
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}
