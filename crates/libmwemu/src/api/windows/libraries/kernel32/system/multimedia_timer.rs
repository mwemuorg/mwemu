use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::helper::read_tick_count;
use crate::winapi::policy::api_try;
use crate::windows::constants::time::TIMERR_NOCANDO;

pub fn timeGetTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ticks = read_tick_count(emu) as u32;
    log_red!(emu, "timeGetTime milliseconds={ticks}");
    call.set_return(emu, u64::from(ticks));
}

pub fn timeGetDevCaps(emu: &mut Emu) {
    const MIN_PERIOD_MS: u32 = 1;
    const MAX_PERIOD_MS: u32 = 1_000_000;
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let length = api_try!(emu, call.arg_u32(emu, 1));
    let caps = [MIN_PERIOD_MS.to_le_bytes(), MAX_PERIOD_MS.to_le_bytes()];
    let status = if out != 0
        && length as usize >= size_of_val(&caps)
        && emu
            .maps
            .validate_write_range(out, size_of_val(&caps) as u64)
        && emu.maps.write_bytes(out, caps.as_flattened())
    {
        0
    } else {
        TIMERR_NOCANDO
    };
    log_red!(
        emu,
        "timeGetDevCaps capabilities_out={out:#x} size={length} status={status}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}

pub(crate) fn dispatch_multimedia_timer(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "timeGetDevCaps" => timeGetDevCaps(emu),
        "timeGetTime" => timeGetTime(emu),
    }, handled(()) => true, fallback => false)
}
