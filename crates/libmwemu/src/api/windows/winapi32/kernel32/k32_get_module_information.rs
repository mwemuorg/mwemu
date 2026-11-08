use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn K32GetModuleInformation(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let h_module = arg(emu, 1) as u64;
    let lp_modinfo = arg(emu, 2) as u64;
    let cb = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!K32GetModuleInformation hProc:0x{:x} hMod:0x{:x} pInfo:0x{:x} cb:{}",
        h_process,
        h_module,
        lp_modinfo,
        cb
    );

    if lp_modinfo != 0 && cb >= 24 {
        emu.maps.write_qword(lp_modinfo, h_module);
        emu.maps.write_dword(lp_modinfo + 8, 0x1000);
        emu.maps.write_qword(lp_modinfo + 16, h_module);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
