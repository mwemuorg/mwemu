use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn HttpQueryInfoA(emu: &mut emu::Emu) {
    http_query_info(emu, "HttpQueryInfoA", false);
}

pub(super) fn HttpQueryInfoW(emu: &mut emu::Emu) {
    http_query_info(emu, "HttpQueryInfoW", true);
}

fn http_query_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let info_level = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    let buffer_len_ptr = api_try!(emu, call.arg(emu, 3));
    let level_raw = info_level & 0x0000FFFF;
    log_red!(
        emu,
        "wininet!{api} handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );
    if wide {
        let response: Option<&str> = match level_raw {
            1 => Some("text/html"),
            5 => Some("1024"),
            19 => Some("200"),
            21 => Some("HTTP/1.1 200 OK"),
            22 => Some("HTTP/1.1 200 OK\r\n"),
            _ => None,
        };
        match response {
            Some(val) => {
                if buffer != 0 {
                    strings::write_string(emu, buffer, val, wide);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, (val.len() * 2) as u32);
                }
                call.set_return(emu, 1);
            }
            None => {
                call.set_return(emu, 0);
            }
        }
    } else {
        let response: Option<&str> = match level_raw {
            1 => Some("text/html"),            // HTTP_QUERY_CONTENT_TYPE
            5 => Some("1024"),                 // HTTP_QUERY_CONTENT_LENGTH
            19 => Some("200"),                 // HTTP_QUERY_STATUS_CODE
            21 => Some("HTTP/1.1 200 OK"),     // HTTP_QUERY_RAW_HEADERS
            22 => Some("HTTP/1.1 200 OK\r\n"), // HTTP_QUERY_RAW_HEADERS_CRLF
            _ => None,
        };
        match response {
            Some(val) => {
                if buffer != 0 {
                    strings::write_string(emu, buffer, val, wide);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, val.len() as u32);
                }
                call.set_return(emu, 1); // TRUE
            }
            None => {
                log_red!(emu, "  unsupported info level {}", level_raw);
                call.set_return(emu, 0); // FALSE
            }
        }
    }
}
