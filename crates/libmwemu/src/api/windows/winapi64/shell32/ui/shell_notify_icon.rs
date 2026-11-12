use crate::emu;
use crate::winapi::common::strings;

pub fn Shell_NotifyIcon(emu: &mut emu::Emu) {
    let dw_message = emu.regs().rcx as u32;
    let lp_data = emu.regs().rdx;
    let tip = if lp_data != 0 {
        emu.maps.read_string(lp_data + 88)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!Shell_NotifyIcon msg:{} tip:'{}' data:0x{:x}",
        dw_message,
        tip,
        lp_data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn Shell_NotifyIconA(emu: &mut emu::Emu) {
    shell_notify_icon(emu, "Shell_NotifyIconA", false);
}

fn shell_notify_icon(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dw_message = emu.regs().rcx as u32;
    let lp_data = emu.regs().rdx;
    let tip = if lp_data != 0 {
        strings::read_text(emu, lp_data + 88, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!{api} msg:{} tip:'{}' data:0x{:x}",
        dw_message,
        tip,
        lp_data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn Shell_NotifyIconW(emu: &mut emu::Emu) {
    shell_notify_icon(emu, "Shell_NotifyIconW", true);
}
