use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn internet_crack_url_a(emu: &mut emu::Emu) {
    internet_crack_url(emu, "InternetCrackUrlA", false);
}

fn internet_crack_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let _url_len = api_try!(emu, call.arg_u32(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let _components = api_try!(emu, call.arg_u32(emu, 3));

    let url = strings::read_text(emu, url_ptr, wide);

    log_red!(emu, "wininet!{api} url: `{}`", url);

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

pub(super) fn internet_crack_url_w(emu: &mut emu::Emu) {
    internet_crack_url(emu, "InternetCrackUrlW", true);
}
