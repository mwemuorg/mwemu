use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub(super) fn InternetSetOptionA(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionA", false);
}

fn internet_set_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = strings::read_text(emu, buffer, wide);

    log_red!(
        emu,
        "wininet!{api} option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

pub(super) fn InternetSetOptionW(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionW", true);
}
