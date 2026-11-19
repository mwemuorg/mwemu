use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_id_authority = api_try!(emu, call.arg(emu, 0));
    let n_sub_authority_count = api_try!(emu, call.arg_u8(emu, 1));
    let sub0 = api_try!(emu, call.arg_u32(emu, 2));
    let sub1 = api_try!(emu, call.arg_u32(emu, 3));
    let sub2 = api_try!(emu, call.arg_u32(emu, 4));
    let sub3 = api_try!(emu, call.arg_u32(emu, 5));
    let sub4 = api_try!(emu, call.arg_u32(emu, 6));
    let sub5 = api_try!(emu, call.arg_u32(emu, 7));
    let sub6 = api_try!(emu, call.arg_u32(emu, 8));
    let sub7 = api_try!(emu, call.arg_u32(emu, 9));
    let pp_sid = api_try!(emu, call.arg(emu, 10));

    let count = n_sub_authority_count.min(8) as usize;
    let sid_size = (8 + count * 4) as u64;

    log_red!(
        emu,
        "kernelbase!AllocateAndInitializeSid count:{} pp_sid:0x{:x}",
        count,
        pp_sid
    );

    if let Some(base) = emu.maps.alloc(sid_size) {
        let _ = emu.maps.create_map(
            &format!("sid_{:x}", base),
            base,
            sid_size,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        emu.maps.write_byte(base, 1); // revision
        emu.maps.write_byte(base + 1, count as u8);
        // Copy 6-byte identifier authority
        for i in 0..6u64 {
            let b = emu.maps.read_byte(p_id_authority + i).unwrap_or(0);
            emu.maps.write_byte(base + 2 + i, b);
        }
        let subs = [sub0, sub1, sub2, sub3, sub4, sub5, sub6, sub7];
        for (i, &s) in subs.iter().enumerate().take(count) {
            emu.maps.write_dword(base + 8 + (i as u64) * 4, s);
        }
        if pp_sid != 0 {
            emu.maps.write_qword(pp_sid, base);
        }
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(8); // ERROR_NOT_ENOUGH_MEMORY
        call.set_return(emu, 0);
    }
}
