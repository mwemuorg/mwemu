use super::read_guid_string;
use crate::emu;

pub fn StringFromGUID2(emu: &mut emu::Emu) {
    let rguid = emu.regs().rcx;
    let lpsz = emu.regs().rdx;
    let cch_max = emu.regs().r8 as i32;

    let guid_str = read_guid_string(emu, rguid);
    log_red!(
        emu,
        "ole32!StringFromGUID2 {} buf: 0x{:x} max: {}",
        guid_str,
        lpsz,
        cch_max
    );

    let needed = 39; // "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" = 38 + null = 39
    if cch_max < needed {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    if lpsz != 0 {
        emu.maps.write_wide_string(lpsz, &guid_str);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = needed as u64;
}
