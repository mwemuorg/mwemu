use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let id_hook = arg(emu, 0) as i32;
    let lpfn = arg(emu, 1);
    let hmod = arg(emu, 2);
    let thread_id = arg(emu, 3);
    log_red!(
        emu,
        "user32!SetWindowsHookExA idHook={} lpfn=0x{:x} hMod=0x{:x} threadId={}",
        id_hook,
        lpfn,
        hmod,
        thread_id
    );
    emu.regs_mut().rax = helper::handler_create("hook://");
    stdcall_ret(emu, 4);
}
