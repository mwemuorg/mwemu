//! Shared OLE Automation floating-point calls. x86 uses DWORD stack slots;
//! x64 uses positional XMM registers, while output pointers stay in GPR slots.
use std::cmp::Ordering;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};

use crate::emu::Emu;
use crate::winapi::common::memory::Call;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

const VARIANT_TRUE: i16 = -1;
const VARIANT_FALSE: i16 = 0;
const VARCMP_LT: u64 = 0;
const VARCMP_EQ: u64 = 1;
const VARCMP_GT: u64 = 2;
const VARCMP_NULL: u64 = 3;
const CURRENCY_SCALE: f64 = 10_000.0;
const DECIMAL_SCALE_MAX: u8 = 28;
const DECIMAL_NEG: u8 = 0x80;
const DECIMAL_SCALE_OFFSET: u64 = 2;
const DECIMAL_SIGN_OFFSET: u64 = 3;
const DECIMAL_HIGH_OFFSET: u64 = 4;
const DECIMAL_LOW_OFFSET: u64 = 8;
const DOS_DATE_YEAR_SHIFT: u32 = 9;
const DOS_DATE_MONTH_SHIFT: u32 = 5;
const DOS_TIME_HOUR_SHIFT: u32 = 11;
const DOS_TIME_MINUTE_SHIFT: u32 = 5;
const DATE_MIN: f64 = -657_434.0;
const DATE_END: f64 = 2_958_466.0;
const SECONDS_PER_DAY: f64 = 86_400.0;
const DOS_YEAR_START: i32 = 1980;
const DOS_YEAR_END: i32 = 2107;

#[derive(Clone, Copy)]
pub(crate) enum FloatWidth {
    Single,
    Double,
}

impl FloatWidth {
    fn words(self) -> usize {
        match self {
            Self::Single => size_of::<f32>() / size_of::<u32>(),
            Self::Double => size_of::<f64>() / size_of::<u32>(),
        }
    }

    fn read(self, emu: &Emu) -> f64 {
        match self {
            Self::Single => WinApiAbi::f32_arg(emu, 0, 0) as f64,
            Self::Double => WinApiAbi::f64_arg(emu, 0, 0),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Number {
    Bool,
    Signed(u32),
    Unsigned(u32),
    Single,
    Double,
    Date,
    Currency,
}

fn integer(value: f64, bits: u32, signed: bool) -> Result<u64, u64> {
    let rounded = value.round_ties_even();
    let limit = 2.0f64.powi((bits - signed as u32) as i32);
    let minimum = if signed { -limit } else { 0.0 };
    if !rounded.is_finite() || rounded < minimum || rounded >= limit {
        return Err(DISP_E_OVERFLOW);
    }
    Ok(if signed {
        rounded as i64 as u64
    } else {
        rounded as u64
    })
}

fn numeric_bytes(value: f64, number: Number) -> Result<Vec<u8>, u64> {
    match number {
        Number::Bool => Ok((if value == 0.0 {
            VARIANT_FALSE
        } else {
            VARIANT_TRUE
        })
        .to_le_bytes()
        .to_vec()),
        Number::Signed(bits) | Number::Unsigned(bits) => {
            let value = integer(value, bits, matches!(number, Number::Signed(_)))?;
            Ok(value.to_le_bytes()[..(bits / u8::BITS) as usize].to_vec())
        }
        Number::Single => {
            let result = value as f32;
            if value.is_finite() && !result.is_finite() {
                return Err(DISP_E_OVERFLOW);
            }
            Ok(result.to_le_bytes().to_vec())
        }
        Number::Double => Ok(value.to_le_bytes().to_vec()),
        Number::Date => {
            if !(DATE_MIN..DATE_END).contains(&value.trunc()) {
                return Err(DISP_E_OVERFLOW);
            }
            Ok(value.to_le_bytes().to_vec())
        }
        Number::Currency => Ok(integer(value * CURRENCY_SCALE, i64::BITS, true)?
            .to_le_bytes()
            .to_vec()),
    }
}

fn output(emu: &mut Emu, pointer: u64, bytes: &[u8]) -> Result<(), u64> {
    if pointer == 0
        || !emu.maps.validate_write_range(pointer, bytes.len() as u64)
        || !emu.maps.write_bytes(pointer, bytes)
    {
        return Err(E_POINTER);
    }
    Ok(())
}

fn after_float(emu: &Emu, index: usize, x86_word: usize) -> u64 {
    WinApiAbi::arg(
        emu,
        if emu.cfg.arch.is_64bits() {
            index
        } else {
            x86_word
        },
    )
}

pub(crate) fn convert(emu: &mut Emu, api: &str, width: FloatWidth, number: Number) {
    let value = width.read(emu);
    let destination = after_float(emu, 1, width.words());
    let result = numeric_bytes(value, number).and_then(|bytes| output(emu, destination, &bytes));
    log_red!(
        emu,
        "oleaut32!{} {} destination:0x{:x} => {:?}",
        api,
        value,
        destination,
        result
    );
    Call::Api.finish(emu, result.err().unwrap_or(S_OK), width.words() + 1);
}

/// Locale-dependent formatting and full DECIMAL rounding remain unmodelled.
/// Read the correct FP ABI and fail without modifying the caller's output.
pub(crate) fn unsupported_conversion(emu: &mut Emu, api: &str, width: FloatWidth, locale: bool) {
    let value = width.read(emu);
    let pointer_index = if locale { 3 } else { 1 };
    let destination = after_float(emu, pointer_index, width.words() + pointer_index - 1);
    log_red!(
        emu,
        "oleaut32!{} {} destination:0x{:x} not implemented",
        api,
        value,
        destination
    );
    Call::Api.finish(emu, E_NOTIMPL, width.words() + pointer_index);
}

fn compare(left: f64, right: f64) -> u64 {
    match left.partial_cmp(&right) {
        Some(Ordering::Less) => VARCMP_LT,
        Some(Ordering::Equal) => VARCMP_EQ,
        Some(Ordering::Greater) => VARCMP_GT,
        None => VARCMP_NULL,
    }
}

pub(crate) fn compare_single(emu: &mut Emu) {
    let left = WinApiAbi::f32_arg(emu, 0, 0) as f64;
    let right = WinApiAbi::f64_arg(emu, 1, 1);
    let value = compare(left, right);
    log_red!(emu, "oleaut32!VarR4CmpR8 {} {} => {}", left, right, value);
    Call::Api.finish(emu, value, 3);
}

pub(crate) fn compare_currency(emu: &mut Emu) {
    let left = WinApiAbi::arg64(emu, 0) as i64 as f64 / CURRENCY_SCALE;
    let right = WinApiAbi::f64_arg(emu, 1, 2);
    let value = compare(left, right);
    log_red!(emu, "oleaut32!VarCyCmpR8 {} {} => {}", left, right, value);
    Call::Api.finish(emu, value, 4);
}

fn read_decimal(emu: &Emu, address: u64) -> Option<f64> {
    let scale = emu
        .maps
        .read_byte(address.checked_add(DECIMAL_SCALE_OFFSET)?)?;
    let sign = emu
        .maps
        .read_byte(address.checked_add(DECIMAL_SIGN_OFFSET)?)?;
    if address == 0 || scale > DECIMAL_SCALE_MAX || sign & !DECIMAL_NEG != 0 {
        return None;
    }
    let high = emu
        .maps
        .read_dword(address.checked_add(DECIMAL_HIGH_OFFSET)?)?;
    let low = emu
        .maps
        .read_qword(address.checked_add(DECIMAL_LOW_OFFSET)?)?;
    let magnitude = ((high as u128) << u64::BITS) | low as u128;
    let value = magnitude as f64 / 10.0f64.powi(scale as i32);
    Some(if sign == DECIMAL_NEG { -value } else { value })
}

pub(crate) fn compare_decimal(emu: &mut Emu) {
    let address = WinApiAbi::arg(emu, 0);
    let right = WinApiAbi::f64_arg(emu, 1, 1);
    let value =
        read_decimal(emu, address).map_or(HRESULT_E_INVALID_ARG, |left| compare(left, right));
    log_red!(
        emu,
        "oleaut32!VarDecCmpR8 decimal:0x{:x} {} => {}",
        address,
        right,
        value
    );
    Call::Api.finish(emu, value, 3);
}

pub(crate) fn power(emu: &mut Emu) {
    let left = WinApiAbi::f64_arg(emu, 0, 0);
    let right = WinApiAbi::f64_arg(emu, 1, 2);
    let destination = after_float(emu, 2, 4);
    let value = left.powf(right);
    let result = if value.is_finite() {
        output(emu, destination, &value.to_le_bytes())
    } else {
        Err(DISP_E_OVERFLOW)
    };
    log_red!(emu, "oleaut32!VarR8Pow {} {} => {:?}", left, right, result);
    Call::Api.finish(emu, result.err().unwrap_or(S_OK), 5);
}

pub(crate) fn round(emu: &mut Emu) {
    let value = WinApiAbi::f64_arg(emu, 0, 0);
    let decimals = after_float(emu, 1, 2) as i32;
    let destination = after_float(emu, 2, 3);
    let result = if decimals < 0 {
        Err(HRESULT_E_INVALID_ARG)
    } else {
        let scale = 10.0f64.powi(decimals);
        let scaled = value * scale;
        let rounded = if scaled.is_finite() {
            scaled.round_ties_even() / scale
        } else {
            value
        };
        output(emu, destination, &rounded.to_le_bytes())
    };
    log_red!(
        emu,
        "oleaut32!VarR8Round {} decimals:{} => {:?}",
        value,
        decimals,
        result
    );
    Call::Api.finish(emu, result.err().unwrap_or(S_OK), 4);
}

fn variant_date(value: f64) -> Option<NaiveDateTime> {
    if !(DATE_MIN..DATE_END).contains(&value.trunc()) {
        return None;
    }
    let epoch = NaiveDate::from_ymd_opt(1899, 12, 30)?.and_hms_opt(0, 0, 0)?;
    let days = Duration::try_days(value.trunc() as i64)?;
    // Negative DATE values still encode the time as an absolute fraction.
    let seconds = Duration::seconds((value.fract().abs() * SECONDS_PER_DAY).round() as i64);
    let date = epoch
        .checked_add_signed(days)?
        .checked_add_signed(seconds)?;
    (100..=9999).contains(&date.year()).then_some(date)
}

fn system_time_bytes(date: NaiveDateTime) -> Vec<u8> {
    [
        date.year() as u16,
        date.month() as u16,
        date.weekday().num_days_from_sunday() as u16,
        date.day() as u16,
        date.hour() as u16,
        date.minute() as u16,
        date.second() as u16,
        0,
    ]
    .into_iter()
    .flat_map(u16::to_le_bytes)
    .collect()
}

pub(crate) fn to_system_time(emu: &mut Emu, unpacked: bool) {
    let value = WinApiAbi::f64_arg(emu, 0, 0);
    let destination = after_float(
        emu,
        if unpacked { 2 } else { 1 },
        if unpacked { 3 } else { 2 },
    );
    let flags = if unpacked {
        after_float(emu, 1, 2) as u32
    } else {
        0
    };
    let result = if flags != 0 {
        Err(HRESULT_E_INVALID_ARG)
    } else {
        variant_date(value)
            .ok_or(HRESULT_E_INVALID_ARG)
            .and_then(|date| {
                let mut bytes = system_time_bytes(date);
                if unpacked {
                    bytes.extend_from_slice(&(date.ordinal() as u16).to_le_bytes());
                }
                output(emu, destination, &bytes)
            })
    };
    let api = if unpacked {
        "VarUdateFromDate"
    } else {
        "VariantTimeToSystemTime"
    };
    log_red!(
        emu,
        "oleaut32!{} {} destination:0x{:x} => {:?}",
        api,
        value,
        destination,
        result
    );
    let status = if unpacked {
        result.err().unwrap_or(S_OK)
    } else {
        result.is_ok() as u64
    };
    Call::Api.finish(emu, status, if unpacked { 4 } else { 3 });
}

pub(crate) fn to_dos_time(emu: &mut Emu) {
    let value = WinApiAbi::f64_arg(emu, 0, 0);
    let date_pointer = after_float(emu, 1, 2);
    let time_pointer = after_float(emu, 2, 3);
    let result = variant_date(value)
        .filter(|d| (DOS_YEAR_START..=DOS_YEAR_END).contains(&d.year()))
        .ok_or(HRESULT_E_INVALID_ARG)
        .and_then(|date| {
            if date_pointer == 0
                || time_pointer == 0
                || !emu
                    .maps
                    .validate_write_range(date_pointer, size_of::<u16>() as u64)
                || !emu
                    .maps
                    .validate_write_range(time_pointer, size_of::<u16>() as u64)
            {
                return Err(E_POINTER);
            }
            let day = (((date.year() - DOS_YEAR_START) as u16) << DOS_DATE_YEAR_SHIFT)
                | ((date.month() as u16) << DOS_DATE_MONTH_SHIFT)
                | date.day() as u16;
            let time = ((date.hour() as u16) << DOS_TIME_HOUR_SHIFT)
                | ((date.minute() as u16) << DOS_TIME_MINUTE_SHIFT)
                | (date.second() / 2) as u16;
            output(emu, date_pointer, &day.to_le_bytes())?;
            output(emu, time_pointer, &time.to_le_bytes())
        });
    log_red!(
        emu,
        "oleaut32!VariantTimeToDosDateTime {} => {:?}",
        value,
        result
    );
    Call::Api.finish(emu, result.is_ok() as u64, 4);
}
