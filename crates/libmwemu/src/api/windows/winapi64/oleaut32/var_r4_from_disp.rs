use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VarR4FromDisp(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let src_val = api_try!(emu, call.arg(emu, 0));
    let dst_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "oleaut32!VarR4FromDisp src:0x{:x} dst:0x{:x}",
        src_val,
        dst_ptr
    );
    if dst_ptr != 0 {
        emu.maps.write_dword(dst_ptr, src_val as u32);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
