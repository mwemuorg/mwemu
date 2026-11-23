use super::data_in;
use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::winapi::registry::create_path;
use crate::winapi::registry_store::{self as store, REG_SZ, Value};
use crate::windows::constants::errors::*;

pub(super) fn set_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let slots = 6;
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name = match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
        Ok(value) => value.unwrap_or_default(),
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let kind = api_try!(emu, call.arg_u32(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let size = api_try!(emu, call.arg_u32(emu, 5)) as usize;

    let path = match key_path(emu, hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, slots, api, &name, err),
    };
    let data = if size == 0 {
        Vec::new()
    } else {
        match emu.maps.try_read_bytes(data_ptr, size) {
            Some(bytes) => bytes.to_vec(),
            None => return finish(emu, slots, api, &name, ERROR_INVALID_PARAMETER),
        }
    };
    let value = Value {
        kind,
        data: data_in(kind, &data, wide),
    };
    emu.registry.set_value(&path, &name, value);
    let what = format!("{} `{}` type {} ({} bytes)", path, name, kind, size);
    finish(emu, slots, api, &what, ERROR_SUCCESS);
}

fn set_subkey_value(emu: &mut emu::Emu, api: &str, wide: bool, legacy: bool) {
    let call = WinApi(Stdcall);
    let [key, subkey] = api_try!(emu, call.args(emu));
    let (name, kind, data, size) = if legacy {
        (
            0,
            api_try!(emu, call.arg_u32(emu, 2)),
            api_try!(emu, call.arg(emu, 3)),
            0,
        )
    } else {
        (
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg_u32(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
            api_try!(emu, call.arg_u32(emu, 5)),
        )
    };
    let result = (|| {
        let root = key_path(emu, key)?;
        let subkey = read_name(emu, subkey, wide)?.unwrap_or_default();
        let path = store::join(&root, &subkey);
        let name = read_name(emu, name, wide)?.unwrap_or_default();
        let bytes = if legacy {
            if kind != REG_SZ || data == 0 {
                return Err(ERROR_INVALID_PARAMETER);
            }
            let mut text = Encoding::from_wide(wide).read(emu, data)?;
            text.push(0);
            Encoding::Wide.encode(&text)
        } else if size == 0 {
            Vec::new()
        } else {
            let bytes = emu
                .maps
                .try_read_bytes(data, size as usize)
                .ok_or(ERROR_NOACCESS)?;
            data_in(kind, &bytes, wide)
        };
        if create_path(emu, &path).is_none()
            || !emu
                .registry
                .set_value(&path, &name, Value { kind, data: bytes })
        {
            return Err(ERROR_ACCESS_DENIED);
        }
        Ok(())
    })();
    finish(
        emu,
        if legacy { 5 } else { 6 },
        api,
        "",
        result.err().unwrap_or(ERROR_SUCCESS),
    );
}
pub fn RegSetValueA(emu: &mut emu::Emu) {
    set_subkey_value(emu, "RegSetValueA", false, true);
}
pub fn RegSetValueW(emu: &mut emu::Emu) {
    set_subkey_value(emu, "RegSetValueW", true, true);
}
pub fn RegSetKeyValueA(emu: &mut emu::Emu) {
    set_subkey_value(emu, "RegSetKeyValueA", false, false);
}
pub fn RegSetKeyValueW(emu: &mut emu::Emu) {
    set_subkey_value(emu, "RegSetKeyValueW", true, false);
}
