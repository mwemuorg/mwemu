use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _time32(emu: &mut emu::Emu) {
    let timer_ptr = WinApiAbi::arg(emu, 0);
    let time_val = emu.tick as u32;
    if timer_ptr != 0 {
        emu.maps.write_dword(timer_ptr, time_val);
    }
    log_red!(emu, "msvcrt!_time32 => {}", time_val);

    emu.regs_mut().rax = time_val as u64;
}
