use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameA", false);
}

fn lookup_privilege_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _system_ptr = api_try!(emu, call.arg(emu, 0));
    let luid_ptr = api_try!(emu, call.arg(emu, 1));
    let name_buf = api_try!(emu, call.arg(emu, 2));
    let size_ptr = api_try!(emu, call.arg(emu, 3));
    let priv_name = "SeDebugPrivilege";
    if name_buf != 0 {
        strings::write_string(emu, name_buf, priv_name, wide);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, priv_name.len() as u32);
    }
    log_red!(
        emu,
        "advapi32!{api} luid:0x{:x} => '{}'",
        luid_ptr,
        priv_name
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

pub fn LookupPrivilegeNameW(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameW", true);
}
