use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHGetNewLinkInfo(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "shell32!SHGetNewLinkInfo arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn SHGetNewLinkInfoA(emu: &mut emu::Emu) {
    sh_get_new_link_info(emu, "SHGetNewLinkInfoA");
}

fn sh_get_new_link_info(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn SHGetNewLinkInfoW(emu: &mut emu::Emu) {
    sh_get_new_link_info(emu, "SHGetNewLinkInfoW");
}
