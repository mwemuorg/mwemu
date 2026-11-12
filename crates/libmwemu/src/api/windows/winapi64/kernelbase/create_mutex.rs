use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexA");
}

fn create_mutex(emu: &mut emu::Emu, api: &str) {
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
    let hndl = helper::handler_create(&format!("mutex://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn CreateMutexW(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexW");
}
