use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;
//use crate::endpoint;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PathFileExistsA" => PathFileExistsA(emu),
        "PathFileExistsW" => PathFileExistsW(emu),
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

fn PathFileExistsA(emu: &mut emu::Emu) {
    let ptr_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let path = emu.maps.read_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsA path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}

fn PathFileExistsW(emu: &mut emu::Emu) {
    let ptr_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let path = emu.maps.read_wide_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsW path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}
