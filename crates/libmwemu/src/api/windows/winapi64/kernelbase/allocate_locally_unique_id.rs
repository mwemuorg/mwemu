use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AllocateLocallyUniqueId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let luid_ptr = api_try!(emu, call.arg(emu, 0));
    static LUID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0x10000);
    let val = LUID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if luid_ptr != 0 {
        emu.maps.write_dword(luid_ptr, val as u32);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(
        emu,
        "kernelbase!AllocateLocallyUniqueId ptr:0x{:x} => 0x{:x}",
        luid_ptr,
        val
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
