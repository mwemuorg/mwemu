use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSecurityDescriptorGroup(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let group_ptr_ptr = api_try!(emu, call.arg(emu, 1));
    let defaulted_ptr = api_try!(emu, call.arg(emu, 2));
    if sd_ptr != 0 {
        let group = emu.maps.read_qword(sd_ptr + 20).unwrap_or(0);
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if group_ptr_ptr != 0 {
            emu.maps.write_qword(group_ptr_ptr, group);
        }
        if defaulted_ptr != 0 {
            let defaulted = (control & 0x0002) != 0; // SE_GROUP_DEFAULTED
            emu.maps
                .write_dword(defaulted_ptr, if defaulted { 1 } else { 0 });
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorGroup sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
