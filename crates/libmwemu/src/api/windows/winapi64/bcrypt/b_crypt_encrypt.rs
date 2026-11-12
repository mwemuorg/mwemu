use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BCryptEncrypt(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let pb_input = emu.regs().rdx;
    let cb_input = emu.regs().r8 as usize;
    let _pb_padding = emu.regs().r9;
    let _pb_iv = WinApiAbi::arg(emu, 4);
    let _cb_iv = WinApiAbi::arg(emu, 5) as usize;
    let pb_output = WinApiAbi::arg(emu, 6);
    let cb_output = WinApiAbi::arg(emu, 7) as usize;
    let pcb_result = WinApiAbi::arg(emu, 8);
    let _flags = WinApiAbi::arg(emu, 9);

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
}
