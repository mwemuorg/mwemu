use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn StringFromIID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let iid_ptr = api_try!(emu, call.arg(emu, 0));
    let out_ptr = api_try!(emu, call.arg(emu, 1));
    let guid_str = read_guid_string(emu, iid_ptr);
    if out_ptr != 0 {
        let alloc_sz: u64 = 39 * 2 + 2;
        if let Some(buf) = emu.maps.alloc(alloc_sz) {
            emu.maps.create_map(
                &format!("iid_str_{:x}", buf),
                buf,
                alloc_sz,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_wide_string(buf, &guid_str);
            emu.maps.write_dword(out_ptr, buf as u32);
        } else {
            emu.maps.write_dword(out_ptr, 0);
        }
    }
    log_red!(emu, "ole32!StringFromIID => '{}'", guid_str);
    call.set_return(emu, 0);
}
