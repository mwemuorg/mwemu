use crate::emu;
use crate::winapi::helper;

pub fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let id_hook = emu.regs().rcx as i32;
    let lpfn = emu.regs().rdx;
    let hmod = emu.regs().r8;
    let thread_id = emu.regs().r9 as u32;

    log_red!(
        emu,
        "user32!SetWindowsHookExA idHook={} lpfn=0x{:x} hMod=0x{:x} threadId={}",
        id_hook,
        lpfn,
        hmod,
        thread_id
    );

    emu.regs_mut().rax = helper::handler_create("hook://");
}
