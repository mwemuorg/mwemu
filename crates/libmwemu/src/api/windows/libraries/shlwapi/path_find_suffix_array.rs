use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{pointer_size, read_pointer};
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;

pub fn PathFindSuffixArrayA(emu: &mut Emu) {
    invoke(emu, false, "PathFindSuffixArrayA");
}

pub fn PathFindSuffixArrayW(emu: &mut Emu) {
    invoke(emu, true, "PathFindSuffixArrayW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let path = api_try!(emu, call.arg(emu, 0));
    let array = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_i32(emu, 2));
    let codec = Encoding::from_wide(wide);
    let found = find_suffix(emu, codec, path, array, count);
    log_red!(
        emu,
        "shlwapi!{api} path={path} count={count} result={found:?}",
        path = input_string(emu, path, codec)
    );
    call.set_return(emu, found.unwrap_or(0));
    call.cleanup_args(emu, 3);
}

fn find_suffix(emu: &Emu, codec: Encoding, path: u64, array: u64, count: i32) -> Option<u64> {
    if count <= 0 || array == 0 {
        return None;
    }
    let text = codec.read(emu, path).ok()?;
    let stride = pointer_size(emu);
    for index in 0..count as u64 {
        let address = read_pointer(emu, array.checked_add(index * stride)?)?;
        if address == 0 {
            continue;
        }
        let suffix = codec.read(emu, address).ok()?;
        if let Some(prefix) = text.strip_suffix(suffix.as_slice()) {
            return path.checked_add((prefix.len() * codec.width()) as u64);
        }
    }
    None
}
