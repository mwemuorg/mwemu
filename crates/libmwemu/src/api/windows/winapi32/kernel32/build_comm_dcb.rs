use crate::emu;
use crate::windows::constants::errors::*;

pub fn BuildCommDCBA(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBA");
}

fn build_comm_dcb(emu: &mut emu::Emu, api: &str) {
    let _lpDef = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpDCB = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn BuildCommDCBW(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBW");
}
