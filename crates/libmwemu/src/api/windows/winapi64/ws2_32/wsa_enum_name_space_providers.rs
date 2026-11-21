use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAEnumNameSpaceProvidersA(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers(emu, "WSAEnumNameSpaceProvidersA");
}

fn wsa_enum_name_space_providers(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let buf_len_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10014);
    call.set_return(emu, 0xFFFFFFFF);
}

pub fn WSAEnumNameSpaceProvidersW(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers(emu, "WSAEnumNameSpaceProvidersW");
}
