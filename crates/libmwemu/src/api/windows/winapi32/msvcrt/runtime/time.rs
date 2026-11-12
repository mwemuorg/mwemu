use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn time(emu: &mut emu::Emu) {
    let time_ptr = WinApiAbi::arg(emu, 0);
    let tick = emu.tick as u32;
    if time_ptr != 0 {
        emu.maps.write_dword(time_ptr, tick);
    }
    log_red!(emu, "msvcrt!time => {}", tick);

    emu.regs_mut().rax = tick as u64;
}
