use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetDefaultCommConfigA(emu: &mut emu::Emu) {
    get_default_comm_config(emu, "GetDefaultCommConfigA", false);
}

fn get_default_comm_config(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszName = WinApiAbi::arg(emu, 0) as u32;
    let _lpCC = WinApiAbi::arg(emu, 1) as u32;
    let _lpdwSize = WinApiAbi::arg(emu, 2) as u32;

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
