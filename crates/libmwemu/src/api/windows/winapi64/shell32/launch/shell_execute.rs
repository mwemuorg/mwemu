use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteA", false);
}

fn shell_execute(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = WinApiAbi::arg(emu, 4);
    let n_show_cmd = WinApiAbi::arg(emu, 5);

    let operation = if lp_operation != 0 {
        strings::read_text(emu, lp_operation, wide)
    } else {
        "open".to_string()
    };
    let file = strings::read_text(emu, lp_file, wide);
    let params = if lp_parameters != 0 {
        strings::read_text(emu, lp_parameters, wide)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!{api} op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 42; // > 32 means success
}

pub fn ShellExecuteW(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteW", true);
}
