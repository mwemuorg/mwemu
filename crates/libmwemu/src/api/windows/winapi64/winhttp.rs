use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
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
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
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

pub fn WinHttpOpen(emu: &mut emu::Emu) {
    let agent_ptr = emu.regs().rcx;
    let _access_type = emu.regs().rdx as u32;
    let _proxy_ptr = emu.regs().r8;
    let _proxy_bypass_ptr = emu.regs().r9;

    let agent = if agent_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(agent_ptr)
    };

    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);

    let uri = format!("winhttp://session/{}", agent);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn WinHttpConnect(emu: &mut emu::Emu) {
    let h_session = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8 as u16;

    let server = if server_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(server_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        h_session,
        server,
        port
    );

    if !helper::handler_exist(h_session) {
        log::trace!("\tinvalid session handle");
    }

    let uri = format!("winhttp://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let h_connect = emu.regs().rcx;
    let verb_ptr = emu.regs().rdx;
    let object_ptr = emu.regs().r8;
    let _version_ptr = emu.regs().r9;

    let verb = if verb_ptr == 0 {
        "GET".to_string()
    } else {
        emu.maps.read_wide_string(verb_ptr)
    };

    let object = if object_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(object_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        h_connect,
        verb,
        object
    );

    if !helper::handler_exist(h_connect) {
        log::trace!("\tinvalid connect handle");
    }

    let uri = format!("winhttp://request/{} {}", verb, object);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let h_request = emu.regs().rcx;

    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", h_request);

    if !helper::handler_exist(h_request) {
        log::trace!("\tinvalid request handle");
    }

    emu.regs_mut().rax = 1;
}

pub fn WinHttpReceiveResponse(emu: &mut emu::Emu) {
    let h_request = emu.regs().rcx;

    log_red!(
        emu,
        "winhttp!WinHttpReceiveResponse request: 0x{:x}",
        h_request
    );

    if !helper::handler_exist(h_request) {
        log::trace!("\tinvalid request handle");
    }

    emu.regs_mut().rax = 1;
}

pub fn WinHttpReadData(emu: &mut emu::Emu) {
    let h_request = emu.regs().rcx;
    let _buffer = emu.regs().rdx;
    let _bytes_to_read = emu.regs().r8 as u32;
    let bytes_read_ptr = emu.regs().r9;

    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", h_request);

    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0);
    }

    emu.regs_mut().rax = 1;
}

pub fn WinHttpCloseHandle(emu: &mut emu::Emu) {
    let h_internet = emu.regs().rcx;

    log_red!(emu, "winhttp!WinHttpCloseHandle handle: 0x{:x}", h_internet);

    helper::handler_close(h_internet);
    emu.regs_mut().rax = 1;
}
