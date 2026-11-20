use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::policy::{self, api_try};

pub(super) fn frame_info(emu: &mut Emu, api: &str) {
    let call = WinApi(Cdecl);
    let begin = api_try!(emu, call.arg(emu, 0));
    let register = api == "__register_frame_info";
    if register {
        let object = api_try!(emu, call.arg(emu, 1));
        log_red!(
            emu,
            "libgcc_s_dw2-1!{api} begin={begin:#x} object={object:#x}"
        );
    } else {
        log_red!(emu, "libgcc_s_dw2-1!{api} begin={begin:#x}");
    }
    if begin != 0 {
        match emu.maps.read_dword(begin) {
            Some(0) => {}
            Some(_) => {
                policy::unsupported_with_abi(emu, "libgcc_s_dw2-1", api, emu.pc(), call);
                return;
            }
            None => {
                emu.exception(ExceptionType::BadAddressDereferencing);
                return;
            }
        }
    }
    if !register {
        call.set_return(emu, 0);
    }
}
