use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn recvfrom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0));
    let buff = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let len = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    log_red!(
        emu,
        "ws2_32!recvfrom sock: {} buff: 0x{:x} len: {}",
        sock,
        buff,
        len
    );
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, 0xffff_ffff); // SOCKET_ERROR
        return;
    }
    // No real peer: report an orderly connection close (0 bytes).
    call.set_return(emu, 0);
    call.cleanup_args(emu, 6);
}
