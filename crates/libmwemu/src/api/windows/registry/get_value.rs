use super::{data_out, finish, is_string, key_path, read_name, type_flag};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::environment_store::{self, Encoding};
use crate::winapi::policy::api_try;
use crate::winapi::registry_store::{self as store, Value};
use crate::windows::constants::errors::*;
use crate::windows::constants::security::*;

struct ValueOutput {
    type_ptr: u64,
    data_ptr: u64,
    size_ptr: u64,
    capacity: usize,
}

impl ValueOutput {
    fn set_type(&self, emu: &mut Emu, kind: u32) -> Result<(), u64> {
        if self.type_ptr != 0 && !emu.maps.write_dword(self.type_ptr, kind) {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    }

    fn write(&self, emu: &mut Emu, value: &Value, wide: bool) -> Result<(), u64> {
        self.set_type(emu, value.kind)?;
        let mut bytes = data_out(value, wide);
        let width = Encoding::from_wide(wide).width();
        let mut reserve = 0;
        if is_string(value.kind) {
            let count = if value.kind == store::REG_MULTI_SZ {
                2
            } else {
                1
            };
            let present = bytes
                .rchunks_exact(width)
                .take(count)
                .take_while(|word| word.iter().all(|&byte| byte == 0))
                .count();
            bytes.resize(bytes.len() + (count - present) * width, 0);
            // A size query allows room for terminators even if the stored data already had them.
            reserve = present * width;
        }
        let short = self.data_ptr != 0 && bytes.len() > self.capacity;
        let length = if self.data_ptr == 0 || short {
            bytes.len() + reserve
        } else {
            bytes.len()
        };
        if self.size_ptr != 0
            && !emu.maps.write_dword(
                self.size_ptr,
                u32::try_from(length).map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?,
            )
        {
            return Err(ERROR_NOACCESS);
        }
        if short {
            return Err(ERROR_MORE_DATA);
        }
        if self.data_ptr != 0 && !emu.maps.write_bytes(self.data_ptr, &bytes) {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    }
}

fn query(
    emu: &mut Emu,
    hkey: u64,
    names: [u64; 2],
    flags: u32,
    output: &ValueOutput,
    wide: bool,
) -> Result<(), u64> {
    let allowed = flags & RRF_RT_ANY;
    if allowed == 0
        || flags & RRF_WOW64_MASK == RRF_WOW64_MASK
        || (output.data_ptr != 0 && output.size_ptr == 0)
    {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let base = key_path(emu, hkey)?;
    let sub = read_name(emu, names[0], wide)?.unwrap_or_default();
    let name = read_name(emu, names[1], wide)?.unwrap_or_default();
    let Some(mut value) = emu.registry.get_value(&store::join(&base, &sub), &name) else {
        output.set_type(emu, store::REG_NONE)?;
        return Err(ERROR_FILE_NOT_FOUND);
    };
    if value.kind == store::REG_EXPAND_SZ && flags & RRF_NOEXPAND == 0 {
        let words: Vec<_> = value
            .data
            .as_chunks::<2>()
            .0
            .iter()
            .copied()
            .map(u16::from_le_bytes)
            .take_while(|&c| c != 0)
            .collect();
        let limit = emu.maps.max_alloc_size as usize / size_of::<u16>();
        let expanded = environment_store::process(emu)
            .expand(&words, limit)
            .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        value = Value {
            kind: store::REG_SZ,
            data: Encoding::Wide.terminated(&expanded),
        };
    }
    let transfer = output.write(emu, &value, wide);
    if matches!(transfer, Err(ERROR_NOACCESS)) {
        return transfer;
    }
    if allowed != RRF_RT_ANY && allowed & type_flag(value.kind) == 0 {
        return Err(ERROR_UNSUPPORTED_TYPE);
    }
    if value.kind == store::REG_BINARY
        && ((allowed == RRF_RT_DWORD && value.data.len() != size_of::<u32>())
            || (allowed == RRF_RT_QWORD && value.data.len() != size_of::<u64>()))
    {
        return Err(ERROR_DATATYPE_MISMATCH);
    }
    transfer
}

pub(super) fn get_value(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [hkey, sub, name, flags, type_ptr, data_ptr, size_ptr] = api_try!(emu, call.args(emu));
    let flags = flags as u32;
    let capacity = if size_ptr == 0 || data_ptr == 0 {
        0
    } else {
        let Some(size) = emu.maps.read_dword(size_ptr) else {
            return finish(emu, 7, api, "", ERROR_NOACCESS);
        };
        size as usize
    };
    let output = ValueOutput {
        type_ptr,
        data_ptr,
        size_ptr,
        capacity,
    };
    let status = match query(emu, hkey, [sub, name], flags, &output, wide) {
        Ok(()) => ERROR_SUCCESS,
        Err(_)
            if flags & RRF_ZEROONFAILURE != 0
                && data_ptr != 0
                && !emu.maps.fill_range(data_ptr, 0, capacity) =>
        {
            ERROR_NOACCESS
        }
        Err(error) => error,
    };
    finish(
        emu,
        7,
        api,
        &format!("hkey:{hkey:#x} flags:{flags:#x}"),
        status,
    );
}
