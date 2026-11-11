use crate::emu;
use crate::winapi::helper;

pub(super) fn InternetSetOptionA(emu: &mut emu::Emu) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionA option: 0x{:x} buff: {{{}}} {}",
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
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_wide_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionW option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}
