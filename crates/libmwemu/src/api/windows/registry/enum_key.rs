use super::finish;
use super::key_path;
use super::write_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn enum_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let slots = 8;
    let hkey = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1)) as usize;
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let cch_ptr = api_try!(emu, call.arg(emu, 3));
    let class_cch_ptr = api_try!(emu, call.arg(emu, 6));
    let time_ptr = api_try!(emu, call.arg(emu, 7));

    let path = match key_path(emu, hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, slots, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some(name) = emu.registry.subkeys(&path).into_iter().nth(index) else {
        return finish(emu, slots, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        // keys carry no class
        if class_cch_ptr != 0 {
            emu.maps.write_dword(class_cch_ptr, 0);
        }
        if time_ptr != 0 {
            emu.maps.write_qword(time_ptr, 0);
        }
    }
    finish(emu, slots, api, &format!("{} = {}", what, name), err);
}

fn enum_legacy(emu: &mut emu::Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [key, index, output, capacity] = api_try!(emu, call.args(emu));
    let result = (|| {
        let path = key_path(emu, key)?;
        let item = emu
            .registry
            .subkeys(&path)
            .into_iter()
            .nth(index as u32 as usize)
            .ok_or(ERROR_NO_MORE_ITEMS)?;
        let data = encoding.terminated(&item.encode_utf16().collect::<Vec<_>>());
        if data.len() / encoding.width() > capacity as u32 as usize {
            return Err(ERROR_MORE_DATA);
        }
        if output == 0
            || !emu.maps.validate_write_range(output, data.len() as u64)
            || !emu.maps.write_bytes(output, &data)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    finish(emu, 4, api, "", result.err().unwrap_or(ERROR_SUCCESS));
}
pub fn RegEnumKeyA(emu: &mut emu::Emu) {
    enum_legacy(emu, "RegEnumKeyA", Encoding::Ansi);
}
pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    enum_legacy(emu, "RegEnumKeyW", Encoding::Wide);
}
