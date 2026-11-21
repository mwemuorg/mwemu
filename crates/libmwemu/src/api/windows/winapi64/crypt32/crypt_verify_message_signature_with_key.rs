use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyMessageSignatureWithKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    let arg6 = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptVerifyMessageSignatureWithKey 0x{:x} 0x{:x}",
        arg1,
        arg2
    );
    let out_ptr = api_try!(emu, call.arg(emu, 5));
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
