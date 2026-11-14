use crate::emu;
use crate::winapi::common::strings;
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
    shell_execute_ex(emu, "ShellExecuteExA", false);
}

pub fn ShellExecuteExW(emu: &mut emu::Emu) {
    shell_execute_ex(emu, "ShellExecuteExW", true);
}

fn shell_execute_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("shellexecuteexw");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("shellexecuteexa");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    }
}
