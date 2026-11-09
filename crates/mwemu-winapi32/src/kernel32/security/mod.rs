use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "CryptCreateHash" => kernel32::CryptCreateHash(emu),
        "OpenProcessToken" => kernel32::OpenProcessToken(emu),
        "AddResourceAttributeAce" => kernel32::AddResourceAttributeAce(emu),
        "BasepFreeActivationTokenInfo" => kernel32::BasepFreeActivationTokenInfo(emu),
        "BasepGetPackageActivationTokenForFilePath" => {
            kernel32::BasepGetPackageActivationTokenForFilePath(emu)
        }
        "BasepGetPackageActivationTokenForFilePath2" => {
            kernel32::BasepGetPackageActivationTokenForFilePath2(emu)
        }
        "BasepGetPackageActivationTokenForSxS" => {
            kernel32::BasepGetPackageActivationTokenForSxS(emu)
        }
        "BasepGetPackageActivationTokenForSxS2" => {
            kernel32::BasepGetPackageActivationTokenForSxS2(emu)
        }
        "BasepGetPackageActivationTokenForSxS3" => {
            kernel32::BasepGetPackageActivationTokenForSxS3(emu)
        }
        "CheckTokenCapability" => kernel32::CheckTokenCapability(emu),
        "CheckTokenMembershipEx" => kernel32::CheckTokenMembershipEx(emu),
        "GetAppContainerAce" => kernel32::GetAppContainerAce(emu),
        "OpenThreadToken" => kernel32::OpenThreadToken(emu),
        "SetThreadToken" => kernel32::SetThreadToken(emu),
        _ => return false,
    }
    true
}
