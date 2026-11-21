use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn CryptAcquireContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov_ptr = api_try!(emu, call.arg(emu, 0));
    let container_ptr = api_try!(emu, call.arg(emu, 1));
    let provider_ptr = api_try!(emu, call.arg(emu, 2));
    let prov_type = api_try!(emu, call.arg_u32(emu, 3));
    let flags = api_try!(emu, call.arg_u32(emu, 4));

    let container = if container_ptr != 0 {
        emu.maps.read_string(container_ptr)
    } else {
        String::new()
    };
    let provider = if provider_ptr != 0 {
        emu.maps.read_string(provider_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "crypt32!CryptAcquireContext container: '{}' provider: '{}' type: {} flags: 0x{:x}",
        container,
        provider,
        prov_type,
        flags
    );

    let h_prov = helper::handler_create(&format!("cryptprov:{}", prov_type));
    if h_prov_ptr != 0 {
        emu.maps.write_qword(h_prov_ptr, h_prov);
    }
    call.set_return(emu, 1); // TRUE
}
