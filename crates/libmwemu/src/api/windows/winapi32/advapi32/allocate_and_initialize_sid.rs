use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sia_ptr = api_try!(emu, call.arg(emu, 0));
    let sub_auth_count = api_try!(emu, call.arg_u32(emu, 1));
    let sa0 = api_try!(emu, call.arg_u32(emu, 2));
    let sa1 = api_try!(emu, call.arg_u32(emu, 3));
    let sa2 = api_try!(emu, call.arg_u32(emu, 4));
    let sa3 = api_try!(emu, call.arg_u32(emu, 5));
    let sa4 = api_try!(emu, call.arg_u32(emu, 6));
    let sa5 = api_try!(emu, call.arg_u32(emu, 7));
    let sa6 = api_try!(emu, call.arg_u32(emu, 8));
    let sa7 = api_try!(emu, call.arg_u32(emu, 9));
    let pp_sid = api_try!(emu, call.arg(emu, 10));

    // SID: revision(1) + sub_auth_count(1) + authority(6) + sub_auths(4*count)
    let count = std::cmp::min(sub_auth_count, 8) as usize;
    let sid_size = 8 + count * 4;
    if let Some(sid_addr) = emu.maps.alloc(sid_size as u64) {
        // revision=1, sub_auth_count
        emu.maps.write_byte(sid_addr, 1);
        emu.maps.write_byte(sid_addr + 1, count as u8);
        // Copy SIA (6 bytes)
        if sia_ptr != 0 {
            for i in 0..6 {
                let b = emu.maps.read_byte(sia_ptr + i).unwrap_or(0);
                emu.maps.write_byte(sid_addr + 2 + i, b);
            }
        }
        // Write sub-authorities
        let sas = [sa0, sa1, sa2, sa3, sa4, sa5, sa6, sa7];
        for (i, &sa) in sas.iter().enumerate().take(count) {
            emu.maps.write_dword(sid_addr + 8 + (i as u64) * 4, sa);
        }
        if pp_sid != 0 {
            emu.maps.write_dword(pp_sid, sid_addr as u32);
        }
        log_red!(
            emu,
            "advapi32!AllocateAndInitializeSid count:{} => 0x{:x}",
            count,
            sid_addr
        );
    } else {
        log_red!(emu, "advapi32!AllocateAndInitializeSid alloc failed");
    }
    call.cleanup_args(emu, 11);
    call.set_return(emu, 1);
}
