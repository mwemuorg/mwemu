use crate::emu;

pub(super) fn DefWindowProcA(emu: &mut emu::Emu) {
    def_window_proc(emu, "DefWindowProcA");
}

fn def_window_proc(emu: &mut emu::Emu, api: &str) {
    log_red!(
        emu,
        "user32!{api} hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn DefWindowProcW(emu: &mut emu::Emu) {
    def_window_proc(emu, "DefWindowProcW");
}
