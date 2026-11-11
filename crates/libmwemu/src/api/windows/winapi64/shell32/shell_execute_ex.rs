use crate::emu;
use crate::winapi::helper;

pub fn ShellExecuteEx(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!ShellExecuteEx arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ShellExecuteExA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!ShellExecuteExA '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shellexecuteexa");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn ShellExecuteExW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!ShellExecuteExW '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shellexecuteexw");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
