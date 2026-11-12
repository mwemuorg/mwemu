use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn ShellExecuteEx(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "shell32!ShellExecuteEx arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ShellExecuteExA(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!ShellExecuteExA '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shellexecuteexa");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 1);
}

pub fn ShellExecuteExW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!ShellExecuteExW '{}' arg2:0x{:x}", s, arg2);
    let handle = helper::handler_create("shellexecuteexw");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 1);
}
