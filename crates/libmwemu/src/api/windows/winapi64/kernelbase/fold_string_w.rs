use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FoldStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _flags = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let src_len = api_try!(emu, call.arg_i32(emu, 2));
    let dst_ptr = api_try!(emu, call.arg(emu, 3));
    let dst_len = api_try!(emu, call.arg_i32(emu, 4));
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let result_len = src.encode_utf16().count() + 1;
    if dst_ptr != 0 && dst_len > 0 {
        emu.maps.write_wide_string(dst_ptr, &src);
    }
    log_red!(emu, "kernelbase!FoldStringW '{}' => {}", src, result_len);
    emu.set_last_error(0);
    call.set_return(emu, result_len as u64);
}
