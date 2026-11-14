use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetMailslotInfo(emu: &mut emu::Emu) {
    let _hMailslot = WinApiAbi::arg(emu, 0) as u32;
    let _lpMaxMessageSize = WinApiAbi::arg(emu, 1) as u32;
    let _lpNextSize = WinApiAbi::arg(emu, 2) as u32;
    let _lpMessageCount = WinApiAbi::arg(emu, 3) as u32;
    let _lpReadTimeout = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!GetMailslotInfo");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
