use crate::emu;
use crate::winapi::common::strings;

pub(super) fn InternetQueryOptionA(emu: &mut emu::Emu) {
    internet_query_option(emu, "InternetQueryOptionA", false);
}

pub(super) fn InternetQueryOptionW(emu: &mut emu::Emu) {
    internet_query_option(emu, "InternetQueryOptionW", true);
}

fn internet_query_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let handle = emu.regs().rcx;
    let option = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;
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
                emu.regs_mut().rax = 1;
            }
            37 => {
                if buffer != 0 {
                    emu.maps.write_dword(buffer, 1);
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, 4);
                }
                emu.regs_mut().rax = 1;
            }
            _ => {
                emu.regs_mut().rax = 0;
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
                emu.regs_mut().rax = 1;
            }
            37 => {
                // INTERNET_OPTION_HANDLE_TYPE
                if buffer != 0 {
                    emu.maps.write_dword(buffer, 1); // INTERNET_HANDLE_TYPE_INTERNET
                }
                if buffer_len_ptr != 0 {
                    emu.maps.write_dword(buffer_len_ptr, 4);
                }
                emu.regs_mut().rax = 1;
            }
            _ => {
                log_red!(emu, "  unsupported option {}", option);
                emu.regs_mut().rax = 0;
            }
        }
    }
}
