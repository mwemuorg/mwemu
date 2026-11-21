use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn ProgIDFromCLSID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let clsid_ptr = api_try!(emu, call.arg(emu, 0));
    let ppsz = api_try!(emu, call.arg(emu, 1));
    let guid_str = read_guid_string(emu, clsid_ptr);
    let prog_id = format!("Class.{}", &guid_str[1..9]);
    if ppsz != 0 {
        let alloc_sz = (prog_id.len() as u64 + 1) * 2;
        if let Some(buf) = emu.maps.alloc(alloc_sz) {
            emu.maps.create_map(
                &format!("progid_{:x}", buf),
                buf,
                alloc_sz,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_wide_string(buf, &prog_id);
            emu.maps.write_dword(ppsz, buf as u32);
        }
    }
    log_red!(emu, "ole32!ProgIDFromCLSID {} => '{}'", guid_str, prog_id);
    call.set_return(emu, 0);
}
