use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn fread(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let fp = api_try!(emu, call.arg(emu, 3));
    let name = helper::handler_get_uri(fp);
    log_red!(
        emu,
        "msvcrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} '{}'",
        buf,
        size,
        count,
        fp,
        name
    );
    helper::handler_set_eof(fp);
    call.set_return(emu, 0);
}
