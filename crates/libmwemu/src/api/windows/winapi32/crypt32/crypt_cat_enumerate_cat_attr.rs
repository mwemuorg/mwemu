use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATEnumerateCatAttr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_catalog = api_try!(emu, call.arg(emu, 0));
    let p_prev_attr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptCATEnumerateCatAttr h_catalog:0x{:x} p_prev_attr:0x{:x}",
        h_catalog,
        p_prev_attr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
