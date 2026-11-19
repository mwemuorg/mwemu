use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let codepage = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let utf8_ptr = api_try!(emu, call.arg(emu, 2));
    let cbMultiByte = api_try!(emu, call.arg_u32(emu, 3));
    let wide_ptr = api_try!(emu, call.arg(emu, 4));
    let cchWideChar = api_try!(emu, call.arg_u32(emu, 5));

    call.cleanup_args(emu, 6);

    let utf8 = emu.maps.read_string(utf8_ptr);
    let mut wide = String::new();
    for c in utf8.chars() {
        wide.push_str(&format!("{}", c));
        wide.push('\x00');
    }

    log_red!(
        emu,
        "kernel32!MultiByteToWideChar '{}' dst:0x{:x}",
        utf8,
        wide_ptr
    );

    if cchWideChar > 0 {
        emu.maps.write_string(wide_ptr, &wide);
    }
    call.set_return(emu, wide.len() as u64);
}
