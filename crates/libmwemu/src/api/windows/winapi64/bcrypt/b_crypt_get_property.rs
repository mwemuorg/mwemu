use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn BCryptGetProperty(emu: &mut emu::Emu) {
    let h_object = emu.regs().rcx;
    let psz_property = emu.regs().rdx;
    let pb_output = emu.regs().r8;
    let cb_output = emu.regs().r9 as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let prop_name = emu.maps.read_wide_string(psz_property);

    let algo_uri = helper::handler_get_uri(h_object);
    let algo_name = algo_uri
        .strip_prefix("bcrypt://")
        .or_else(|| algo_uri.strip_prefix("bcrypthash://"))
        .unwrap_or(&algo_uri);

    let value: u32 = match prop_name.as_str() {
        "HashDigestLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 32,
            "SHA1" => 20,
            "MD5" => 16,
            _ => 0,
        },
        "ObjectLength" => 128,
        "BlockLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 64,
            "SHA1" => 64,
            "MD5" => 64,
            _ => 8,
        },
        _ => 0,
    };

    if pb_output != 0 && cb_output >= 4 {
        emu.maps.write_dword(pb_output, value);
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 4);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGetProperty obj=0x{:x} prop={} => {}",
        h_object,
        prop_name,
        value
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
