use crate::emu;
use crate::winapi::helper;

pub fn select(emu: &mut emu::Emu) {
    let nfds = emu.regs().rcx as u32;
    let readfds = emu.regs().rdx;
    let writefds = emu.regs().r8;
    let exceptfds = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!select nfds:{} read:0x{:x} write:0x{:x}",
        nfds,
        readfds,
        writefds
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
