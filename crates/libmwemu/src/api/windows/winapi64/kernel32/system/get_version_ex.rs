use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetVersionExA(emu: &mut emu::Emu) {
    get_version_ex(emu, "GetVersionExA", false);
}

pub fn GetVersionExW(emu: &mut emu::Emu) {
    get_version_ex(emu, "GetVersionExW", true);
}

fn get_version_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let version_info_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!{api} 0x{:x}", version_info_ptr);
    if wide {
        if api_try!(emu, call.arg(emu, 0)) != 0
            && emu.maps.is_mapped(api_try!(emu, call.arg(emu, 0)))
        {
            emu.maps
                .write_dword(api_try!(emu, call.arg(emu, 0)) + 4, 10);
            emu.maps.write_dword(api_try!(emu, call.arg(emu, 0)) + 8, 0);
            emu.maps
                .write_dword(api_try!(emu, call.arg(emu, 0)) + 12, 22621);
        }
        let os_version_info = structures::OsVersionInfoExW::new();
        os_version_info.save(version_info_ptr, &mut emu.maps);
        emu.set_last_error(0);
    } else {
        if version_info_ptr == 0 || !emu.maps.is_mapped(version_info_ptr) {
            log_red!(emu, "{api}: Invalid pointer");
            emu.set_last_error(0);
            call.set_return(emu, 0);
            return;
        }
        let struct_size = emu.maps.read_dword(version_info_ptr).unwrap_or(0);
        log_red!(emu, "{api}: Structure size: {}", struct_size);
        const OSVERSIONINFOA_SIZE: u32 = 148;
        const OSVERSIONINFOEXA_SIZE: u32 = 284;
        let use_extended = match struct_size {
            OSVERSIONINFOA_SIZE => {
                log_red!(emu, "Using OSVERSIONINFOA (basic)");
                false
            }
            OSVERSIONINFOEXA_SIZE => {
                log_red!(emu, "Using OSVERSIONINFOEXA (extended)");
                true
            }
            _ => {
                log_red!(
                    emu,
                    "{api}: Invalid struct size: {} (expected {} or {})",
                    struct_size,
                    OSVERSIONINFOA_SIZE,
                    OSVERSIONINFOEXA_SIZE
                );
                call.set_return(emu, 0);
                return;
            }
        };
        if use_extended {
            let os_version_info_ex_a = structures::OsVersionInfoExA::new();
            os_version_info_ex_a.save(version_info_ptr, &mut emu.maps);
        } else {
            let os_version_info_a = structures::OsVersionInfoA::new();
            os_version_info_a.save(version_info_ptr, &mut emu.maps);
        }
    }

    call.set_return(emu, 1);
}
