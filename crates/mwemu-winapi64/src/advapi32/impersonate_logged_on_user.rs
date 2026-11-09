use crate::emu;
use crate::winapi::helper;

pub fn ImpersonateLoggedOnUser(emu: &mut emu::Emu) {
    let htoken = emu.regs().rcx;
    log_red!(
        emu,
        "advapi32!ImpersonateLoggedOnUser hToken:0x{:x}",
        htoken
    );
    let result = if helper::handler_exist(htoken) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
