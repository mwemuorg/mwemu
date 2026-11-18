use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptGenRandom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let len = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    log_red!(emu, "bcrypt!BCryptGenRandom buf: 0x{:x} len: {}", buf, len);
    for i in 0..len {
        emu.maps
            .write_byte(buf + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 4);
}
