use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn AuditLookupSubCategoryNameA(emu: &mut emu::Emu) {
    audit_lookup_sub_category_name(emu, "AuditLookupSubCategoryNameA", false);
}

fn audit_lookup_sub_category_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn AuditLookupSubCategoryNameW(emu: &mut emu::Emu) {
    audit_lookup_sub_category_name(emu, "AuditLookupSubCategoryNameW", true);
}
