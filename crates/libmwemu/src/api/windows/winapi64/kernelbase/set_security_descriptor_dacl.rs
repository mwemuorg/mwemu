use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetSecurityDescriptorDacl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let dacl_present = api_try!(emu, call.arg(emu, 1)) != 0;
    let dacl_ptr = api_try!(emu, call.arg(emu, 2));
    let defaulted = api_try!(emu, call.arg(emu, 3)) != 0;
    if sd_ptr != 0 {
        let mut control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if dacl_present {
            control |= 0x0004;
        } else {
            control &= !0x0004;
        }
        if defaulted {
            control |= 0x0008;
        } else {
            control &= !0x0008;
        }
        emu.maps.write_word(sd_ptr + 2, control);
        emu.maps.write_qword(sd_ptr + 28, dacl_ptr);
    }
    log_red!(
        emu,
        "kernelbase!SetSecurityDescriptorDacl sd:0x{:x} present:{} dacl:0x{:x}",
        sd_ptr,
        dacl_present,
        dacl_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
