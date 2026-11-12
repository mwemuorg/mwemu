use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyA");
}

fn reg_query_info_key(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let lp_class = emu.regs().rdx;
    let lpcch_class = emu.regs().r8;
    let _reserved = emu.regs().r9;
    let lpc_sub_keys = WinApiAbi::arg(emu, 4);
    let lpcb_max_sub_key_len = WinApiAbi::arg(emu, 5);
    let lpcb_max_class_len = WinApiAbi::arg(emu, 6);
    let lpc_values = WinApiAbi::arg(emu, 7);
    let lpcb_max_value_name_len = WinApiAbi::arg(emu, 8);
    let lpcb_max_value_len = WinApiAbi::arg(emu, 9);
    let lpcb_security_desc = WinApiAbi::arg(emu, 10);
    let lpft_last_write = WinApiAbi::arg(emu, 11);

    log_red!(emu, "kernelbase!{api} hkey:0x{:x}", hkey);

    if lpcch_class != 0 {
        emu.maps.write_dword(lpcch_class, 0);
    }
    if lpc_sub_keys != 0 {
        emu.maps.write_dword(lpc_sub_keys, 0);
    }
    if lpcb_max_sub_key_len != 0 {
        emu.maps.write_dword(lpcb_max_sub_key_len, 0);
    }
    if lpcb_max_class_len != 0 {
        emu.maps.write_dword(lpcb_max_class_len, 0);
    }
    if lpc_values != 0 {
        emu.maps.write_dword(lpc_values, 0);
    }
    if lpcb_max_value_name_len != 0 {
        emu.maps.write_dword(lpcb_max_value_name_len, 0);
    }
    if lpcb_max_value_len != 0 {
        emu.maps.write_dword(lpcb_max_value_len, 0);
    }
    if lpcb_security_desc != 0 {
        emu.maps.write_dword(lpcb_security_desc, 0);
    }
    if lpft_last_write != 0 {
        emu.maps.write_qword(lpft_last_write, 0);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyW");
}
