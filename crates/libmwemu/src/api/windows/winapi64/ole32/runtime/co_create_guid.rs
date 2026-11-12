use crate::emu;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn CoCreateGuid(emu: &mut emu::Emu) {
    let pguid = emu.regs().rcx;

    log_red!(emu, "ole32!CoCreateGuid ptr: 0x{:x}", pguid);

    if pguid == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
        return;
    }

    // Write a deterministic GUID (version 4 format)
    // Using emu.pos as seed for reproducibility
    let seed = emu.pos as u32;
    emu.maps
        .write_dword(pguid, seed.wrapping_mul(0x6C078965).wrapping_add(1));
    emu.maps
        .write_word(pguid + 4, ((seed & 0xFFFF) ^ 0x4000) as u16);
    emu.maps
        .write_word(pguid + 6, (0x4000 | (seed & 0x0FFF)) as u16); // version 4
    emu.maps
        .write_byte(pguid + 8, (0x80 | ((seed >> 16) & 0x3F)) as u8); // variant 1
    emu.maps.write_byte(pguid + 9, (seed >> 24) as u8);
    for i in 0..6u64 {
        emu.maps
            .write_byte(pguid + 10 + i, ((seed.wrapping_add(i as u32)) & 0xFF) as u8);
    }

    let guid_str = read_guid_string(emu, pguid);
    log_red!(emu, "CoCreateGuid -> {}", guid_str);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
