use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn EnumerateLocalComputerNamesA(emu: &mut emu::Emu) {
    enumerate_local_computer_names(emu, "EnumerateLocalComputerNamesA", false);
}

pub fn EnumerateLocalComputerNamesW(emu: &mut emu::Emu) {
    enumerate_local_computer_names(emu, "EnumerateLocalComputerNamesW", true);
}

fn enumerate_local_computer_names(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;
    if wide {
        let _lpBuffer = WinApiAbi::arg(emu, 1) as u32;
        let _a3 = WinApiAbi::arg(emu, 2) as u32;
        log_red!(emu, "kernel32!{api}");
    } else {
        let lpMultiByteStr = WinApiAbi::arg(emu, 1) as u32;
        let _a3 = WinApiAbi::arg(emu, 2) as u32;
        let lpMultiByteStr = strings::read_text(emu, lpMultiByteStr as u64, wide);
        log_red!(emu, "kernel32!{api} {}", lpMultiByteStr);
    }

    emu.regs_mut().rax = ERROR_SUCCESS;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
