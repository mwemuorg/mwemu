use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn QueryPerformanceCounter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let counter_ptr = api_try!(emu, call.arg(emu, 0));

    // Use emu.tick directly, maybe scaled up to simulate higher frequency
    let counter_value = (emu.tick as u64) * 1000; // Scale to simulate ~1MHz frequency

    emu.maps.write_qword(counter_ptr, counter_value);

    log_red!(
        emu,
        "kernel32!QueryPerformanceCounter counter: {}",
        counter_value
    );

    call.set_return(emu, 1); // Return TRUE
}
