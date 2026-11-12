use crate::emu;
use crate::winapi::common::strings;

pub fn ClearEventLogA(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogA", false);
}

fn clear_event_log(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn ClearEventLogW(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogW", true);
}
