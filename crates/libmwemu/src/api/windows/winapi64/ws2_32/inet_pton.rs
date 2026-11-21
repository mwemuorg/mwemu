use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn inet_pton(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let dst_ptr = api_try!(emu, call.arg(emu, 2));
    let src_str = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!inet_pton family: {} src: '{}'",
        family,
        src_str
    );
    if family == 2 && dst_ptr != 0 {
        // AF_INET
        let parts: Vec<&str> = src_str.split('.').collect();
        if parts.len() == 4
            && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            )
        {
            let addr = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
            emu.maps.write_dword(dst_ptr, addr);
            call.set_return(emu, 1); // success
        } else {
            call.set_return(emu, 0); // invalid
        }
    } else {
        call.set_return(emu, 0);
    }
}

pub fn InetPtonW(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let dst_ptr = api_try!(emu, call.arg(emu, 2));
    let src_str = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!InetPtonW family:{} src:'{}'", family, src_str);
    if family == 2 && dst_ptr != 0 {
        let parts: Vec<&str> = src_str.split('.').collect();
        if parts.len() == 4
            && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            )
        {
            let addr = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
            emu.maps.write_dword(dst_ptr, addr);
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        call.set_return(emu, 0);
    }
}
