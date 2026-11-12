use crate::emu;
use crate::winapi::helper;

pub fn OpenSemaphoreA(emu: &mut emu::Emu) {
    open_semaphore(emu, "OpenSemaphoreA");
}

fn open_semaphore(emu: &mut emu::Emu, api: &str) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn OpenSemaphoreW(emu: &mut emu::Emu) {
    open_semaphore(emu, "OpenSemaphoreW");
}
