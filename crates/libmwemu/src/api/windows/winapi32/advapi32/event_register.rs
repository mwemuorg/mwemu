use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EventRegister(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_id = api_try!(emu, call.arg_u32(emu, 0));
    let enable_callback = api_try!(emu, call.arg_u32(emu, 1));
    let callback_ctx = api_try!(emu, call.arg_u32(emu, 2));
    let reg_handle = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "advapi32!EventRegister prov: 0x{:x} cb: 0x{:x} ctx: 0x{:x} out: 0x{:x}",
        provider_id,
        enable_callback,
        callback_ctx,
        reg_handle
    );
    if reg_handle != 0 {
        let h = helper::handler_create("event_provider");
        emu.maps.write_dword(reg_handle as u64, h as u32);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}
