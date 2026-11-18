use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn select(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let nfds = api_try!(emu, call.arg_u32(emu, 0));
    let readfds = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let writefds = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let exceptfds = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let timeout_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let timeout_str = if timeout_ptr == 0 {
        "infinite".to_string()
    } else {
        let tv_sec = emu.maps.read_dword(timeout_ptr).unwrap_or(0);
        let tv_usec = emu.maps.read_dword(timeout_ptr + 4).unwrap_or(0);
        format!("{}.{:06}s", tv_sec, tv_usec)
    };
    log_red!(
        emu,
        "ws2_32!select nfds:{} read:0x{:x} write:0x{:x} except:0x{:x} timeout:{}",
        nfds,
        readfds,
        writefds,
        exceptfds,
        timeout_str
    );
    // Return 0 = timeout, no sockets ready
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}
