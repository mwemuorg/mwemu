use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ProcessPrng(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pb_data = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let cb_data = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(
        emu,
        "bcrypt!ProcessPrng buf:0x{:x} len:{}",
        pb_data,
        cb_data
    );
    for i in 0..cb_data {
        emu.maps
            .write_byte(pb_data + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
