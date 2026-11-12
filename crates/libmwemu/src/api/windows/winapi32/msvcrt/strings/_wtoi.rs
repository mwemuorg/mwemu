use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _wtoi(emu: &mut emu::Emu) {
    let ptr = WinApiAbi::arg(emu, 0);
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!_wtoi '{}' => {}", ws, val);
    emu.regs_mut().rax = val as u64;
}
