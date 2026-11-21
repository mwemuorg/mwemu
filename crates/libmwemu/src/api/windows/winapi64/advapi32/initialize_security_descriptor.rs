use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeSecurityDescriptor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let revision = api_try!(emu, call.arg_u32(emu, 1));
    if sd_ptr != 0 {
        for i in 0..20u64 {
            emu.maps.write_byte(sd_ptr + i, 0);
        }
        emu.maps.write_byte(sd_ptr, revision as u8);
    }
    log_red!(
        emu,
        "advapi32!InitializeSecurityDescriptor sd:0x{:x} rev:{}",
        sd_ptr,
        revision
    );
    call.set_return(emu, 1);
}
