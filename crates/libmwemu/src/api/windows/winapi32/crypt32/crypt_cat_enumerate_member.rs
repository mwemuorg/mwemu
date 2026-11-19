use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATEnumerateMember(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_catalog = api_try!(emu, call.arg(emu, 0));
    let p_prev_member = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptCATEnumerateMember h_catalog:0x{:x} p_prev_member:0x{:x}",
        h_catalog,
        p_prev_member
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
