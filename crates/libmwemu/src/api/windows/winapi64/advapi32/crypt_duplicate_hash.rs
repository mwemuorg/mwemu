use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDuplicateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let _reserved = api_try!(emu, call.arg(emu, 1));
    let _flags = api_try!(emu, call.arg(emu, 2));
    let ph_hash = api_try!(emu, call.arg(emu, 3));
    let new_handle = helper::handler_create("crypt_hash_dup");
    log_red!(
        emu,
        "advapi32!CryptDuplicateHash src:0x{:x} => 0x{:x}",
        hhash,
        new_handle
    );
    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, new_handle);
    }
    call.set_return(emu, 1);
}
