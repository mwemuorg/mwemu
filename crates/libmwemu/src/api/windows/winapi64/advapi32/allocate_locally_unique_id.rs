use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AllocateLocallyUniqueId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let luid_ptr = api_try!(emu, call.arg(emu, 0));
    if luid_ptr != 0 {
        static LUID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1000);
        let low = LUID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        emu.maps.write_dword(luid_ptr, low);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(emu, "advapi32!AllocateLocallyUniqueId ptr:0x{:x}", luid_ptr);
    call.set_return(emu, 1);
}
