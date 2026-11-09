use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn towlower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u32;
    let ch = char::from_u32(c).unwrap_or('\0');
    let lower = ch.to_lowercase().next().unwrap_or(ch);
    let result = lower as u32;
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    log_red!(emu, "msvcrt!towlower '{}' => '{}'", ch, lower);
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
