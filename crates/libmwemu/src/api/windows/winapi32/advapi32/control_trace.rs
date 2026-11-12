use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ControlTraceA(emu: &mut emu::Emu) {
    control_trace(emu, "ControlTraceA", false);
}

fn control_trace(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = WinApiAbi::arg(emu, 2) as u32 as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    stdcall_ret(emu, 5);
    emu.regs_mut().rax = 1;
}

pub fn ControlTraceW(emu: &mut emu::Emu) {
    control_trace(emu, "ControlTraceW", true);
}
