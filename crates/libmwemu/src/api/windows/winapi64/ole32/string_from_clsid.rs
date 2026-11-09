use super::read_guid_string;
use crate::emu;

pub fn StringFromCLSID(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let ppsz = emu.regs().rdx;

    let guid_str = read_guid_string(emu, rclsid);
    log_red!(emu, "ole32!StringFromCLSID {} ppsz: 0x{:x}", guid_str, ppsz);

    // Allocate wide string for the GUID "{...}\0" = 39 chars
    let alloc_sz = 39 * 2 + 2;
    let buf = match emu.maps.alloc(alloc_sz) {
        Some(a) => a,
        None => {
            if ppsz != 0 {
                emu.maps.write_qword(ppsz, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x8007000E; // E_OUTOFMEMORY
            return;
        }
    };
    emu.maps.create_map(
        &format!("guid_str_{:x}", buf),
        buf,
        alloc_sz,
        crate::maps::mem64::Permission::READ_WRITE,
    );

    emu.maps.write_wide_string(buf, &guid_str);

    if ppsz != 0 {
        emu.maps.write_qword(ppsz, buf);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
