use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn StartDoc(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!StartDoc arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
