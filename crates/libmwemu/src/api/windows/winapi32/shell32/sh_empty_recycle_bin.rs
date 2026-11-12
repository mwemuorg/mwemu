use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHEmptyRecycleBinA(emu: &mut emu::Emu) {
    sh_empty_recycle_bin(emu, "SHEmptyRecycleBinA");
}

fn sh_empty_recycle_bin(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
