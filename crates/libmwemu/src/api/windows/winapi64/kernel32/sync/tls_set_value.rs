use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg(emu, 0)) as usize; // First parameter in RCX
    let val = api_try!(emu, call.arg(emu, 1)); // Second parameter in RDX

    log_red!(emu, "kernel32!TlsSetValue idx: {} val: 0x{:x}", idx, val);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    if idx < emu.tls64().len() {
        emu.tls64_mut()[idx] = val;
    } else {
        // Expand TLS array if needed
        while emu.tls64().len() <= idx {
            emu.tls64_mut().push(0);
        }
        emu.tls64_mut()[idx] = val;
    }

    emu.set_last_error(0);
    call.set_return(emu, 1); // Return TRUE
}
