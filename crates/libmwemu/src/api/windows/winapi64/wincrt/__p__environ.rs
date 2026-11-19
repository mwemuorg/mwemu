use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn __p__environ(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    // char *** __p__environ(void)
    // Return a pointer to a NULL-terminated environment pointer list (empty env).
    let envp = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__environ alloc failed");
            call.set_return(emu, 0);
            return;
        }
    };
    let _ = emu.maps.write_qword(envp, 0);
    call.set_return(emu, envp);
}
