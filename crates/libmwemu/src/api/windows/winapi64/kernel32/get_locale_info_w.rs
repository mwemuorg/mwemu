use crate::winapi::winapi64::kernel32::{LAST_ERROR, clear_last_error};
use crate::{emu, windows::constants};

// FROM THE ASSEMBLY FILE - THESE ARE THE CORRECT VALUES:
const LOCALE_ILANGUAGE: u64 = 0x1;
const LOCALE_SLANGUAGE: u64 = 0x2;
const LOCALE_SABBREVLANGNAME: u64 = 0x3;
const LOCALE_SNATIVELANGNAME: u64 = 0x4;
const LOCALE_ICOUNTRY: u64 = 0x5;
const LOCALE_SCOUNTRY: u64 = 0x6;
const LOCALE_SABBREVCTRYNAME: u64 = 0x7;
const LOCALE_SNATIVECTRYNAME: u64 = 0x8;
const LOCALE_IDEFAULTLANGUAGE: u64 = 0x9;
const LOCALE_IDEFAULTCOUNTRY: u64 = 0xA;
const LOCALE_IDEFAULTCODEPAGE: u64 = 0xB;
const LOCALE_SLIST: u64 = 0xC;
const LOCALE_IMEASURE: u64 = 0xD;
const LOCALE_SDECIMAL: u64 = 0xE;
const LOCALE_STHOUSAND: u64 = 0xF;
const LOCALE_SGROUPING: u64 = 0x10;
const LOCALE_IDIGITS: u64 = 0x11;
const LOCALE_ILZERO: u64 = 0x12;
const LOCALE_SNATIVEDIGITS: u64 = 0x13;
const LOCALE_SCURRENCY: u64 = 0x14;
const LOCALE_SINTLSYMBOL: u64 = 0x15;
const LOCALE_SMONDECIMALSEP: u64 = 0x16;
const LOCALE_SMONTHOUSANDSEP: u64 = 0x17;
const LOCALE_SMONGROUPING: u64 = 0x18;
const LOCALE_ICURRDIGITS: u64 = 0x19;
const LOCALE_IINTLCURRDIGITS: u64 = 0x1A;
const LOCALE_ICURRENCY: u64 = 0x1B;
const LOCALE_INEGCURR: u64 = 0x1C;
const LOCALE_SDATE: u64 = 0x1D;
const LOCALE_STIME: u64 = 0x1E;
const LOCALE_SSHORTDATE: u64 = 0x1F;
const LOCALE_SLONGDATE: u64 = 0x20;
const LOCALE_IDATE: u64 = 0x21;
const LOCALE_ILDATE: u64 = 0x22;
const LOCALE_ITIME: u64 = 0x23;
const LOCALE_ICENTURY: u64 = 0x24;
const LOCALE_ITLZERO: u64 = 0x25;
const LOCALE_IDAYLZERO: u64 = 0x26;
const LOCALE_IMONLZERO: u64 = 0x27;
const LOCALE_S1159: u64 = 0x28;
const LOCALE_S2359: u64 = 0x29;
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
const LOCALE_STIMEFORMAT: u64 = 0x1003;
const LOCALE_IDEFAULTANSICODEPAGE: u64 = 0x1004;
const LOCALE_SYEARMONTH: u64 = 0x1006;
const LOCALE_SENGCURRNAME: u64 = 0x1007;
const LOCALE_SNATIVECURRNAME: u64 = 0x1008;
const LOCALE_INEGNUMBER: u64 = 0x1010;
const LOCALE_IDEFAULTMACCODEPAGE: u64 = 0x1011;
const LOCALE_SSORTNAME: u64 = 0x1013;
const LOCALE_IDIGITSUBSTITUTION: u64 = 0x1014;

// These don't have conflicts anymore, but removing duplicates:
const LOCALE_ICALENDARTYPE: u64 = 0x100A; // Calendar type (1=Gregorian) - MADE UP VALUE
const LOCALE_IOPTIONALCALENDAR: u64 = 0x100B; // Optional calendar type - MADE UP VALUE
const LOCALE_IFIRSTDAYOFWEEK: u64 = 0x100C; // First day of week (0=Monday, 6=Sunday) - MADE UP VALUE
const LOCALE_IFIRSTWEEKOFYEAR: u64 = 0x100D; // First week of year - MADE UP VALUE
const LOCALE_RETURN_NUMBER: u64 = 0x20000000;

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

    let result = match lctype {
        LOCALE_SLANGUAGE => "English",
        LOCALE_SCOUNTRY => "United States",
        LOCALE_SLIST => ",",
        LOCALE_SDECIMAL => ".",
        LOCALE_STHOUSAND => ",",
        LOCALE_SCURRENCY => "$",
        LOCALE_SDATE => "/",
        LOCALE_STIME => ":",
        LOCALE_ICURRDIGITS => "2",
        LOCALE_IINTLCURRDIGITS => "2",
        LOCALE_ICURRENCY => "0",
        LOCALE_INEGCURR => "0",
        LOCALE_SSHORTDATE => "M/d/yyyy",
        LOCALE_SLONGDATE => "dddd, MMMM d, yyyy",
        LOCALE_IDATE => "0",
        LOCALE_ILDATE => "0",
        LOCALE_ITIME => "0",
        LOCALE_ICENTURY => "1",
        LOCALE_ITLZERO => "0",
        LOCALE_IDAYLZERO => "0",
        LOCALE_IMONLZERO => "0",
        LOCALE_S1159 => "AM",
        LOCALE_S2359 => "PM",
        LOCALE_ICALENDARTYPE => "1",
        LOCALE_IOPTIONALCALENDAR => "1",
        LOCALE_IFIRSTDAYOFWEEK => "6",
        LOCALE_IFIRSTWEEKOFYEAR => "0",
        LOCALE_ICOUNTRY => "1",
        LOCALE_SDAYNAME1 => "Monday",
        LOCALE_SDAYNAME2 => "Tuesday",
        LOCALE_SDAYNAME3 => "Wednesday",
        LOCALE_SDAYNAME4 => "Thursday",
        LOCALE_SDAYNAME5 => "Friday",
        LOCALE_SDAYNAME6 => "Saturday",
        LOCALE_SDAYNAME7 => "Sunday",
        LOCALE_SMONTHNAME1 => "January",
        LOCALE_SMONTHNAME2 => "February",
        LOCALE_SMONTHNAME3 => "March",
        LOCALE_SMONTHNAME4 => "April",
        LOCALE_SMONTHNAME5 => "May",
        LOCALE_SMONTHNAME6 => "June",
        LOCALE_SMONTHNAME7 => "July",
        LOCALE_SMONTHNAME8 => "August",
        LOCALE_SMONTHNAME9 => "September",
        LOCALE_SMONTHNAME10 => "October",
        LOCALE_SMONTHNAME11 => "November",
        LOCALE_SMONTHNAME12 => "December",
        LOCALE_SABBREVMONTHNAME1 => "Jan",
        LOCALE_SABBREVMONTHNAME2 => "Feb",
        LOCALE_SABBREVMONTHNAME3 => "Mar",
        LOCALE_SABBREVMONTHNAME4 => "Apr",
        LOCALE_SABBREVMONTHNAME5 => "May",
        LOCALE_SABBREVMONTHNAME6 => "Jun",
        LOCALE_SABBREVMONTHNAME7 => "Jul",
        LOCALE_SABBREVMONTHNAME8 => "Aug",
        LOCALE_SABBREVMONTHNAME9 => "Sep",
        LOCALE_SABBREVMONTHNAME10 => "Oct",
        LOCALE_SABBREVMONTHNAME11 => "Nov",
        LOCALE_SABBREVMONTHNAME12 => "Dec",
        LOCALE_SABBREVDAYNAME1 => "Mon",
        LOCALE_SABBREVDAYNAME2 => "Tue",
        LOCALE_SABBREVDAYNAME3 => "Wed",
        LOCALE_SABBREVDAYNAME4 => "Thu",
        LOCALE_SABBREVDAYNAME5 => "Fri",
        LOCALE_SABBREVDAYNAME6 => "Sat",
        LOCALE_SABBREVDAYNAME7 => "Sun",

        // Additional commonly used constants:
        LOCALE_ILANGUAGE => "0409",
        LOCALE_IDEFAULTLANGUAGE => "0409",
        LOCALE_IDEFAULTCOUNTRY => "1",
        LOCALE_IDEFAULTCODEPAGE => "1252",
        LOCALE_IMEASURE => "1",
        LOCALE_SGROUPING => "3;0",
        LOCALE_IDIGITS => "2",
        LOCALE_ILZERO => "1",
        LOCALE_SINTLSYMBOL => "USD",
        LOCALE_SMONDECIMALSEP => ".",
        LOCALE_SMONTHOUSANDSEP => ",",
        LOCALE_SMONGROUPING => "3;0",
        LOCALE_SPOSITIVESIGN => "",
        LOCALE_SNEGATIVESIGN => "-",
        LOCALE_STIMEFORMAT => "h:mm:ss tt",
        LOCALE_IDEFAULTANSICODEPAGE => "1252",
        LOCALE_SYEARMONTH => "MMMM yyyy",
        LOCALE_SENGCURRNAME => "US Dollar",
        LOCALE_SNATIVECURRNAME => "US Dollar",
        LOCALE_INEGNUMBER => "1",
        LOCALE_IDEFAULTMACCODEPAGE => "10000",
        LOCALE_SSORTNAME => "Default",
        LOCALE_IDIGITSUBSTITUTION => "1",

        _ => {
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
