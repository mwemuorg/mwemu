use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const FIRST_STRING_ATOM: u16 = 0xc000;
const MAX_ATOM_NAME: usize = 255;

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct AtomTables {
    local: BTreeMap<u16, Atom>,
    global: BTreeMap<u16, Atom>,
    user: BTreeMap<Vec<u16>, u16>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Atom {
    name: Vec<u16>,
    references: u32,
}

impl AtomTables {
    pub(crate) fn register_user_name(&mut self, name: &[u16]) -> Result<u16, u64> {
        if name.is_empty() {
            return Ok(0);
        }
        if name.len() > MAX_ATOM_NAME {
            return Err(ERROR_MORE_DATA);
        }
        if let Some(id) = integer_atom_name(name)? {
            return Ok(id);
        }
        let key = folded(name);
        if let Some(&id) = self.user.get(&key) {
            return Ok(id);
        }
        let id = u16::try_from(FIRST_STRING_ATOM as usize + self.user.len())
            .map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
        self.user.insert(key, id);
        Ok(id)
    }

    fn table(&mut self, global: bool) -> &mut BTreeMap<u16, Atom> {
        if global {
            &mut self.global
        } else {
            &mut self.local
        }
    }
}

fn folded(name: &[u16]) -> Vec<u16> {
    // Preserve unpaired UTF-16 units. Compare one code unit at a time.
    name.iter()
        .map(|&unit| {
            char::from_u32(u32::from(unit))
                .and_then(|ch| {
                    let mut upper = ch.to_uppercase();
                    let first = upper.next()?;
                    (upper.next().is_none()).then_some(first)
                })
                .and_then(|ch| u16::try_from(u32::from(ch)).ok())
                .unwrap_or(unit)
        })
        .collect()
}

fn integer_atom(value: u16) -> Result<u16, u64> {
    (1..FIRST_STRING_ATOM)
        .contains(&value)
        .then_some(value)
        .ok_or(ERROR_INVALID_PARAMETER)
}

fn integer_atom_name(name: &[u16]) -> Result<Option<u16>, u64> {
    let Some(digits) = name.strip_prefix(&[b'#' as u16]) else {
        return Ok(None);
    };
    if digits.is_empty()
        || !digits
            .iter()
            .all(|d| (b'0' as u16..=b'9' as u16).contains(d))
    {
        return Ok(None);
    }
    let id = digits
        .iter()
        .try_fold(0u16, |id, d| {
            id.checked_mul(10)?.checked_add(d - b'0' as u16)
        })
        .ok_or(ERROR_INVALID_PARAMETER)?;
    integer_atom(id).map(Some)
}

fn lookup(emu: &mut Emu, global: bool, add: bool, ptr: u64, codec: Encoding) -> Result<u16, u64> {
    if let Ok(value) = u16::try_from(ptr) {
        return integer_atom(value);
    }
    let name = codec.read(emu, ptr)?;
    if name.is_empty() || name.len() > MAX_ATOM_NAME {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if let Some(id) = integer_atom_name(&name)? {
        return Ok(id);
    }
    let key = folded(&name);
    let table = emu.atom_tables.table(global);
    if let Some((&id, entry)) = table
        .iter_mut()
        .find(|(_, entry)| folded(&entry.name) == key)
    {
        if add {
            entry.references = entry
                .references
                .checked_add(1)
                .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        }
        return Ok(id);
    }
    if !add {
        return Err(ERROR_FILE_NOT_FOUND);
    }
    let id = (FIRST_STRING_ATOM..=u16::MAX)
        .find(|id| !table.contains_key(id))
        .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    table.insert(
        id,
        Atom {
            name,
            references: 1,
        },
    );
    Ok(id)
}

fn delete(emu: &mut Emu, global: bool, id: u16) -> Result<u16, u64> {
    if id < FIRST_STRING_ATOM {
        return integer_atom(id).map(|_| 0);
    }
    let table = emu.atom_tables.table(global);
    let atom = table.get_mut(&id).ok_or(ERROR_INVALID_HANDLE)?;
    atom.references -= 1;
    if atom.references == 0 {
        table.remove(&id);
    }
    Ok(0)
}

fn get_name(
    emu: &mut Emu,
    global: bool,
    id: u16,
    dest: u64,
    capacity: i32,
    codec: Encoding,
) -> Result<u16, u64> {
    if dest == 0 || capacity <= 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let name = if id < FIRST_STRING_ATOM {
        format!("#{}", integer_atom(id)?).encode_utf16().collect()
    } else {
        emu.atom_tables
            .table(global)
            .get(&id)
            .ok_or(ERROR_INVALID_HANDLE)?
            .name
            .clone()
    };
    let mut bytes = codec.encode(&name);
    let width = codec.width();
    if capacity == 1 && !bytes.is_empty() {
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    let units = (bytes.len() / width).min(capacity as usize - 1);
    bytes.truncate(units * width);
    bytes.resize((units + 1) * width, 0);
    if !emu.maps.validate_write_range(dest, bytes.len() as u64)
        || !emu.maps.write_bytes(dest, &bytes)
    {
        return Err(ERROR_NOACCESS);
    }
    Ok(units as u16)
}

pub(crate) fn atom(api: &str, library: &str, emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let global = api.starts_with("Global");
    let codec = Encoding::from_wide(api.ends_with('W'));
    let (result, slots, failure) = match api.trim_start_matches("Global") {
        "AddAtomA" | "AddAtomW" | "FindAtomA" | "FindAtomW" | "AddAtomExA" | "AddAtomExW" => {
            let extended = api.contains("AtomEx");
            let ptr = api_try!(emu, call.arg(emu, 0));
            if extended && api_try!(emu, call.arg_u32(emu, 1)) != 0 {
                policy::unsupported(emu, library, api, emu.pc());
                return;
            }
            (
                lookup(emu, global, api.contains("Add"), ptr, codec),
                1 + usize::from(extended),
                0,
            )
        }
        "DeleteAtom" => {
            let id = api_try!(emu, call.arg_u16(emu, 0));
            (delete(emu, global, id), 1, if global { 0 } else { id })
        }
        "GetAtomNameA" | "GetAtomNameW" => {
            let id = api_try!(emu, call.arg_u16(emu, 0));
            let dest = api_try!(emu, call.arg(emu, 1));
            let capacity = api_try!(emu, call.arg_i32(emu, 2));
            (get_name(emu, global, id, dest, capacity, codec), 3, 0)
        }
        _ => return,
    };
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        failure
    });
    log_red!(emu, "{library}!{api} => {value:#x}");
    call.set_return(emu, u64::from(value));
    call.cleanup_args(emu, slots);
}
