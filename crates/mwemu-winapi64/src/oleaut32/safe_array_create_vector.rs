use crate::emu;
use crate::winapi::helper;

pub fn SafeArrayCreateVector(emu: &mut emu::Emu) {
    let vt = emu.regs().rcx;
    let lower = emu.regs().rdx;
    let count = emu.regs().r8;
    let hndl = helper::handler_create(&format!("safearray_vec_{}", count));
    log_red!(
        emu,
        "oleaut32!SafeArrayCreateVector vt:{} lower:{} count:{} => 0x{:x}",
        vt,
        lower,
        count,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
