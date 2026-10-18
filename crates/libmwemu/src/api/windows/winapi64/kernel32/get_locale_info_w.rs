use crate::winapi::winapi64::kernel32::{LAST_ERROR, clear_last_error};
use crate::{emu, windows::constants};

// FROM THE ASSEMBLY FILE - THESE ARE THE CORRECT VALUES:
const LOCALE_SDAYNAME1: u64 = 0x2A; // Monday
const LOCALE_SDAYNAME2: u64 = 0x2B; // Tuesday
const LOCALE_SDAYNAME3: u64 = 0x2C; // Wednesday
const LOCALE_SDAYNAME4: u64 = 0x2D; // Thursday
const LOCALE_SDAYNAME5: u64 = 0x2E; // Friday
const LOCALE_SDAYNAME6: u64 = 0x2F; // Saturday
const LOCALE_SDAYNAME7: u64 = 0x30; // Sunday
const LOCALE_SABBREVDAYNAME1: u64 = 0x31; // Monday abbreviated
const LOCALE_SABBREVDAYNAME2: u64 = 0x32; // Tuesday abbreviated
const LOCALE_SABBREVDAYNAME3: u64 = 0x33; // Wednesday abbreviated
const LOCALE_SABBREVDAYNAME4: u64 = 0x34; // Thursday abbreviated
const LOCALE_SABBREVDAYNAME5: u64 = 0x35; // Friday abbreviated
const LOCALE_SABBREVDAYNAME6: u64 = 0x36; // Saturday abbreviated
const LOCALE_SABBREVDAYNAME7: u64 = 0x37; // Sunday abbreviated
const LOCALE_SMONTHNAME1: u64 = 0x38; // January
const LOCALE_SMONTHNAME2: u64 = 0x39; // February
const LOCALE_SMONTHNAME3: u64 = 0x3A; // March
const LOCALE_SMONTHNAME4: u64 = 0x3B; // April
const LOCALE_SMONTHNAME5: u64 = 0x3C; // May
const LOCALE_SMONTHNAME6: u64 = 0x3D; // June
const LOCALE_SMONTHNAME7: u64 = 0x3E; // July
const LOCALE_SMONTHNAME8: u64 = 0x3F; // August
const LOCALE_SMONTHNAME9: u64 = 0x40; // September
const LOCALE_SMONTHNAME10: u64 = 0x41; // October
const LOCALE_SMONTHNAME11: u64 = 0x42; // November
const LOCALE_SMONTHNAME12: u64 = 0x43; // December
const LOCALE_SABBREVMONTHNAME1: u64 = 0x44; // January abbreviated
const LOCALE_SABBREVMONTHNAME2: u64 = 0x45; // February abbreviated
const LOCALE_SABBREVMONTHNAME3: u64 = 0x46; // March abbreviated
const LOCALE_SABBREVMONTHNAME4: u64 = 0x47; // April abbreviated
const LOCALE_SABBREVMONTHNAME5: u64 = 0x48; // May abbreviated
const LOCALE_SABBREVMONTHNAME6: u64 = 0x49; // June abbreviated
const LOCALE_SABBREVMONTHNAME7: u64 = 0x4A; // July abbreviated
const LOCALE_SABBREVMONTHNAME8: u64 = 0x4B; // August abbreviated
const LOCALE_SABBREVMONTHNAME9: u64 = 0x4C; // September abbreviated
const LOCALE_SABBREVMONTHNAME10: u64 = 0x4D; // October abbreviated
const LOCALE_SABBREVMONTHNAME11: u64 = 0x4E; // November abbreviated
const LOCALE_SABBREVMONTHNAME12: u64 = 0x4F; // December abbreviated
const LOCALE_SPOSITIVESIGN: u64 = 0x50; // Positive sign
const LOCALE_SNEGATIVESIGN: u64 = 0x51; // Negative sign

// Additional constants:

// These don't have conflicts anymore, but removing duplicates:
const LOCALE_ICALENDARTYPE: u64 = 0x100A; // Calendar type (1=Gregorian) - MADE UP VALUE
const LOCALE_IOPTIONALCALENDAR: u64 = 0x100B; // Optional calendar type - MADE UP VALUE
const LOCALE_IFIRSTDAYOFWEEK: u64 = 0x100C; // First day of week (0=Monday, 6=Sunday) - MADE UP VALUE
const LOCALE_IFIRSTWEEKOFYEAR: u64 = 0x100D; // First week of year - MADE UP VALUE

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let lctype = emu.regs().rdx;
    let lp_lc_data = emu.regs().r8 as usize;
    let cch_data = emu.regs().r9 as usize;

    log_red!(
        emu,
        "** {} kernel32!GetLocaleInfoW locale: 0x{:x} lctype: 0x{:x} lp_lc_data: 0x{:x} cch_data: {}",
        emu.pos,
        locale,
        lctype,
        lp_lc_data,
        cch_data
    );

    let result = match crate::api::windows::common::kernel32::locale_info_value(lctype) {
        Some(v) => v,
        None => {
            log::warn!(
                "{} GetLocaleInfoW unhandled lctype: 0x{:x}",
                emu.pos,
                lctype
            );
            "." // Default fallback
        }
    };

    let required_size = result.len() + 1; // Include null terminator

    // Check if it wants buffer size
    if cch_data == 0 {
        emu.regs_mut().rax = required_size as u64;
        clear_last_error(emu);
        return;
    }

    // Validate buffer pointer
    if lp_lc_data == 0 {
        log::warn!("{} GetLocaleInfoW invalid parameter - null buffer", emu.pos);
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INVALID_PARAMETER;
        emu.regs_mut().rax = 0;
        return;
    }

    // Check if buffer is too small
    if cch_data < required_size {
        log::warn!(
            "{} buffer too small for result cch_data: {} required_size: {}",
            emu.pos,
            cch_data,
            required_size
        );
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INSUFFICIENT_BUFFER;
        emu.regs_mut().rax = 0;
        return;
    }

    // Write the result to the buffer
    emu.maps.write_wide_string(lp_lc_data as u64, result);
    emu.regs_mut().rax = required_size as u64;
    clear_last_error(emu);
}
