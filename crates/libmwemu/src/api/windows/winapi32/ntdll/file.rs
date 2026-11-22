use crate::api::dispatch_api;
use crate::emu;
use crate::winapi::common::files;
use crate::winapi::policy;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "NtCreateFile" | "NtOpenFile" => files::native_open_file(emu, api),
        "NtQueryInformationFile" => files::native_query_file(emu),
        "NtReadFile" => files::nt_transfer_file(emu, false),
        "RtlDosPathNameToNtPathName_U" => files::native_path(emu),
        "NtSetInformationFile" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "NtWriteFile" => files::nt_transfer_file(emu, true),
    },
        handled(()) => true,
        fallback => false,
    )
}
