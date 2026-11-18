use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn WSASend(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let buffers = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_sent_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;

    log_red!(emu, "ws2_32!WSASend sock:0x{:x} bufs:{}", sock, count);

    let total: u32 = if buffers != 0 && count > 0 {
        let mut sum = 0u32;
        for i in 0..count as u64 {
            let len = emu.maps.read_dword(buffers + i * 8).unwrap_or(0);
            sum += len;
        }
        sum
    } else {
        0
    };

    if bytes_sent_ptr != 0 {
        emu.maps.write_dword(bytes_sent_ptr, total);
    }

    call.set_return(emu, 0);
    call.cleanup_args(emu, 7);
}
