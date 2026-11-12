use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn CoCreateInstance(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let _punk_outer = emu.regs().rdx;
    let dw_cls_context = emu.regs().r8;
    let riid = emu.regs().r9;
    let ppv = WinApiAbi::arg(emu, 4);

    let clsid_str = if rclsid != 0 {
        read_guid_string(emu, rclsid)
    } else {
        String::from("{NULL}")
    };
    let riid_str = if riid != 0 {
        read_guid_string(emu, riid)
    } else {
        String::from("{NULL}")
    };

    log_red!(
        emu,
        "ole32!CoCreateInstance clsid:{} ctx:0x{:x} riid:{} ppv:0x{:x}",
        clsid_str,
        dw_cls_context,
        riid_str,
        ppv
    );

    if ppv != 0 {
        emu.maps.write_qword(ppv, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80040154; // REGDB_E_CLASSNOTREG
}
