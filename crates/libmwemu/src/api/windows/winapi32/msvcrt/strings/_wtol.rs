use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _wtol(emu: &mut emu::Emu) {
    let ptr = WinApiAbi::arg(emu, 0);
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!_wtol '{}' => {}", ws, val);
    emu.regs_mut().rax = val as u64;
}
