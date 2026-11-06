use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "ShellExecuteA" => ShellExecuteA(emu),
        "ShellExecuteW" => ShellExecuteW(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn ShellExecuteA(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let lp_operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let lp_file = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let lp_parameters = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lp_directory = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let n_show_cmd = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let operation = if lp_operation != 0 {
        emu.maps.read_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteA op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 42; // > 32 means success
}

fn ShellExecuteW(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let lp_operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let lp_file = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let lp_parameters = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lp_directory = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let n_show_cmd = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let operation = if lp_operation != 0 {
        emu.maps.read_wide_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_wide_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_wide_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteW op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 42; // > 32 means success
}
