use crate::emu;
use crate::winapi::common::strings;

pub fn InternetSetDialState(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetSetDialState arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetSetDialStateA(emu: &mut emu::Emu) {
    internet_set_dial_state(emu, "InternetSetDialStateA", false);
}

fn internet_set_dial_state(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetSetDialStateW(emu: &mut emu::Emu) {
    internet_set_dial_state(emu, "InternetSetDialStateW", true);
}
