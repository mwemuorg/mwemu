use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteA", false);
}

fn shell_execute(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let lp_operation = WinApiAbi::arg(emu, 1);
    let lp_file = WinApiAbi::arg(emu, 2);
    let lp_parameters = WinApiAbi::arg(emu, 3);
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

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 42; // > 32 means success
}

pub fn ShellExecuteW(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteW", true);
}
