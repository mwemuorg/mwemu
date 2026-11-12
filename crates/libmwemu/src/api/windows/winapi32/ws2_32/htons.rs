use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn htons(emu: &mut emu::Emu) {
    let port: u16 = WinApiAbi::arg(emu, 0) as u16;

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.stack_pop32(false);
    emu.regs_mut().rax = port.to_be() as u64;
}
