use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IntersectRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src1 = api_try!(emu, call.arg(emu, 1));
    let src2 = api_try!(emu, call.arg(emu, 2));
    if dst != 0 && src1 != 0 && src2 != 0 {
        let l1 = emu.maps.read_dword(src1).unwrap_or(0) as i32;
        let t1 = emu.maps.read_dword(src1 + 4).unwrap_or(0) as i32;
        let r1 = emu.maps.read_dword(src1 + 8).unwrap_or(0) as i32;
        let b1 = emu.maps.read_dword(src1 + 12).unwrap_or(0) as i32;
        let l2 = emu.maps.read_dword(src2).unwrap_or(0) as i32;
        let t2 = emu.maps.read_dword(src2 + 4).unwrap_or(0) as i32;
        let r2 = emu.maps.read_dword(src2 + 8).unwrap_or(0) as i32;
        let b2 = emu.maps.read_dword(src2 + 12).unwrap_or(0) as i32;
        let left = std::cmp::max(l1, l2);
        let top = std::cmp::max(t1, t2);
        let right = std::cmp::min(r1, r2);
        let bottom = std::cmp::min(b1, b2);
        if left < right && top < bottom {
            emu.maps.write_dword(dst, left as u32);
            emu.maps.write_dword(dst + 4, top as u32);
            emu.maps.write_dword(dst + 8, right as u32);
            emu.maps.write_dword(dst + 12, bottom as u32);
            call.set_return(emu, 1);
        } else {
            emu.maps.write_dword(dst, 0);
            emu.maps.write_dword(dst + 4, 0);
            emu.maps.write_dword(dst + 8, 0);
            emu.maps.write_dword(dst + 12, 0);
            call.set_return(emu, 0);
        }
    } else {
        call.set_return(emu, 0);
    }
    log_red!(
        emu,
        "user32!IntersectRect dst:0x{:x} s1:0x{:x} s2:0x{:x}",
        dst,
        src1,
        src2
    );
}
