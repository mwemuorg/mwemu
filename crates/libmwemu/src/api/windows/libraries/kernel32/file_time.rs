use std::cmp::Ordering;

use chrono::{Datelike, NaiveDate, Timelike};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::common::memory::read_array;
use crate::winapi::common::strings::write_text;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::{
    DATE_SHORTDATE, EN_US_LOCALE, LOCALE_SYSTEM_DEFAULT, LOCALE_USER_DEFAULT,
};
use crate::windows::constants::time::*;
use crate::windows::structures::SystemTime;

fn read_file_time(emu: &Emu, input: u64) -> Result<u64, u64> {
    if input == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    read_array(emu, input)
        .map(u64::from_le_bytes)
        .ok_or(ERROR_NOACCESS)
}

fn write_time(emu: &mut Emu, output: u64, bytes: &[u8]) -> Result<(), u64> {
    if output == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    emu.maps
        .write_bytes(output, bytes)
        .then_some(())
        .ok_or(ERROR_NOACCESS)
}

fn return_conversion(emu: &mut Emu, api: &str, words: usize, result: Result<(), u64>) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!{api} => {result:?}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok() as u64);
    call.cleanup_args(emu, words);
}

pub fn FileTimeToSystemTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = (|| {
        let value = read_file_time(emu, input)?;
        let date = SystemTime::from_file_time(value).ok_or(ERROR_INVALID_PARAMETER)?;
        write_time(emu, output, &date.to_bytes())
    })();
    return_conversion(emu, "FileTimeToSystemTime", 2, result);
}

pub fn SystemTimeToFileTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = (|| {
        if input == 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let date = SystemTime::from_bytes(read_array(emu, input).ok_or(ERROR_NOACCESS)?);
        let value = date.file_time().ok_or(ERROR_INVALID_PARAMETER)?;
        write_time(emu, output, &value.to_le_bytes())
    })();
    return_conversion(emu, "SystemTimeToFileTime", 2, result);
}

pub fn FileTimeToDosDateTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let date_output = api_try!(emu, call.arg(emu, 1));
    let time_output = api_try!(emu, call.arg(emu, 2));
    let result = (|| {
        let value = read_file_time(emu, input)?;
        // FAT timestamps round upward to the next two-second boundary, including subsecond ticks.
        let rounded = value
            .checked_add(DOS_TIME_TICKS - 1)
            .ok_or(ERROR_INVALID_PARAMETER)?
            / DOS_TIME_TICKS
            * DOS_TIME_TICKS;
        let date = SystemTime::from_file_time(rounded)
            .and_then(|value| value.datetime())
            .ok_or(ERROR_INVALID_PARAMETER)?;
        if !(DOS_YEAR_START..=DOS_YEAR_END).contains(&date.year()) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if date_output == 0 || time_output == 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !emu
            .maps
            .validate_write_range(date_output, size_of::<u16>() as u64)
            || !emu
                .maps
                .validate_write_range(time_output, size_of::<u16>() as u64)
        {
            return Err(ERROR_NOACCESS);
        }
        let fat_date = ((date.year() - DOS_YEAR_START) as u16) << DOS_DATE_YEAR_SHIFT
            | (date.month() as u16) << DOS_DATE_MONTH_SHIFT
            | date.day() as u16;
        let fat_time = (date.hour() as u16) << DOS_TIME_HOUR_SHIFT
            | (date.minute() as u16) << DOS_TIME_MINUTE_SHIFT
            | (date.second() / 2) as u16;
        write_time(emu, date_output, &fat_date.to_le_bytes())?;
        write_time(emu, time_output, &fat_time.to_le_bytes())
    })();
    return_conversion(emu, "FileTimeToDosDateTime", 3, result);
}

pub fn DosDateTimeToFileTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let date = api_try!(emu, call.arg_u16(emu, 0));
    let time = api_try!(emu, call.arg_u16(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let result = (|| {
        let year = DOS_YEAR_START + (date >> DOS_DATE_YEAR_SHIFT) as i32;
        let month = ((date >> DOS_DATE_MONTH_SHIFT) & DOS_DATE_MONTH_MASK) as u32;
        let day = (date & DOS_DATE_DAY_MASK) as u32;
        let hour = (time >> DOS_TIME_HOUR_SHIFT) as u32;
        let minute = ((time >> DOS_TIME_MINUTE_SHIFT) & DOS_TIME_MINUTE_MASK) as u32;
        let second = (time & DOS_TIME_SECOND_MASK) as u32 * 2;
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
            .ok_or(ERROR_INVALID_PARAMETER)?;
        let value = SystemTime::from_datetime(date)
            .file_time()
            .ok_or(ERROR_INVALID_PARAMETER)?;
        write_time(emu, output, &value.to_le_bytes())
    })();
    return_conversion(emu, "DosDateTimeToFileTime", 3, result);
}

pub fn CompareFileTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!CompareFileTime left={left:#x} right={right:#x}"
    );
    let (Ok(left), Ok(right)) = (read_file_time(emu, left), read_file_time(emu, right)) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    let result = match left.cmp(&right) {
        Ordering::Less => -1i32,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    call.set_return(emu, result as u32 as u64);
    call.cleanup_args(emu, 2);
}

// The guest clock starts on 2024-01-01 UTC. The modeled local time zone is UTC.
const GUEST_EPOCH_FILETIME: u64 = 133_485_408_000_000_000;

pub(crate) fn guest_file_time(emu: &Emu) -> u64 {
    GUEST_EPOCH_FILETIME.saturating_add(performance_count(emu))
}
fn performance_count(emu: &Emu) -> u64 {
    (emu.tick as u64).saturating_mul(FILETIME_TICKS_PER_MILLISECOND)
}

fn clock_output(emu: &mut Emu, api: &str, structure: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let value = guest_file_time(emu);
    let written = if structure {
        SystemTime::from_file_time(value)
            .is_some_and(|time| emu.maps.write_bytes(address, &time.to_bytes()))
    } else {
        emu.maps.write_bytes(address, &value.to_le_bytes())
    };
    log_red!(emu, "{api} output={address:#x}");
    if !written {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.cleanup_args(emu, 1);
}
pub fn GetLocalTime(emu: &mut Emu) {
    clock_output(emu, "GetLocalTime", true);
}
pub fn GetSystemTime(emu: &mut Emu) {
    clock_output(emu, "GetSystemTime", true);
}
pub fn GetSystemTimeAsFileTime(emu: &mut Emu) {
    clock_output(emu, "GetSystemTimeAsFileTime", false);
}
pub fn GetSystemTimePreciseAsFileTime(emu: &mut Emu) {
    clock_output(emu, "GetSystemTimePreciseAsFileTime", false);
}

pub(crate) fn nt_performance(emu: &mut Emu, call: WindowsAbi) {
    let [counter, frequency] = api_try!(emu, call.args(emu));
    let size = size_of::<u64>() as u64;
    let status = if counter == 0 {
        STATUS_INVALID_PARAMETER
    } else if !emu.maps.validate_write_range(counter, size)
        || (frequency != 0 && !emu.maps.validate_write_range(frequency, size))
    {
        STATUS_ACCESS_VIOLATION
    } else {
        let ticks = performance_count(emu);
        emu.maps.write_qword(counter, ticks);
        if frequency != 0 {
            emu.maps.write_qword(frequency, FILETIME_TICKS_PER_SECOND);
        }
        STATUS_SUCCESS
    };
    log_red!(
        emu,
        "NtQueryPerformanceCounter counter={counter:#x} frequency={frequency:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}
pub(crate) fn win_performance(emu: &mut Emu, frequency: bool) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let (name, value) = if frequency {
        ("QueryPerformanceFrequency", FILETIME_TICKS_PER_SECOND)
    } else {
        ("QueryPerformanceCounter", performance_count(emu))
    };
    log_red!(emu, "{name} out={ptr:#x}");
    let ok = ptr != 0 && emu.maps.write_qword(ptr, value);
    if !ok {
        emu.set_last_error(ERROR_NOACCESS);
    }
    call.set_return(emu, u64::from(ok));
    call.cleanup_args(emu, 1);
}

pub(crate) fn native_delay(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let _alertable = api_try!(emu, call.arg_u8(emu, 0));
    let pointer = api_try!(emu, call.arg(emu, 1));
    let status = if let Some(raw) = emu.maps.read_qword(pointer) {
        let interval = raw as i64;
        let duration = if interval < 0 {
            interval.unsigned_abs()
        } else {
            raw.saturating_sub(guest_file_time(emu))
        };
        let millis = usize::try_from(duration.div_ceil(FILETIME_TICKS_PER_MILLISECOND))
            .unwrap_or(usize::MAX);
        emu.tick = emu.tick.saturating_add(millis);
        STATUS_SUCCESS
    } else {
        STATUS_ACCESS_VIOLATION
    };
    log_red!(
        emu,
        "NtDelayExecution interval={pointer:#x} status={status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}

pub(crate) fn interrupt_time(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let time = performance_count(emu);
    let written =
        ptr != 0 && emu.maps.validate_write_range(ptr, 8) && emu.maps.write_qword(ptr, time);
    log_red!(emu, "{api} => {time}");
    if api == "QueryUnbiasedInterruptTime" {
        if !written {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
        }
        call.set_return(emu, u64::from(written));
    } else if !written {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.cleanup_args(emu, 1);
}

pub(crate) fn format_local(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let [locale, flags, ptr, pattern, out, capacity] = api_try!(emu, call.args(emu));
    let date = api.starts_with("GetDateFormat");
    let ex = api.ends_with("Ex");
    let calendar = if date && ex {
        api_try!(emu, call.arg(emu, 6))
    } else {
        0
    };
    let locale_known = if ex {
        locale == 0
            || Encoding::Wide
                .read(emu, locale)
                .is_ok_and(|units| String::from_utf16_lossy(&units).eq_ignore_ascii_case("en-US"))
    } else {
        [
            LOCALE_USER_DEFAULT,
            LOCALE_SYSTEM_DEFAULT,
            u64::from(EN_US_LOCALE),
        ]
        .contains(&u64::from(locale as u32))
    };
    if !locale_known
        || pattern != 0
        || calendar != 0
        || !(flags as u32 == 0 || date && flags as u32 == DATE_SHORTDATE)
    {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let result = (|| {
        let bytes = if ptr == 0 {
            SystemTime::from_file_time(guest_file_time(emu))
                .ok_or(ERROR_INVALID_PARAMETER)?
                .to_bytes()
        } else {
            read_array::<SYSTEMTIME_BYTES>(emu, ptr).ok_or(ERROR_NOACCESS)?
        };
        let [year, month, _, day, hour, minute, second, _]: [u16; SYSTEMTIME_FIELDS] =
            std::array::from_fn(|i| u16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]));
        if date {
            NaiveDate::from_ymd_opt(i32::from(year), u32::from(month), u32::from(day))
                .ok_or(ERROR_INVALID_PARAMETER)?;
            Ok(format!("{month}/{day}/{year:04}"))
        } else {
            if hour >= 24 || minute >= 60 || second >= 60 {
                return Err(ERROR_INVALID_PARAMETER);
            }
            let marker = if hour < 12 { "AM" } else { "PM" };
            let hour = if hour.is_multiple_of(12) {
                12
            } else {
                hour % 12
            };
            Ok(format!("{hour}:{minute:02}:{second:02} {marker}"))
        }
    })();
    let length = match result {
        Ok(text) => write_text(emu, out, capacity as i32, &text, ex || api.ends_with('W')),
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    log_red!(emu, "{api} result={length}");
    call.set_return(emu, length);
    call.cleanup_args(emu, 6 + usize::from(date && ex));
}
