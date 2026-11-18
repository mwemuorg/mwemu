use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSIPLoad(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pg_subject = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let p_sip_dispatch = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptSIPLoad pg_subject:0x{:x} dw_flags:0x{:x} p_sip_dispatch:0x{:x}",
        pg_subject,
        dw_flags,
        p_sip_dispatch
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
