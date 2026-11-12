use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn EndUpdateResourceA(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceA");
}

fn end_update_resource(emu: &mut emu::Emu, api: &str) {
    let _hUpdate = WinApiAbi::arg(emu, 0) as u32;
    let _fDiscard = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn EndUpdateResourceW(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceW");
}
