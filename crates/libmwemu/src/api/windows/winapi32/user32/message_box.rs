use crate::emu;
use crate::winapi::common::strings;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxA", false);
}

fn message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let titleptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let msgptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let msg = strings::read_text(emu, msgptr, wide);
    let title = strings::read_text(emu, titleptr, wide);

    log_red!(emu, "user32!{api} {} {}", title, msg);

    emu.regs_mut().rax = 0;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub(super) fn MessageBoxW(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxW", true);
}
