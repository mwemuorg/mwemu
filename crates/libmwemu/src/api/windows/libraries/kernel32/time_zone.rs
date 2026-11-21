use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_array;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;
use crate::windows::constants::time::SYSTEMTIME_BYTES;
use crate::windows::structures::SystemTime;
use chrono::TimeDelta;

const TIME_ZONE_NAME_UNITS: usize = 32;
const TIME_ZONE_INFORMATION_BYTES: usize =
    3 * size_of::<i32>() + 2 * (TIME_ZONE_NAME_UNITS * size_of::<u16>() + SYSTEMTIME_BYTES);

fn convert_file_time(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = (|| {
        if input == 0 || output == 0 || input == output {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let bytes = read_array::<{ size_of::<u64>() }>(emu, input).ok_or(ERROR_NOACCESS)?;
        // The modeled local zone is UTC with no daylight-saving adjustment.
        if !emu.maps.write_bytes(output, &bytes) {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    log_red!(
        emu,
        "kernel32!{api} input={input:#x} output={output:#x} => {result:?}"
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok() as u64);
    call.cleanup_args(emu, 2);
}

pub fn FileTimeToLocalFileTime(emu: &mut Emu) {
    convert_file_time(emu, "FileTimeToLocalFileTime");
}

pub fn LocalFileTimeToFileTime(emu: &mut Emu) {
    convert_file_time(emu, "LocalFileTimeToFileTime");
}

fn utc_information() -> Vec<u8> {
    let mut data = Vec::with_capacity(TIME_ZONE_INFORMATION_BYTES);
    data.extend_from_slice(&0i32.to_le_bytes());
    for _ in 0..2 {
        let mut name = [0; TIME_ZONE_NAME_UNITS * size_of::<u16>()];
        for (word, code_unit) in name
            .chunks_exact_mut(size_of::<u16>())
            .zip("UTC".encode_utf16())
        {
            word.copy_from_slice(&code_unit.to_le_bytes());
        }
        data.extend_from_slice(&name);
        data.extend_from_slice(&[0; SYSTEMTIME_BYTES]);
        data.extend_from_slice(&0i32.to_le_bytes());
    }
    data
}

pub fn GetTimeZoneInformation(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let output = api_try!(emu, call.arg(emu, 0));
    let data = utc_information();
    let result = if output == 0 {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        TIME_ZONE_ID_INVALID
    } else if !emu.maps.write_bytes(output, &data) {
        emu.set_last_error(ERROR_NOACCESS);
        TIME_ZONE_ID_INVALID
    } else {
        TIME_ZONE_ID_UNKNOWN
    };
    log_red!(
        emu,
        "kernel32!GetTimeZoneInformation output={output:#x} => {result}"
    );
    call.set_return(emu, result as u64);
    call.cleanup_args(emu, 1);
}

pub(crate) fn year(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let year = api_try!(emu, call.arg_u16(emu, 0));
    let dynamic = api_try!(emu, call.arg(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "{library}!GetTimeZoneInformationForYear year={year} dynamic_zone={dynamic:#x} output={output:#x}"
    );
    if dynamic != 0 {
        policy::unsupported(emu, library, "GetTimeZoneInformationForYear", emu.pc());
        return;
    }
    let success = if output == 0 {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        false
    } else if !emu
        .maps
        .validate_write_range(output, TIME_ZONE_INFORMATION_BYTES as u64)
        || !emu.maps.write_bytes(output, &utc_information())
    {
        emu.set_last_error(ERROR_NOACCESS);
        false
    } else {
        true
    };
    call.set_return(emu, u64::from(success));
    call.cleanup_args(emu, 3);
}

pub(crate) fn local_system_time(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let [zone, input, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!SystemTimeToTzSpecificLocalTime zone={zone:#x} universal_time={input:#x} local_time={output:#x}"
    );
    let mut bias = 0;
    if zone != 0 {
        let Some(bytes) = read_array::<TIME_ZONE_INFORMATION_BYTES>(emu, zone) else {
            emu.set_last_error(ERROR_NOACCESS);
            call.set_return(emu, 0);
            call.cleanup_args(emu, 3);
            return;
        };
        const STANDARD_MONTH: usize =
            size_of::<u32>() + TIME_ZONE_NAME_UNITS * size_of::<u16>() + size_of::<u16>();
        const DAYLIGHT_MONTH: usize = STANDARD_MONTH
            + SYSTEMTIME_BYTES
            + size_of::<u32>()
            + TIME_ZONE_NAME_UNITS * size_of::<u16>();
        if bytes[STANDARD_MONTH..STANDARD_MONTH + 2] != [0, 0]
            || bytes[DAYLIGHT_MONTH..DAYLIGHT_MONTH + 2] != [0, 0]
        {
            policy::unsupported(emu, library, "SystemTimeToTzSpecificLocalTime", emu.pc());
            return;
        }
        bias = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
    let converted = (|| {
        if input == 0 || output == 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let utc = SystemTime::from_bytes(read_array(emu, input).ok_or(ERROR_NOACCESS)?);
        let date = utc
            .datetime()
            .and_then(|dt| dt.checked_sub_signed(TimeDelta::minutes(i64::from(bias))))
            .ok_or(ERROR_INVALID_PARAMETER)?;
        let local = SystemTime::from_datetime(date);
        if local.file_time().is_none() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !emu
            .maps
            .validate_write_range(output, SYSTEMTIME_BYTES as u64)
            || !emu.maps.write_bytes(output, &local.to_bytes())
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    if let Err(error) = converted {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(converted.is_ok()));
    call.cleanup_args(emu, 3);
}
