use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptSetProperty(emu: &mut emu::Emu) {
    let h_object = arg(emu, 0) as u64;
    let psz_property = arg(emu, 1) as u64;
    let pb_input = arg(emu, 2) as u64;
    let cb_input = arg(emu, 3) as usize;

    let prop = emu.maps.read_wide_string(psz_property);
    let value = if pb_input != 0 && cb_input > 0 {
        let val_bytes = emu.maps.read_bytes(pb_input, cb_input.min(256)).to_vec();
        String::from_utf16_lossy(
            &val_bytes
                .chunks(2)
                .filter_map(|c| {
                    if c.len() == 2 {
                        let w = u16::from_le_bytes([c[0], c[1]]);
                        if w != 0 { Some(w) } else { None }
                    } else {
                        None
                    }
                })
                .collect::<Vec<u16>>(),
        )
    } else {
        String::new()
    };
    log_red!(
        emu,
        "bcrypt!BCryptSetProperty obj=0x{:x} prop={} value={}",
        h_object,
        prop,
        value
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 5);
}
