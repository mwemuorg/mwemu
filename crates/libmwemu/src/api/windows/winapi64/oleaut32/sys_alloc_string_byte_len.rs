use crate::emu;
use crate::maps::mem64::Permission;

pub(super) fn SysAllocStringByteLen(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;
    let len = emu.regs().rdx;

    log_red!(
        emu,
        "oleaut32!SysAllocStringByteLen psz: 0x{:x} len: {}",
        psz,
        len
    );

    let total = 4 + len + 2 + 16;

    let base = match emu.maps.alloc(total) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_alloc_{:x}", base);
    emu.maps
        .create_map(&name, base, total, Permission::READ_WRITE);

    emu.maps.write_dword(base, len as u32);

    if psz != 0 {
        for i in 0..len {
            let b = emu.maps.read_byte(psz + i).unwrap_or(0);
            emu.maps.write_byte(base + 4 + i, b);
        }
    }
    emu.maps.write_word(base + 4 + len, 0);

    let bstr = base + 4;
    emu.set_last_error(0);
    emu.regs_mut().rax = bstr;
}
