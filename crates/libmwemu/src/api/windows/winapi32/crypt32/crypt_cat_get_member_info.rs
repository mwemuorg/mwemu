use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATGetMemberInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_catalog = api_try!(emu, call.arg(emu, 0));
    let pwsz_ref_tag = api_try!(emu, call.arg(emu, 1));
    let p_cat_member = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATGetMemberInfo h_catalog:0x{:x} pwsz_ref_tag:0x{:x} p_cat_member:0x{:x}",
        h_catalog,
        pwsz_ref_tag,
        p_cat_member
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
