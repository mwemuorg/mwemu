use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store::{self, HKEY_CLASSES_ROOT, REG_SZ};

pub fn PathIsContentTypeA(emu: &mut Emu) {
    invoke(emu, false, "PathIsContentTypeA");
}

pub fn PathIsContentTypeW(emu: &mut Emu) {
    invoke(emu, true, "PathIsContentTypeW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let path = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let expected = strings::read_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    log_red!(emu, "shlwapi!{api} path={path:?} content_type={expected:?}");
    let matched = path
        .rsplit_once('.')
        .filter(|(_, extension)| !extension.contains(['\\', '/']))
        .and_then(|(_, extension)| {
            let root = registry_store::predefined_path(HKEY_CLASSES_ROOT)?;
            emu.registry.get_value(
                &registry_store::join(root, &format!(".{extension}")),
                "Content Type",
            )
        })
        .filter(|value| value.kind == REG_SZ)
        .is_some_and(|value| {
            #[allow(unknown_lints, clippy::chunks_exact_to_as_chunks)]
            let words: Vec<u16> = value
                .data
                .chunks_exact(size_of::<u16>())
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .take_while(|c| *c != 0)
                .collect();
            String::from_utf16_lossy(&words).eq_ignore_ascii_case(&expected)
        });
    call.set_return(emu, matched as u64);
    call.cleanup_args(emu, 2);
}
