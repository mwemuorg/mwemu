use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg_u32(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let phk_result = api_try!(emu, call.arg(emu, 2));
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!RegCreateKeyA key:0x{:x} sub:'{}'",
        hkey,
        subkey
    );
    let new_key = helper::handler_create("reg_key");
    if phk_result != 0 {
        emu.maps.write_dword(phk_result, new_key as u32);
    }
    call.cleanup_args(emu, 3);
    call.set_return(emu, 0);
}

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!RegCreateKeyW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
