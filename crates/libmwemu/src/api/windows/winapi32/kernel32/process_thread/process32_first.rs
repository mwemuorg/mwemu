use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn Process32First(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let lppe = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(
        emu,
        "kernel32!Process32First hndl: 0x{:x} lppe: 0x{:x}",
        handle,
        lppe
    );
    if !helper::handler_exist(handle) {
        call.set_return(emu, 0);
    } else {
        // PROCESSENTRY32: szExeFile at +36 on 32-bit (ULONG_PTR th32DefaultHeapID is 4 bytes)
        emu.maps.write_dword(lppe + 8, 4); // th32ProcessID
        emu.maps.write_string(lppe + 36, "smss.exe");
        call.set_return(emu, 1);
    }
    call.cleanup_args(emu, 2);
}

pub fn Process32FirstW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hSnapshot = api_try!(emu, call.arg_u32(emu, 0));
    let _lppe = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!Process32FirstW");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
