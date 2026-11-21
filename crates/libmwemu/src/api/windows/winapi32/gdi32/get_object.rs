use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetObjectA(emu: &mut emu::Emu) {
    get_object(emu, "GetObjectA");
}

fn get_object(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg_u32(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    log_red!(
        emu,
        "gdi32!{api} obj:0x{:x} size:{} buf:0x{:x}",
        obj,
        size,
        buffer
    );
    if buffer != 0 && size > 0 {
        let fill = size.min(64) as usize;
        for i in 0..fill {
            emu.maps.write_byte(buffer + i as u64, 0);
        }
        call.set_return(emu, fill as u64);
    } else {
        helper::advance_tick(emu, 1);
        call.set_return(emu, 24);
    }
    call.cleanup_args(emu, 3);
}

pub fn GetObjectW(emu: &mut emu::Emu) {
    get_object(emu, "GetObjectW");
}
