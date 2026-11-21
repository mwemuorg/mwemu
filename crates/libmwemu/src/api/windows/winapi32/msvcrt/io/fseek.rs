use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn fseek(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fp = api_try!(emu, call.arg(emu, 0));
    let offset = api_try!(emu, call.arg_i32(emu, 1)) as i64;
    let origin = api_try!(emu, call.arg_u32(emu, 2));
    let new_pos = helper::handler_seek(fp, offset, origin);
    log_red!(
        emu,
        "msvcrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
        fp,
        offset,
        origin,
        new_pos
    );
    call.set_return(emu, if new_pos >= 0 { 0 } else { u64::MAX });
}
