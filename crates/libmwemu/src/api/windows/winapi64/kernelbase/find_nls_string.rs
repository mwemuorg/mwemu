use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindNLSString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale_id = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    let src_count = api_try!(emu, call.arg_i32(emu, 3));
    let src = emu.maps.read_wide_string(src_ptr);
    log_red!(
        emu,
        "kernelbase!FindNLSString locale:{} flags:0x{:x} src:'{}'",
        locale_id,
        flags,
        src
    );
    emu.set_last_error(0);
    call.set_return(emu, -1i64 as u64); // not found
}
