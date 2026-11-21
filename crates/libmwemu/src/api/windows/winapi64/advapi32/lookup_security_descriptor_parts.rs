use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn LookupSecurityDescriptorPartsA(emu: &mut emu::Emu) {
    lookup_security_descriptor_parts(emu, "LookupSecurityDescriptorPartsA", false);
}

fn lookup_security_descriptor_parts(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let system_ptr = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn LookupSecurityDescriptorPartsW(emu: &mut emu::Emu) {
    lookup_security_descriptor_parts(emu, "LookupSecurityDescriptorPartsW", true);
}
