use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ProcessPrng(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pb_data = api_try!(emu, call.arg(emu, 0));
    let cb_data = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "bcrypt!ProcessPrng buf: 0x{:x} len: {}",
        pb_data,
        cb_data
    );
    for i in 0..cb_data {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_data + i, byte);
    }
    call.set_return(emu, 1); // BOOL TRUE
}
