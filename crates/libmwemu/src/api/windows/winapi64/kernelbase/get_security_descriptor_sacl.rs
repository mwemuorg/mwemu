use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSecurityDescriptorSacl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sd_ptr = api_try!(emu, call.arg(emu, 0));
    let sacl_present_ptr = api_try!(emu, call.arg(emu, 1));
    let sacl_ptr_ptr = api_try!(emu, call.arg(emu, 2));
    let defaulted_ptr = api_try!(emu, call.arg(emu, 3));
    if sd_ptr != 0 {
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        let has_sacl = (control & 0x0010) != 0; // SE_SACL_PRESENT
        let sacl = emu.maps.read_qword(sd_ptr + 20).unwrap_or(0);
        if sacl_present_ptr != 0 {
            emu.maps
                .write_dword(sacl_present_ptr, if has_sacl { 1 } else { 0 });
        }
        if sacl_ptr_ptr != 0 {
            emu.maps
                .write_qword(sacl_ptr_ptr, if has_sacl { sacl } else { 0 });
        }
        if defaulted_ptr != 0 {
            let defaulted = (control & 0x0020) != 0; // SE_SACL_DEFAULTED
            emu.maps
                .write_dword(defaulted_ptr, if defaulted { 1 } else { 0 });
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorSacl sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
