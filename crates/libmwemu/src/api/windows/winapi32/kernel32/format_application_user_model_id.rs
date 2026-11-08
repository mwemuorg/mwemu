use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FormatApplicationUserModelId(emu: &mut emu::Emu) {
    let p_family_name = arg(emu, 0) as u64;
    let p_app_id = arg(emu, 1) as u64;
    let p_app_model_id_len = arg(emu, 2) as u64;
    let p_app_model_id = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!FormatApplicationUserModelId family:0x{:x} appId:0x{:x}",
        p_family_name,
        p_app_id
    );
    let src = arg(emu, 0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
