use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn NormalizeString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let norm_form = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let src_len = api_try!(emu, call.arg_i32(emu, 2));
    let dst_ptr = api_try!(emu, call.arg(emu, 3));
    let dst_len = api_try!(emu, call.arg_i32(emu, 4));
    let src = emu.maps.read_wide_string(src_ptr);
    let actual_len = (src.len() + 1) as i32;
    if dst_len == 0 {
        log_red!(
            emu,
            "kernelbase!NormalizeString form:{} '{}' => need {} chars",
            norm_form,
            src,
            actual_len
        );
        call.set_return(emu, actual_len as u64);
    } else {
        if dst_ptr != 0 {
            emu.maps.write_wide_string(dst_ptr, &src);
        }
        log_red!(
            emu,
            "kernelbase!NormalizeString form:{} '{}' => {}",
            norm_form,
            src,
            actual_len
        );
        emu.set_last_error(0);
        call.set_return(emu, actual_len as u64);
    }
}
