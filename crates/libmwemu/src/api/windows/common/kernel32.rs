pub(crate) fn is_api_set_contract(module: &str) -> bool {
    let module = module.trim().to_ascii_lowercase();
    module.starts_with("api-ms-win-") || module.starts_with("ext-ms-")
}

pub(crate) fn normalize_library_name(libname: &str, allow_exe_suffix: bool) -> Option<String> {
    let mut dll = libname.trim().to_ascii_lowercase();
    if dll.is_empty() {
        return None;
    }

    if allow_exe_suffix {
        if !dll.ends_with(".dll") && !dll.ends_with(".exe") {
            dll.push_str(".dll");
        }
    } else if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    Some(dll)
}

pub(crate) fn module_name_matches(flink_mod: &str, want: &str) -> bool {
    normalize_module_name(flink_mod) == normalize_module_name(want)
}

fn normalize_module_name(module_name: &str) -> String {
    let module_name = module_name.trim().to_ascii_lowercase();
    let module_name = module_name
        .rsplit_once('\\')
        .map(|(_, name)| name)
        .unwrap_or(&module_name);
    let module_name = module_name
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(module_name);
    module_name
        .strip_suffix(".dll")
        .unwrap_or(module_name)
        .to_string()
}

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
const LOCALE_SDAYNAME1: u64 = 0x2A;
const LOCALE_SDAYNAME2: u64 = 0x2B;
const LOCALE_SDAYNAME3: u64 = 0x2C;
const LOCALE_SDAYNAME4: u64 = 0x2D;
const LOCALE_SDAYNAME5: u64 = 0x2E;
const LOCALE_SDAYNAME6: u64 = 0x2F;
const LOCALE_SDAYNAME7: u64 = 0x30;
const LOCALE_SABBREVDAYNAME1: u64 = 0x31;
const LOCALE_SABBREVDAYNAME2: u64 = 0x32;
const LOCALE_SABBREVDAYNAME3: u64 = 0x33;
const LOCALE_SABBREVDAYNAME4: u64 = 0x34;
const LOCALE_SABBREVDAYNAME5: u64 = 0x35;
const LOCALE_SABBREVDAYNAME6: u64 = 0x36;
const LOCALE_SABBREVDAYNAME7: u64 = 0x37;
const LOCALE_SMONTHNAME1: u64 = 0x38;
const LOCALE_SMONTHNAME2: u64 = 0x39;
const LOCALE_SMONTHNAME3: u64 = 0x3A;
const LOCALE_SMONTHNAME4: u64 = 0x3B;
const LOCALE_SMONTHNAME5: u64 = 0x3C;
const LOCALE_SMONTHNAME6: u64 = 0x3D;
const LOCALE_SMONTHNAME7: u64 = 0x3E;
const LOCALE_SMONTHNAME8: u64 = 0x3F;
const LOCALE_SMONTHNAME9: u64 = 0x40;
const LOCALE_SMONTHNAME10: u64 = 0x41;
const LOCALE_SMONTHNAME11: u64 = 0x42;
const LOCALE_SMONTHNAME12: u64 = 0x43;
const LOCALE_SABBREVMONTHNAME1: u64 = 0x44;
const LOCALE_SABBREVMONTHNAME2: u64 = 0x45;
const LOCALE_SABBREVMONTHNAME3: u64 = 0x46;
const LOCALE_SABBREVMONTHNAME4: u64 = 0x47;
const LOCALE_SABBREVMONTHNAME5: u64 = 0x48;
const LOCALE_SABBREVMONTHNAME6: u64 = 0x49;
const LOCALE_SABBREVMONTHNAME7: u64 = 0x4A;
const LOCALE_SABBREVMONTHNAME8: u64 = 0x4B;
const LOCALE_SABBREVMONTHNAME9: u64 = 0x4C;
const LOCALE_SABBREVMONTHNAME10: u64 = 0x4D;
const LOCALE_SABBREVMONTHNAME11: u64 = 0x4E;
const LOCALE_SABBREVMONTHNAME12: u64 = 0x4F;
const LOCALE_SPOSITIVESIGN: u64 = 0x50;
const LOCALE_SNEGATIVESIGN: u64 = 0x51;
const LOCALE_STIMEFORMAT: u64 = 0x1003;
const LOCALE_IDEFAULTANSICODEPAGE: u64 = 0x1004;
const LOCALE_SYEARMONTH: u64 = 0x1006;
const LOCALE_SENGCURRNAME: u64 = 0x1007;
const LOCALE_SNATIVECURRNAME: u64 = 0x1008;
const LOCALE_INEGNUMBER: u64 = 0x1010;
const LOCALE_IDEFAULTMACCODEPAGE: u64 = 0x1011;
const LOCALE_SSORTNAME: u64 = 0x1013;
const LOCALE_IDIGITSUBSTITUTION: u64 = 0x1014;
const LOCALE_ICALENDARTYPE: u64 = 0x100A;
const LOCALE_IOPTIONALCALENDAR: u64 = 0x100B;
const LOCALE_IFIRSTDAYOFWEEK: u64 = 0x100C;
const LOCALE_IFIRSTWEEKOFYEAR: u64 = 0x100D;
const LOCALE_RETURN_NUMBER: u64 = 0x20000000;

/// The en-US answer for a `GetLocaleInfo*` LCTYPE, shared by the 32- and
/// 64-bit stubs; unknown types fall back to "." and are logged by the caller.
pub(crate) fn locale_info_value(lctype: u64) -> Option<&'static str> {
    Some(match lctype {
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
        _ => return None,
    })
}
