use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageA", false);
}

fn format_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let msg_id = api_try!(emu, call.arg(emu, 2));
    let lang_id = api_try!(emu, call.arg(emu, 3));
    let buf_ptr = api_try!(emu, call.arg(emu, 4));
    let buf_size = api_try!(emu, call.arg(emu, 5));
    let msg = format!("Error {}", msg_id);
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, &msg, wide);
    }
    log_red!(
        emu,
        "kernelbase!{api} flags:0x{:x} msg_id:{} => '{}'",
        flags,
        msg_id,
        msg
    );
    let src = api_try!(emu, call.arg(emu, 0));
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = strings::read_text(emu, src, wide);
    }
    emu.set_last_error(0);
    call.set_return(emu, msg.len() as u64);
}

pub fn FormatMessageW(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageW", true);
}
