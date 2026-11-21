use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LCMapStringEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _locale_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    let src_len = api_try!(emu, call.arg_i32(emu, 3));
    let dst_ptr = api_try!(emu, call.arg(emu, 4));
    let dst_len = api_try!(emu, call.arg_i32(emu, 5));
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let mapped = if flags & 0x00000100 != 0 {
        src.to_uppercase()
    } else if flags & 0x00000200 != 0 {
        src.to_lowercase()
    } else {
        src.clone()
    };
    let result_len = mapped.encode_utf16().count() + 1;
    if dst_ptr != 0 && dst_len > 0 {
        emu.maps.write_wide_string(dst_ptr, &mapped);
    }
    log_red!(
        emu,
        "kernelbase!LCMapStringEx flags:0x{:x} '{}' => '{}'",
        flags,
        src,
        mapped
    );
    emu.set_last_error(0);
    call.set_return(emu, result_len as u64);
}
