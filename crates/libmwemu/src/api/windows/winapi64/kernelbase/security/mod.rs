use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetTokenInformation" => kernelbase::GetTokenInformation(emu),
        "GetAce" => kernelbase::GetAce(emu),
        "GetAclInformation" => kernelbase::GetAclInformation(emu),
        "GetFileSecurityW" => kernelbase::GetFileSecurityW(emu),
        "InitializeAcl" => kernelbase::InitializeAcl(emu),
        "MapGenericMask" => kernelbase::MapGenericMask(emu),
        "SetFileSecurityW" => kernelbase::SetFileSecurityW(emu),
        "OpenProcessToken" => kernelbase::OpenProcessToken(emu),
        "OpenThreadToken" => kernelbase::OpenThreadToken(emu),
        "SetThreadToken" => kernelbase::SetThreadToken(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
