use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptEncrypt(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let pb_input = arg(emu, 1) as u64;
    let cb_input = arg(emu, 2) as usize;
    let _pb_padding = arg(emu, 3) as u64;
    let _pb_iv = arg(emu, 4) as u64;
    let _cb_iv = arg(emu, 5) as usize;
    let pb_output = arg(emu, 6) as u64;
    let cb_output = arg(emu, 7) as usize;
    let pcb_result = arg(emu, 8) as u64;
    let _flags = arg(emu, 9);

    log_red!(
        emu,
        "bcrypt!BCryptEncrypt key=0x{:x} in_len={} out_buf=0x{:x}",
        h_key,
        cb_input,
        pb_output
    );

    if pb_input != 0 && cb_input > 0 && pb_output != 0 && cb_output >= cb_input {
        let mut data = emu
            .maps
            .read_bytes(pb_input, cb_input.min(0x100_0000))
            .to_vec();
        crypto_state::encrypt_data(h_key, &mut data);
        emu.maps.write_bytes(pb_output, &data);
        if pcb_result != 0 {
            emu.maps.write_dword(pcb_result, data.len() as u32);
        }
    } else if pb_output == 0 && pcb_result != 0 {
        emu.maps.write_dword(pcb_result, cb_input as u32);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 10);
}
