use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BCryptOpenAlgorithmProvider" => BCryptOpenAlgorithmProvider(emu),
        "BCryptGenRandom" => BCryptGenRandom(emu),
        "BCryptCloseAlgorithmProvider" => BCryptCloseAlgorithmProvider(emu),
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

fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let out_handle = arg(emu, 0) as u64;
    let alg_id_ptr = arg(emu, 1) as u64;
    let algo = emu.maps.read_wide_string(alg_id_ptr);
    log_red!(emu, "bcrypt!BCryptOpenAlgorithmProvider algo: {}", algo);
    if out_handle != 0 {
        let hndl = helper::handler_create(&format!("bcrypt://{}", algo));
        emu.maps.write_dword(out_handle, hndl as u32);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptGenRandom(emu: &mut emu::Emu) {
    let buf = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64;
    log_red!(emu, "bcrypt!BCryptGenRandom buf: 0x{:x} len: {}", buf, len);
    for i in 0..len {
        emu.maps
            .write_byte(buf + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}

fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x}",
        handle
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
