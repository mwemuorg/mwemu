use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RealShellExecuteA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let file_ptr = emu.regs().r8;
    let params_ptr = emu.regs().r9;
    let dir = WinApiAbi::arg(emu, 4);
    let bShowWindow = WinApiAbi::arg(emu, 5);

    let file = emu.maps.read_string(file_ptr);
    let params = emu.maps.read_string(params_ptr);

    log_red!(emu, "shell32!RealShellExecuteA {} {}", file, params);

    emu.set_last_error(0);
    emu.regs_mut().rax = 34;
}

pub fn RealShellExecuteW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "shell32!RealShellExecuteW '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
