use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn BCryptGenerateKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let ph_key = arg(emu, 1) as u64;
    let length = arg(emu, 2);
    let _flags = arg(emu, 3);

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkeypair://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateKeyPair algo={} bits={} =0x{:x}",
        algo_name,
        length,
        hndl
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
