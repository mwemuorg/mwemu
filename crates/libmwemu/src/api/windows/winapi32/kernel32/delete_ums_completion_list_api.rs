use crate::emu;
use crate::winapi::helper;

pub fn DeleteUmsCompletionList_api(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(handle);
    log_red!(
        emu,
        "kernel32!DeleteUmsCompletionList handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
