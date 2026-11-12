use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetDefaultCommConfigA(emu: &mut emu::Emu) {
    get_default_comm_config(emu, "GetDefaultCommConfigA", false);
}

fn get_default_comm_config(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpCC = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpdwSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpszName = strings::read_text(emu, lpszName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetDefaultCommConfigW(emu: &mut emu::Emu) {
    get_default_comm_config(emu, "GetDefaultCommConfigW", true);
}
