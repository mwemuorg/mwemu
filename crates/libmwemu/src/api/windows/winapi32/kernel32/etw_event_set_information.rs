use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EtwEventSetInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reg_handle = api_try!(emu, call.arg(emu, 0));
    let info_class = api_try!(emu, call.arg_u32(emu, 1));
    let info_ptr = api_try!(emu, call.arg(emu, 2));
    let info_len = api_try!(emu, call.arg_u32(emu, 3));
    let info_data = emu.maps.read_string(info_ptr);
    log_red!(
        emu,
        "kernel32!EventSetInformation handle:0x{:x} class:{} data:'{}' len:{}",
        reg_handle,
        info_class,
        info_data,
        info_len
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, 0);
}
