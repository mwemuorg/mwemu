use crate::emu;
use crate::winapi::helper;

pub fn OpenEventA(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventA");
}

fn open_event(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "kernelbase!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let hndl = helper::handler_create(&format!("event://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn OpenEventW(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventW");
}
