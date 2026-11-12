use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ReportEventA(emu: &mut emu::Emu) {
    report_event(emu, "ReportEventA");
}

fn report_event(emu: &mut emu::Emu, api: &str) {
    let hevt_log = WinApiAbi::arg(emu, 0);
    let wtype = WinApiAbi::arg(emu, 1) as u32;
    let category = WinApiAbi::arg(emu, 2) as u32;
    let event_id = WinApiAbi::arg(emu, 3) as u32;
    let _user_sid = WinApiAbi::arg(emu, 4) as u32;
    let num_strings = WinApiAbi::arg(emu, 5) as u32;
    let _data_size = WinApiAbi::arg(emu, 6) as u32;
    let _strings_ptr = WinApiAbi::arg(emu, 7) as u32;
    let _raw_data = WinApiAbi::arg(emu, 8) as u32;
    log_red!(
        emu,
        "advapi32!{api} log:0x{:x} type:{} cat:{} id:0x{:x} nstr:{}",
        hevt_log,
        wtype,
        category,
        event_id,
        num_strings
    );
    for _ in 0..9 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ReportEventW(emu: &mut emu::Emu) {
    report_event(emu, "ReportEventW");
}
