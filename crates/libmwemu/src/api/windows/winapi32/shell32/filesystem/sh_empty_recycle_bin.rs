use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHEmptyRecycleBinA(emu: &mut emu::Emu) {
    sh_empty_recycle_bin(emu, "SHEmptyRecycleBinA");
}

fn sh_empty_recycle_bin(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn SHEmptyRecycleBinW(emu: &mut emu::Emu) {
    sh_empty_recycle_bin(emu, "SHEmptyRecycleBinW");
}
