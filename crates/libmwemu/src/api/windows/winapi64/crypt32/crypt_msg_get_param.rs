use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptMsgGetParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmsg = api_try!(emu, call.arg(emu, 0));
    let param_type = api_try!(emu, call.arg(emu, 1));
    let index = api_try!(emu, call.arg(emu, 2));
    let pv_data = api_try!(emu, call.arg(emu, 3));
    let pcb_data = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptMsgGetParam msg:0x{:x} type:{} idx:{}",
        hmsg,
        param_type,
        index
    );
    if pcb_data != 0 {
        emu.maps.write_dword(pcb_data, 0);
    }
    call.set_return(emu, 1);
}
