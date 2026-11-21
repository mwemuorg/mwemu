use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    reg_create_key(emu, "RegCreateKeyA", false);
}

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    reg_create_key(emu, "RegCreateKeyW", true);
}

fn reg_create_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    if wide {
        let subkey = strings::read_text(emu, subkey_ptr, wide);
        let hndl = helper::handler_create(&format!("regkey_{}", subkey));
        log_red!(
            emu,
            "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
            hkey,
            subkey,
            hndl
        );
        emu.set_last_error(0);
    } else {
        let phk_result = api_try!(emu, call.arg(emu, 2));
        let subkey = if subkey_ptr != 0 {
            strings::read_text(emu, subkey_ptr, wide)
        } else {
            String::new()
        };
        log_red!(emu, "advapi32!{api} key:0x{:x} sub:'{}'", hkey, subkey);
        let new_key = helper::handler_create("reg_key");
        if phk_result != 0 {
            emu.maps.write_dword(phk_result, new_key as u32);
        }
    }

    call.set_return(emu, 0);
}
