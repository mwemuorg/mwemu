use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetThreadStackGuarantee(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let stack_size_ptr = api_try!(emu, call.arg(emu, 0));

    let requested = if stack_size_ptr != 0 {
        emu.maps.read_dword(stack_size_ptr).unwrap_or(0)
    } else {
        0
    };

    log_red!(
        emu,
        "kernel32!SetThreadStackGuarantee ptr:0x{:x} requested:{}",
        stack_size_ptr,
        requested
    );

    // Write back the current guarantee (simulate a reasonable default)
    if stack_size_ptr != 0 {
        emu.maps.write_dword(stack_size_ptr, 0x1000); // 4KB default
    }

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
