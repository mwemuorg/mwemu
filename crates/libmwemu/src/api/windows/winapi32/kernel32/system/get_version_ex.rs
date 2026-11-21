use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetVersionExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let struct_size = emu.maps.read_dword(ptr).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!GetVersionExA 0x{:x} size: {}",
        ptr,
        struct_size
    );
    match struct_size {
        148 => structures::OsVersionInfoA::new().save(ptr, &mut emu.maps),
        156 => structures::OsVersionInfoExA::new().save(ptr, &mut emu.maps),
        _ => {
            log::warn!(
                "GetVersionExA: unexpected dwOSVersionInfoSize {}",
                struct_size
            );
            call.set_return(emu, 0);
            call.cleanup_args(emu, 1);
            return;
        }
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub fn GetVersionExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let version_info_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!GetVersionExW 0x{:x}", version_info_ptr);

    let os_version_info = structures::OsVersionInfoExW::new();
    os_version_info.save(version_info_ptr, &mut emu.maps);

    call.cleanup_args(emu, 1);

    call.set_return(emu, 1);
}
