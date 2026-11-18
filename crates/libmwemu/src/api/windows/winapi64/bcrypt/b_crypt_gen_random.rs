use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptGenRandom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pb_buffer = api_try!(emu, call.arg(emu, 1));
    let cb_buffer = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "bcrypt!BCryptGenRandom buf: 0x{:x} len: {} flags: 0x{:x}",
        pb_buffer,
        cb_buffer,
        flags
    );

    for i in 0..cb_buffer {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_buffer + i, byte);
    }
    call.set_return(emu, STATUS_SUCCESS);
}
