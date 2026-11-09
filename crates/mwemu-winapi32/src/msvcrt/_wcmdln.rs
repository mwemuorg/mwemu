use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn _wcmdln(emu: &mut emu::Emu, addr: u32, api: &str) {
    let addr = allocate_memory(emu, 64).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_wide_string(addr, "mwemu.exe");
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
    emu.regs_mut().rax = addr;
}
