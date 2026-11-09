use crate::emu;
use crate::winapi::helper;

pub fn SafeArrayCreate(emu: &mut emu::Emu) {
    let vt = emu.regs().rcx;
    let dims = emu.regs().rdx;
    let bounds = emu.regs().r8;
    let hndl = helper::handler_create(&format!("safearray_vt{}", vt));
    log_red!(
        emu,
        "oleaut32!SafeArrayCreate vt:{} dims:{} => 0x{:x}",
        vt,
        dims,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
