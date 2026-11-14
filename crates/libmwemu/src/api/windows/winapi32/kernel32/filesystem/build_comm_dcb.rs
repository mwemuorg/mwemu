use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BuildCommDCBA(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBA");
}

fn build_comm_dcb(emu: &mut emu::Emu, api: &str) {
    let _lpDef = WinApiAbi::arg(emu, 0) as u32;
    let _lpDCB = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn BuildCommDCBW(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBW");
}
