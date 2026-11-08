use crate::emu;
use crate::winapi::helper;

pub fn AllocConsole_api(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "kernel32!AllocConsole arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    let handle = helper::handler_create("allocconsole");
    emu.regs_mut().rax = handle;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
