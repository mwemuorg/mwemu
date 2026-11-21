use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ConvertStringSidToSidA(emu: &mut emu::Emu) {
    convert_string_sid_to_sid(emu, "ConvertStringSidToSidA", false);
}

fn convert_string_sid_to_sid(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_sid_ptr = api_try!(emu, call.arg(emu, 0));
    let sid_out_ptr = api_try!(emu, call.arg(emu, 1));
    let s = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
        strings::read_text(emu, str_sid_ptr, wide)
    } else {
        format!("0x{:x}", str_sid_ptr)
    };
    log_red!(emu, "advapi32!{api} '{}'", s);
    if sid_out_ptr != 0 {
        let sid_bytes: [u8; 28] = [
            1, 5, 0, 0, 0, 0, 0, 5, 0x15, 0, 0, 0, 0xd2, 0x04, 0, 0, 0xd2, 0x04, 0, 0, 0xd2, 0x04,
            0, 0, 0xe9, 0x03, 0, 0,
        ];
        let alloc_size = sid_bytes.len() as u64;
        if let Some(base) = emu.maps.alloc(alloc_size) {
            let name = format!("sid_{:x}", base);
            emu.maps.create_map(
                &name,
                base,
                alloc_size,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_bytes(base, &sid_bytes);
            emu.maps.write_qword(sid_out_ptr, base);
        }
    }
    call.set_return(emu, 1);
}

pub fn ConvertStringSidToSidW(emu: &mut emu::Emu) {
    convert_string_sid_to_sid(emu, "ConvertStringSidToSidW", true);
}
