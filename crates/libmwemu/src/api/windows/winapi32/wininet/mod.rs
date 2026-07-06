use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

mod request;
mod session;
mod teardown;
mod url;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InternetOpenA" => session::internet_open_a(emu),
        "InternetOpenW" => session::internet_open_w(emu),
        "InternetConnectA" => session::internet_connect_a(emu),
        "InternetConnectW" => session::internet_connect_w(emu),
        "HttpOpenRequestA" => request::http_open_request_a(emu),
        "HttpOpenRequestW" => request::http_open_request_w(emu),
        "InternetSetOptionA" => request::internet_set_option_a(emu),
        "InternetSetOptionW" => request::internet_set_option_w(emu),
        "HttpSendRequestA" => request::http_send_request_a(emu),
        "HttpSendRequestW" => request::http_send_request_w(emu),
        "InternetReadFile" => teardown::internet_read_file(emu),
        "InternetErrorDlg" => teardown::internet_error_dlg(emu),
        "HttpQueryInfoA" => request::http_query_info_a(emu),
        "InternetCloseHandle" => teardown::internet_close_handle(emu),
        "InternetCrackUrlA" => url::internet_crack_url_a(emu),
        "InternetCrackUrlW" => url::internet_crack_url_w(emu),
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

pub(super) fn pop_stack32(emu: &mut emu::Emu, count: usize) {
    for _ in 0..count {
        emu.stack_pop32(false);
    }
}

pub(super) fn read_stack_dword(emu: &mut emu::Emu, offset: u64, what: &str) -> u32 {
    emu.maps
        .read_dword(emu.regs().get_esp() + offset)
        .expect(what)
}

pub(super) fn read_ansi_string_if_present(emu: &mut emu::Emu, ptr: u64) -> String {
    if ptr == 0 {
        String::new()
    } else {
        emu.maps.read_string(ptr)
    }
}

pub(super) fn read_wide_string_if_present(emu: &mut emu::Emu, ptr: u64) -> String {
    if ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(ptr)
    }
}
