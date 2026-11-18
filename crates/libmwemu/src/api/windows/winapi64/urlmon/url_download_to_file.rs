use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn URLDownloadToFileA(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileA", false);
}

fn url_download_to_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let pcaller = api_try!(emu, call.arg(emu, 0));
    let sz_url = api_try!(emu, call.arg(emu, 1));
    let sz_filename = api_try!(emu, call.arg(emu, 2));
    let dw_reserved = api_try!(emu, call.arg(emu, 3));
    let lpfn_cb = api_try!(emu, call.arg(emu, 4));

    let url = strings::read_text(emu, sz_url, wide);
    let filename = strings::read_text(emu, sz_filename, wide);

    log_red!(emu, "urlmon!{api} url: {} filename: {}", url, filename);

    call.set_return(emu, 0); // S_OK
}

pub(super) fn URLDownloadToFileW(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileW", true);
}
