use crate::emu;
use crate::maps::mem64::Permission;

pub fn SysAllocString(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;

    if psz == 0 {
        log_red!(emu, "oleaut32!SysAllocString NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let s = emu.maps.read_wide_string(psz);
    let char_count = s.encode_utf16().count() as u64;

    log_red!(
        emu,
        "oleaut32!SysAllocString '{}' ({} chars)",
        s,
        char_count
    );

    let byte_len = char_count * 2;
    let total = 4 + byte_len + 2 + 16;

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

    emu.maps.write_dword(base, byte_len as u32);

    for (i, ch) in s.encode_utf16().enumerate() {
        emu.maps.write_word(base + 4 + (i as u64 * 2), ch);
    }
    emu.maps.write_word(base + 4 + byte_len, 0);

    let bstr = base + 4;
    log_red!(emu, "SysAllocString -> 0x{:x}", bstr);
    emu.set_last_error(0);
    emu.regs_mut().rax = bstr;
}
