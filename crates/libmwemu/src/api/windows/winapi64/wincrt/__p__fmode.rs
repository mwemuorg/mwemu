use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn __p__fmode(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    // int * __p__fmode(void)
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__fmode alloc failed");
            call.set_return(emu, 0);
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    call.set_return(emu, p);
}
