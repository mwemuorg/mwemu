use crate::emu;
use crate::windows::constants::errors::*;

pub fn BCryptSignHash(emu: &mut emu::Emu) {
    let _h_key = emu.regs().rcx;
    let _pb_padding = emu.regs().rdx;
    let pb_input = emu.regs().r8;
    let cb_input = emu.regs().r9 as usize;
    let pb_output = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    log_red!(emu, "bcrypt!BCryptSignHash out=0x{:x}", pb_output);

    let hash_bytes = if pb_input != 0 && cb_input > 0 {
        emu.maps.read_bytes(pb_input, cb_input.min(64)).to_vec()
    } else {
        vec![0u8; 32]
    };
    let out_len = cb_output.min(256);
    if pb_output != 0 && out_len > 0 {
        let mut sig = vec![0u8; out_len];
        if out_len >= hash_bytes.len() + 11 {
            sig[0] = 0x00;
            sig[1] = 0x01;
            let ps_len = out_len - hash_bytes.len() - 3;
            for b in &mut sig[2..2 + ps_len] {
                *b = 0xff;
            }
            sig[2 + ps_len] = 0x00;
            sig[3 + ps_len..].copy_from_slice(&hash_bytes);
        } else {
            let copy_len = hash_bytes.len().min(out_len);
            sig[..copy_len].copy_from_slice(&hash_bytes[..copy_len]);
        }
        emu.maps.write_bytes(pb_output, &sig);
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, out_len as u32);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}
