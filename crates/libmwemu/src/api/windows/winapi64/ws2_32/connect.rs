use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn connect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let sockaddr_ptr = api_try!(emu, call.arg(emu, 1));
    //let sockaddr = emu.maps.read_bytes(sockaddr_ptr, 8);
    let family: u16 = emu.maps.read_word(sockaddr_ptr).unwrap_or(0);
    let port: u16 = emu.maps.read_word(sockaddr_ptr + 2).unwrap_or(0).to_be();
    let ip: u32 = emu.maps.read_dword(sockaddr_ptr + 4).unwrap_or(0);

    let sip = format!(
        "{}.{}.{}.{}",
        ip & 0xff,
        (ip & 0xff00) >> 8,
        (ip & 0xff0000) >> 16,
        (ip & 0xff000000) >> 24
    );
    log_red!(emu, "ws2_32!connect  family: {} {}:{}", family, sip, port);

    if emu.cfg.endpoint {
        /*
        if endpoint::sock_connect(sip.as_str(), port) {
            log::trace!("\tconnected to the endpoint.");
        } else {
            log::trace!("\tcannot connect. dont use -e");
        }*/
        call.set_return(emu, 0);
    } else {
        // offline mode

        if !helper::socket_exist(sock) {
            log::trace!("\tinvalid socket.");
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    }
}
