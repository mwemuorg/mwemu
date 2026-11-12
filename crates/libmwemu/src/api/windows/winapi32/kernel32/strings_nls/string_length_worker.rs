use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn StringLengthWorkerA(emu: &mut emu::Emu) {
    string_length_worker(emu, "StringLengthWorkerA");
}

fn string_length_worker(emu: &mut emu::Emu, api: &str) {
    let _psz = WinApiAbi::arg(emu, 0) as u32;
    let _cchMax = WinApiAbi::arg(emu, 1) as u32;
    let _pcchLength = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn StringLengthWorkerW(emu: &mut emu::Emu) {
    string_length_worker(emu, "StringLengthWorkerW");
}
