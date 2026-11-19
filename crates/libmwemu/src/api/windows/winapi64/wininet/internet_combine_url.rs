use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetCombineUrlA(emu: &mut emu::Emu) {
    internet_combine_url(emu, "InternetCombineUrlA", false);
}

fn internet_combine_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let base_ptr = api_try!(emu, call.arg(emu, 0));
    let relative_ptr = api_try!(emu, call.arg(emu, 1));
    let combined_ptr = api_try!(emu, call.arg(emu, 2));
    let combined_len_ptr = api_try!(emu, call.arg(emu, 3));
    let base = if base_ptr != 0 {
        strings::read_text(emu, base_ptr, wide)
    } else {
        String::new()
    };
    let relative = if relative_ptr != 0 {
        strings::read_text(emu, relative_ptr, wide)
    } else {
        String::new()
    };
    let combined = if relative.starts_with("http://") || relative.starts_with("https://") {
        relative.clone()
    } else {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            relative.trim_start_matches('/')
        )
    };
    log_red!(
        emu,
        "wininet!{api} base:'{}' rel:'{}' => '{}'",
        base,
        relative,
        combined
    );
    if combined_ptr != 0 {
        strings::write_string(emu, combined_ptr, &combined, wide);
    }
    if combined_len_ptr != 0 {
        emu.maps
            .write_dword(combined_len_ptr, combined.len() as u32);
    }
    call.set_return(emu, 1);
}

pub fn InternetCombineUrlW(emu: &mut emu::Emu) {
    internet_combine_url(emu, "InternetCombineUrlW", true);
}
