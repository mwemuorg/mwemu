use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER};
pub fn CryptGenRandom(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let provider = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg_u32(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let error = if emu.crypto.name(provider) != "cryptctx://" {
        Some(ERROR_INVALID_HANDLE)
    } else if !emu.fill_random(output, len.into()) {
        Some(ERROR_INVALID_PARAMETER)
    } else {
        None
    };
    log_red!(emu, "CryptGenRandom provider={provider:#x} len={len}");
    if let Some(error) = error {
        emu.set_last_error(error);
    }
    call.set_return(emu, error.is_none().into());
    call.cleanup_args(emu, 3);
}
