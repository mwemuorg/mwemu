use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeSecurityDescriptor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let revision = api_try!(emu, call.arg_u32(emu, 1));
    if sd_ptr != 0 {
        emu.maps.write_byte(sd_ptr, revision as u8);
        emu.maps.write_byte(sd_ptr + 1, 0); // Sbz1
        emu.maps.write_word(sd_ptr + 2, 0); // Control
        emu.maps.write_dword(sd_ptr + 4, 0); // Owner
        emu.maps.write_dword(sd_ptr + 8, 0); // Group
        emu.maps.write_dword(sd_ptr + 12, 0); // Sacl
        emu.maps.write_dword(sd_ptr + 16, 0); // Dacl
    }
    log_red!(
        emu,
        "advapi32!InitializeSecurityDescriptor sd:0x{:x} rev:{}",
        sd_ptr,
        revision
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
