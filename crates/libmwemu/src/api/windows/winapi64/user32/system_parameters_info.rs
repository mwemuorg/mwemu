use crate::emu;

pub(super) fn SystemParametersInfoA(emu: &mut emu::Emu) {
    let ui_action = emu.regs().rcx as u32;
    let ui_param = emu.regs().rdx as u32;
    let pv_param = emu.regs().r8;
    let _f_win_ini = emu.regs().r9;
    log_red!(
        emu,
        "user32!SystemParametersInfoA action:0x{:x} param:{} pvParam:0x{:x}",
        ui_action,
        ui_param,
        pv_param
    );
    if pv_param != 0 {
        match ui_action {
            0x0030 => {
                // SPI_GETWORKAREA - fill RECT
                emu.maps.write_dword(pv_param, 0);
                emu.maps.write_dword(pv_param + 4, 0);
                emu.maps.write_dword(pv_param + 8, 1920);
                emu.maps.write_dword(pv_param + 12, 1040);
            }
            0x0048 => {
                // SPI_GETNONCLIENTMETRICS
                emu.maps.write_dword(pv_param, ui_param);
            }
            _ => {}
        }
    }
    emu.regs_mut().rax = 1;
}

pub fn SystemParametersInfoW(emu: &mut emu::Emu) {
    let ui_action = emu.regs().rcx as u32;
    let ui_param = emu.regs().rdx as u32;
    let pv_param = emu.regs().r8;
    let _f_win_ini = emu.regs().r9;
    log_red!(
        emu,
        "user32!SystemParametersInfoW action:0x{:x} param:{} pvParam:0x{:x}",
        ui_action,
        ui_param,
        pv_param
    );
    if pv_param != 0 {
        match ui_action {
            0x0030 => {
                emu.maps.write_dword(pv_param, 0);
                emu.maps.write_dword(pv_param + 4, 0);
                emu.maps.write_dword(pv_param + 8, 1920);
                emu.maps.write_dword(pv_param + 12, 1040);
            }
            0x0048 => {
                emu.maps.write_dword(pv_param, ui_param);
            }
            _ => {}
        }
    }
    emu.regs_mut().rax = 1;
}
