use crate::emu;
use crate::windows::constants::errors::*;

pub fn BuildCommDCBAndTimeoutsA(emu: &mut emu::Emu) {
    build_comm_dcb_and_timeouts(emu, "BuildCommDCBAndTimeoutsA");
}

fn build_comm_dcb_and_timeouts(emu: &mut emu::Emu, api: &str) {
    let _lpDef = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpDCB = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpCommTimeouts = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn BuildCommDCBAndTimeoutsW(emu: &mut emu::Emu) {
    build_comm_dcb_and_timeouts(emu, "BuildCommDCBAndTimeoutsW");
}
