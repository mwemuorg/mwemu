use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _id_authority = api_try!(emu, call.arg(emu, 0));
    let sub_count = (api_try!(emu, call.arg_u8(emu, 1))).min(8);
    let sub_auths: [u32; 8] = [
        api_try!(emu, call.arg_u32(emu, 2)),
        api_try!(emu, call.arg_u32(emu, 3)),
        api_try!(emu, call.arg_u32(emu, 4)),
        api_try!(emu, call.arg_u32(emu, 5)),
        api_try!(emu, call.arg_u32(emu, 6)),
        api_try!(emu, call.arg_u32(emu, 7)),
        api_try!(emu, call.arg_u32(emu, 8)),
        api_try!(emu, call.arg_u32(emu, 9)),
    ];
    let psid_ptr = api_try!(emu, call.arg(emu, 10));
    let sid_len = 8 + sub_count as u64 * 4;
    if let Some(base) = emu.maps.alloc(sid_len) {
        let name = format!("sid_{:x}", base);
        emu.maps.create_map(
            &name,
            base,
            sid_len,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        emu.maps.write_byte(base, 1);
        emu.maps.write_byte(base + 1, sub_count);
        for (i, &sa) in sub_auths.iter().enumerate().take(sub_count as usize) {
            emu.maps.write_dword(base + 8 + i as u64 * 4, sa);
        }
        if psid_ptr != 0 {
            emu.maps.write_qword(psid_ptr, base);
        }
        log_red!(
            emu,
            "advapi32!AllocateAndInitializeSid count:{} => 0x{:x}",
            sub_count,
            base
        );
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
