use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let out_handle = arg(emu, 0) as u64;
    let alg_id_ptr = arg(emu, 1) as u64;
    let algo = emu.maps.read_wide_string(alg_id_ptr);
    log_red!(emu, "bcrypt!BCryptOpenAlgorithmProvider algo: {}", algo);
    if out_handle != 0 {
        let hndl = helper::handler_create(&format!("bcrypt://{}", algo));
        emu.maps.write_dword(out_handle, hndl as u32);
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
