use crate::emu;
use crate::winapi::helper;

pub fn SHGetFileInfo(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHGetFileInfo h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHGetFileInfoA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!SHGetFileInfoA '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shgetfileinfoa");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn SHGetFileInfoW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!SHGetFileInfoW '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shgetfileinfow");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
