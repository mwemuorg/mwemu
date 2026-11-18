use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn InternetQueryOptionA(emu: &mut emu::Emu) {
    internet_query_option(emu, "InternetQueryOptionA", false);
}

pub(super) fn InternetQueryOptionW(emu: &mut emu::Emu) {
    internet_query_option(emu, "InternetQueryOptionW", true);
}

fn internet_query_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let option = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    let buffer_len_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "wininet!{api} handle: 0x{:x} option: {}",
        handle,
        option
    );
    if wide {
        match option {
            34 => {
                let url = "http://emulated.local/";
                if buffer != 0 {
                    strings::write_string(emu, buffer, url, wide);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, (url.len() * 2) as u32);
                }
                call.set_return(emu, 1);
            }
            37 => {
                if buffer != 0 {
                    emu.maps.write_dword(buffer, 1);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, 4);
                }
                call.set_return(emu, 1);
            }
            _ => {
                call.set_return(emu, 0);
            }
        }
    } else {
        match option {
            34 => {
                // INTERNET_OPTION_URL
                let url = "http://emulated.local/";
                if buffer != 0 {
                    strings::write_string(emu, buffer, url, wide);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, url.len() as u32);
                }
                call.set_return(emu, 1);
            }
            37 => {
                // INTERNET_OPTION_HANDLE_TYPE
                if buffer != 0 {
                    emu.maps.write_dword(buffer, 1); // INTERNET_HANDLE_TYPE_INTERNET
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, 4);
                }
                call.set_return(emu, 1);
            }
            _ => {
                log_red!(emu, "  unsupported option {}", option);
                call.set_return(emu, 0);
            }
        }
    }
}
