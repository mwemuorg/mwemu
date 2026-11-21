use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSecurityDescriptorControl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let control_ptr = api_try!(emu, call.arg(emu, 1));
    let revision_ptr = api_try!(emu, call.arg(emu, 2));
    if sd_ptr != 0 {
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if control_ptr != 0 {
            emu.maps.write_word(control_ptr, control);
        }
        if revision_ptr != 0 {
            let rev = emu.maps.read_byte(sd_ptr).unwrap_or(1);
            emu.maps.write_dword(revision_ptr, rev as u32);
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorControl sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
