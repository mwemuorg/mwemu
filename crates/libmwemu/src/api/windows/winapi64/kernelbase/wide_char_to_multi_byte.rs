use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let code_page = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let wc_str = api_try!(emu, call.arg(emu, 2));
    let wc_len = api_try!(emu, call.arg_i32(emu, 3));
    let mb_str = api_try!(emu, call.arg(emu, 4));
    let mb_len = api_try!(emu, call.arg_i32(emu, 5));
    let s = emu.maps.read_wide_string(wc_str);
    let slen = s.len();
    if mb_str != 0 && mb_len > 0 {
        emu.maps.write_string(mb_str, &s);
    }
    log_red!(
        emu,
        "kernelbase!WideCharToMultiByte cp={} '{}' => {}",
        code_page,
        s,
        slen + 1
    );
    emu.set_last_error(0);
    call.set_return(emu, (slen + 1) as u64);
}
