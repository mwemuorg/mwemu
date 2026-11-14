use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn OpenJobObjectA(emu: &mut emu::Emu) {
    open_job_object(emu, "OpenJobObjectA", false);
}

fn open_job_object(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _dwDesiredAccess = WinApiAbi::arg(emu, 0) as u32;
    let _bInheritHandle = WinApiAbi::arg(emu, 1) as u32;
    let lpName = WinApiAbi::arg(emu, 2) as u32;

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn OpenJobObjectW(emu: &mut emu::Emu) {
    open_job_object(emu, "OpenJobObjectW", true);
}
