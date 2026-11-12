use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn CreateSemaphoreExA(emu: &mut Emu) {
    invoke(emu, false, "CreateSemaphoreExA");
}

pub fn CreateSemaphoreExW(emu: &mut Emu) {
    invoke(emu, true, "CreateSemaphoreExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let initial = emu.regs().rdx as i32;
    let maximum = emu.regs().r8 as i32;
    let name = strings::read_text(emu, emu.regs().r9, wide);
    log_red!(
        emu,
        "kernel32!{} initial:{} maximum:{} name:'{}'",
        api,
        initial,
        maximum,
        name
    );
    if initial < 0 || maximum <= 0 || initial > maximum {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = helper::handler_create(&name);
    }
}
