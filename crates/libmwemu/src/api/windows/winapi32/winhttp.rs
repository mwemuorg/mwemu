use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "WinHttpOpen" => WinHttpOpen(emu),
        "WinHttpConnect" => WinHttpConnect(emu),
        "WinHttpOpenRequest" => WinHttpOpenRequest(emu),
        "WinHttpSendRequest" => WinHttpSendRequest(emu),
        "WinHttpReceiveResponse" => WinHttpReceiveResponse(emu),
        "WinHttpReadData" => WinHttpReadData(emu),
        "WinHttpCloseHandle" => WinHttpCloseHandle(emu),
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

fn WinHttpOpen(emu: &mut emu::Emu) {
    let agent_ptr = arg(emu, 0) as u64;
    let agent = if agent_ptr != 0 {
        emu.maps.read_wide_string(agent_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://session/{}", agent));
    stdcall_ret(emu, 5);
}

fn WinHttpConnect(emu: &mut emu::Emu) {
    let session = arg(emu, 0);
    let server_ptr = arg(emu, 1) as u64;
    let port = arg(emu, 2) as u16;
    let server = if server_ptr != 0 {
        emu.maps.read_wide_string(server_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        session,
        server,
        port
    );
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://{}:{}", server, port));
    stdcall_ret(emu, 4);
}

fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let connect = arg(emu, 0);
    let verb_ptr = arg(emu, 1) as u64;
    let object_ptr = arg(emu, 2) as u64;
    let verb = if verb_ptr != 0 {
        emu.maps.read_wide_string(verb_ptr)
    } else {
        "GET".to_string()
    };
    let object = if object_ptr != 0 {
        emu.maps.read_wide_string(object_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        connect,
        verb,
        object
    );
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://request/{} {}", verb, object));
    stdcall_ret(emu, 7);
}

fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let request = arg(emu, 0);
    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", request);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}

fn WinHttpReceiveResponse(emu: &mut emu::Emu) {
    let request = arg(emu, 0);
    log_red!(
        emu,
        "winhttp!WinHttpReceiveResponse request: 0x{:x}",
        request
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn WinHttpReadData(emu: &mut emu::Emu) {
    let request = arg(emu, 0);
    let bytes_read_ptr = arg(emu, 3) as u64;
    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", request);
    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0); // orderly end of data
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn WinHttpCloseHandle(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "winhttp!WinHttpCloseHandle handle: 0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
