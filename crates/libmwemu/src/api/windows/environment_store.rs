//! Per-emulator process environment. Names use one-to-one Unicode uppercase matching.
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{PAGE_SIZE, align_up};
use crate::winapi::common::memory::{read_array, read_pointer, writable_pointer, write_pointer};
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::{IBM_437_HIGH, WINDOWS_1252_HIGH};
use crate::windows::structures::{RtlUserProcessParameters32, RtlUserProcessParameters64};

pub(crate) const PROFILE: &str = "C:\\Users\\User";
pub(crate) const MAX_ENVIRONMENT_STRING: usize = 32767;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Environment {
    pub(crate) variables: Vec<(Vec<u16>, Vec<u16>)>,
}

impl Environment {
    pub(crate) fn from_config(cfg: &Config) -> Self {
        Self {
            variables: defaults(cfg)
                .into_iter()
                .map(|(name, value)| {
                    (
                        name.encode_utf16().collect(),
                        value.encode_utf16().collect(),
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn block(&self, encoding: Encoding) -> Vec<u8> {
        let mut text = Vec::new();
        for (name, value) in &self.variables {
            text.extend(name);
            text.push(b'=' as u16);
            text.extend(value);
            text.push(0);
        }
        if text.is_empty() {
            text.push(0);
        }
        text.push(0);
        encoding.encode(&text)
    }

    pub(crate) fn position(&self, name: &[u16]) -> Option<usize> {
        self.variables.iter().position(|(key, _)| {
            key.iter()
                .copied()
                .map(uppercase_unit)
                .eq(name.iter().copied().map(uppercase_unit))
        })
    }

    pub(crate) fn get(&self, name: &[u16]) -> Option<&[u16]> {
        self.position(name).map(|i| self.variables[i].1.as_slice())
    }

    pub(crate) fn set(&mut self, name: Vec<u16>, value: Option<Vec<u16>>) {
        match (self.position(&name), value) {
            (Some(i), Some(value)) => self.variables[i].1 = value,
            (None, Some(value)) => self.variables.push((name, value)),
            (Some(i), None) => {
                self.variables.remove(i);
            }
            (None, None) => {}
        }
    }

    pub(crate) fn expand(&self, text: &[u16], limit: usize) -> Option<Vec<u16>> {
        let mut result = Vec::new();
        let mut rest = text;
        while let Some(start) = rest.iter().position(|&unit| unit == b'%' as u16) {
            result.extend_from_slice(&rest[..start]);
            let after = &rest[start + 1..];
            let Some(end) = after.iter().position(|&unit| unit == b'%' as u16) else {
                rest = &rest[start..];
                break;
            };
            let name = &after[..end];
            let value = self
                .get(name)
                .filter(|_| !name.is_empty())
                .unwrap_or(&rest[start..start + end + 2]);
            if result.len().checked_add(value.len())? > limit {
                return None;
            }
            result.extend_from_slice(value);
            rest = &after[end + 1..];
        }
        if result.len().checked_add(rest.len())? > limit {
            return None;
        }
        result.extend_from_slice(rest);
        Some(result)
    }
}

fn uppercase_unit(unit: u16) -> u16 {
    let Some(ch) = char::from_u32(unit as u32) else {
        return unit;
    };
    let mut upper = ch.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(value), None) if value.len_utf16() == 1 => value as u16,
        _ => unit,
    }
}

pub(crate) fn valid_name(name: &[u16]) -> bool {
    !name.is_empty() && !name.contains(&(b'=' as u16))
}

pub(crate) fn process(emu: &mut Emu) -> &mut Environment {
    emu.environment
        .get_or_insert_with(|| Environment::from_config(&emu.cfg))
}

/// Publish the new environment only after the process parameters can refer to the whole block.
pub(crate) fn replace(emu: &mut Emu, environment: Environment) -> Result<(), u64> {
    let (params_name, offset) = if emu.cfg.arch.is_64bits() {
        (
            "RtlUserProcessParameters64",
            RtlUserProcessParameters64::ENVIRONMENT_OFFSET,
        )
    } else {
        (
            "RtlUserProcessParameters32",
            RtlUserProcessParameters32::ENVIRONMENT_OFFSET,
        )
    };
    if let Some(params) = emu.maps.get_map_by_name(params_name)
        && params.size() > offset as usize
    {
        let slot = params.get_base() + offset;
        write_block(emu, slot, &environment.block(Encoding::Wide))?;
    }
    emu.environment = Some(environment);
    Ok(())
}

fn write_block(emu: &mut Emu, slot: u64, bytes: &[u8]) -> Result<(), u64> {
    if !writable_pointer(emu, slot) {
        return Err(ERROR_NOACCESS);
    }
    let old = read_pointer(emu, slot).ok_or(ERROR_NOACCESS)?;
    let previous = emu
        .maps
        .get_map_by_name(&format!("process_environment_{old:x}"))
        .filter(|map| map.get_base() == old)
        .map(|map| map.size());
    if previous.is_some_and(|size| size >= bytes.len()) {
        return emu
            .maps
            .write_bytes(old, bytes)
            .then_some(())
            .ok_or(ERROR_NOACCESS);
    }
    let length = align_up(bytes.len() as u64, PAGE_SIZE).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    let base = emu.maps.alloc(length).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    let name = format!("process_environment_{base:x}");
    let map = emu
        .maps
        .create_map(&name, base, length, Permission::READ_WRITE)
        .map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
    map.write_bytes(base, bytes);
    if !write_pointer(emu, slot, base) {
        emu.maps.dealloc(base);
        return Err(ERROR_NOACCESS);
    }
    if previous.is_some() {
        emu.maps.dealloc(old);
    }
    Ok(())
}

pub(crate) fn initialize(emu: &mut Emu) -> Result<(), u64> {
    let environment = process(emu).clone();
    replace(emu, environment)
}

#[derive(Clone, Copy)]
pub(crate) enum Encoding {
    Ansi,
    Oem,
    Wide,
}

impl Encoding {
    pub(crate) fn local_computer(self, emu: &Emu, address: u64) -> Result<bool, u64> {
        if address == 0 {
            return Ok(true);
        }
        let name = String::from_utf16_lossy(&self.read(emu, address)?);
        Ok(name.is_empty()
            || name
                .trim_start_matches('\\')
                .eq_ignore_ascii_case(&emu.cfg.host_name))
    }

    pub(crate) fn from_wide(wide: bool) -> Self {
        if wide { Self::Wide } else { Self::Ansi }
    }
    pub(crate) fn width(self) -> usize {
        if matches!(self, Self::Wide) { 2 } else { 1 }
    }

    pub(crate) fn decode_byte(self, byte: u8) -> u16 {
        match self {
            Self::Ansi if (0x80..0xa0).contains(&byte) => WINDOWS_1252_HIGH[(byte - 0x80) as usize],
            Self::Oem if byte >= 0x80 => IBM_437_HIGH[(byte - 0x80) as usize],
            _ => byte as u16,
        }
    }

    pub(crate) fn read(self, emu: &Emu, address: u64) -> Result<Vec<u16>, u64> {
        let mut value = Vec::new();
        for index in 0..=MAX_ENVIRONMENT_STRING {
            let at = address
                .checked_add((index * self.width()) as u64)
                .ok_or(ERROR_NOACCESS)?;
            let unit = if matches!(self, Self::Wide) {
                u16::from_le_bytes(read_array(emu, at).ok_or(ERROR_NOACCESS)?)
            } else {
                let byte = emu.maps.read_byte(at).ok_or(ERROR_NOACCESS)?;
                self.decode_byte(byte)
            };
            if unit == 0 {
                return Ok(value);
            }
            value.push(unit);
        }
        Err(ERROR_INVALID_PARAMETER)
    }

    pub(crate) fn encode(self, value: &[u16]) -> Vec<u8> {
        if matches!(self, Self::Wide) {
            return value.iter().flat_map(|unit| unit.to_le_bytes()).collect();
        }
        char::decode_utf16(value.iter().copied())
            .map(|ch| {
                let scalar = ch.map(u32::from).unwrap_or(u32::MAX);
                if scalar < 0x80 {
                    scalar as u8
                } else if matches!(self, Self::Oem) {
                    IBM_437_HIGH
                        .iter()
                        .position(|&unit| unit as u32 == scalar)
                        .map_or(b'?', |i| i as u8 + 0x80)
                } else if (0xa0..=0xff).contains(&scalar) {
                    scalar as u8
                } else {
                    WINDOWS_1252_HIGH
                        .iter()
                        .position(|&unit| unit as u32 == scalar)
                        .map_or(b'?', |i| i as u8 + 0x80)
                }
            })
            .collect()
    }

    pub(crate) fn terminated(self, value: &[u16]) -> Vec<u8> {
        let mut bytes = self.encode(value);
        bytes.resize(bytes.len() + self.width(), 0);
        bytes
    }
}

fn defaults(cfg: &Config) -> Vec<(String, String)> {
    let local = format!("{}\\AppData\\Local", PROFILE);
    let temp = format!("{}\\Temp", local);
    [
        ("ALLUSERSPROFILE", "C:\\ProgramData".to_string()),
        ("APPDATA", format!("{}\\AppData\\Roaming", PROFILE)),
        (
            "CommonProgramFiles",
            "C:\\Program Files\\Common Files".to_string(),
        ),
        ("COMPUTERNAME", cfg.host_name.to_ascii_uppercase()),
        ("ComSpec", "C:\\Windows\\system32\\cmd.exe".to_string()),
        ("HOMEDRIVE", "C:".to_string()),
        ("HOMEPATH", "\\Users\\User".to_string()),
        ("LOCALAPPDATA", local),
        ("NUMBER_OF_PROCESSORS", "4".to_string()),
        ("OS", "Windows_NT".to_string()),
        (
            "Path",
            "C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem".to_string(),
        ),
        (
            "PATHEXT",
            ".COM;.EXE;.BAT;.CMD;.VBS;.JS;.WSF;.MSC".to_string(),
        ),
        (
            "PROCESSOR_ARCHITECTURE",
            if cfg.arch.is_64bits() { "AMD64" } else { "x86" }.to_string(),
        ),
        ("ProgramData", "C:\\ProgramData".to_string()),
        ("ProgramFiles", "C:\\Program Files".to_string()),
        ("ProgramFiles(x86)", "C:\\Program Files (x86)".to_string()),
        ("PUBLIC", "C:\\Users\\Public".to_string()),
        ("SystemDrive", "C:".to_string()),
        ("SystemRoot", "C:\\Windows".to_string()),
        ("TEMP", temp.clone()),
        ("TMP", temp),
        ("USERDOMAIN", cfg.host_name.to_ascii_uppercase()),
        ("USERNAME", cfg.user_name.clone()),
        ("USERPROFILE", PROFILE.to_string()),
        ("windir", "C:\\Windows".to_string()),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
