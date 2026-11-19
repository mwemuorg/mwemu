use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptHashSessionKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let hkey = api_try!(emu, call.arg(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let placeholder = vec![0x41u8; 16];
    crypto_state::hash_data(hhash, &placeholder);
    log_red!(
        emu,
        "advapi32!CryptHashSessionKey hash:0x{:x} key:0x{:x}",
        hhash,
        hkey
    );
    call.cleanup_args(emu, 3);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
