use super::read_guid_string;
use crate::emu;

pub fn StringFromIID(emu: &mut emu::Emu) {
    let iid_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;
    let guid_str = read_guid_string(emu, iid_ptr);
    if out_ptr != 0 {
        let alloc_sz: u64 = 39 * 2 + 2;
        if let Some(buf) = emu.maps.alloc(alloc_sz) {
            emu.maps.create_map(
                &format!("iid_str_{:x}", buf),
                buf,
                alloc_sz,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_wide_string(buf, &guid_str);
            emu.maps.write_dword(out_ptr, buf as u32);
        } else {
            emu.maps.write_dword(out_ptr, 0);
        }
    }
    log_red!(emu, "ole32!StringFromIID => '{}'", guid_str);
    emu.regs_mut().rax = 0;
}
