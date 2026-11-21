use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetAddrInfoExA(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExA", false);
}

fn get_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let service_ptr = api_try!(emu, call.arg(emu, 1));
    let namespace = api_try!(emu, call.arg_u32(emu, 2));
    let hints_ptr = api_try!(emu, call.arg(emu, 3));
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        strings::read_text(emu, service_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!{api} name:'{}' service:'{}' ns:{}",
        name,
        service,
        namespace
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn GetAddrInfoExW(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExW", true);
}
