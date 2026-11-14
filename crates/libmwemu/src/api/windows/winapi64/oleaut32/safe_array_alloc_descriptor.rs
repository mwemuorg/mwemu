use crate::emu;
use crate::winapi::helper;

pub fn SafeArrayAllocDescriptor(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "oleaut32!SafeArrayAllocDescriptor arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("safearrayallocdescriptor");
    emu.regs_mut().rax = handle;
}
