use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn setsockopt(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let level = WinApiAbi::arg(emu, 1);
    let optname = WinApiAbi::arg(emu, 2);
    let optval = WinApiAbi::arg(emu, 3);
    let optlen = WinApiAbi::arg(emu, 4);

    let val = emu.maps.read_dword(optval).unwrap_or_default();

    log_red!(
        emu,
        "ws2_32!setsockopt  lvl: {} opt: {} val: {}",
        level,
        optname,
        val
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
