use crate::emu;
use crate::windows::constants::errors::*;

pub fn EndUpdateResourceA(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceA");
}

fn end_update_resource(emu: &mut emu::Emu, api: &str) {
    let _hUpdate = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _fDiscard = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn EndUpdateResourceW(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceW");
}
