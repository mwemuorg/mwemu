use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "ShellExecuteA" => ShellExecuteA(emu),
        "ShellExecuteW" => ShellExecuteW(emu),
        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
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
