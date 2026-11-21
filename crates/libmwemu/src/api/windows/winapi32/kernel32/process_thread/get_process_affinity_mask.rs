use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessAffinityMask(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    let proc_affinity_mask_ptr = api_try!(emu, call.arg(emu, 1));
    let sys_affinity_mask_ptr = api_try!(emu, call.arg(emu, 2));

    emu.maps.write_dword(proc_affinity_mask_ptr, 0x1337);
    emu.maps.write_dword(sys_affinity_mask_ptr, 0x1337);

    log_red!(emu, "kernel32!GetProcessAffinityMask");

    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
