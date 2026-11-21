use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{
    security::{process_user_sid, read_sid},
    trace,
};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::security::SID_TYPE_USER;

fn finish_query(emu: &mut Emu, result: Result<(), u64>, words: usize) {
    let call = WinApi(Stdcall);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, words);
}

fn get_user_name(emu: &mut Emu, name: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [buffer, size] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", name);
    let text: Vec<_> = emu.cfg.user_name.encode_utf16().chain([0]).collect();
    let data = encoding.encode(&text);
    let required = (data.len() / encoding.width()) as u32;
    let result = (|| {
        let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)?;
        if !emu.maps.validate_write_range(size, 4) {
            return Err(ERROR_NOACCESS);
        }
        if capacity < required {
            emu.maps.write_dword(size, required);
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if buffer == 0
            || !emu.maps.validate_write_range(buffer, data.len() as u64)
            || !emu.maps.write_bytes(buffer, &data)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_dword(size, required);
        Ok(())
    })();
    finish_query(emu, result, 2);
}

pub fn GetUserNameA(emu: &mut Emu) {
    get_user_name(emu, "GetUserNameA", Encoding::Ansi);
}
pub fn GetUserNameW(emu: &mut Emu) {
    get_user_name(emu, "GetUserNameW", Encoding::Wide);
}

// Privilege values are the Windows well-known LUIDs for the local system.
const PRIVILEGES: &[(&str, u32)] = &[
    ("SeCreateTokenPrivilege", 2),
    ("SeAssignPrimaryTokenPrivilege", 3),
    ("SeLockMemoryPrivilege", 4),
    ("SeIncreaseQuotaPrivilege", 5),
    ("SeMachineAccountPrivilege", 6),
    ("SeTcbPrivilege", 7),
    ("SeSecurityPrivilege", 8),
    ("SeTakeOwnershipPrivilege", 9),
    ("SeLoadDriverPrivilege", 10),
    ("SeSystemProfilePrivilege", 11),
    ("SeSystemtimePrivilege", 12),
    ("SeProfileSingleProcessPrivilege", 13),
    ("SeIncreaseBasePriorityPrivilege", 14),
    ("SeCreatePagefilePrivilege", 15),
    ("SeCreatePermanentPrivilege", 16),
    ("SeBackupPrivilege", 17),
    ("SeRestorePrivilege", 18),
    ("SeShutdownPrivilege", 19),
    ("SeDebugPrivilege", 20),
    ("SeAuditPrivilege", 21),
    ("SeSystemEnvironmentPrivilege", 22),
    ("SeChangeNotifyPrivilege", 23),
    ("SeRemoteShutdownPrivilege", 24),
    ("SeUndockPrivilege", 25),
    ("SeSyncAgentPrivilege", 26),
    ("SeEnableDelegationPrivilege", 27),
    ("SeManageVolumePrivilege", 28),
    ("SeImpersonatePrivilege", 29),
    ("SeCreateGlobalPrivilege", 30),
    ("SeTrustedCredManAccessPrivilege", 31),
    ("SeRelabelPrivilege", 32),
    ("SeIncreaseWorkingSetPrivilege", 33),
    ("SeTimeZonePrivilege", 34),
    ("SeCreateSymbolicLinkPrivilege", 35),
    ("SeDelegateSessionUserImpersonatePrivilege", 36),
];

fn privilege(emu: &mut Emu, api: &str, encoding: Encoding, by_name: bool) {
    let call = WinApi(Stdcall);
    let [server, input, output] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    let size = if by_name {
        0
    } else {
        api_try!(emu, call.arg(emu, 3))
    };
    match encoding.local_computer(emu, server) {
        Ok(true) => {}
        Ok(false) => {
            policy::unsupported(emu, "advapi32", api, emu.pc());
            return;
        }
        Err(error) => {
            finish_query(emu, Err(error), if by_name { 3 } else { 4 });
            return;
        }
    }
    let result = (|| {
        if by_name {
            let text = String::from_utf16_lossy(&encoding.read(emu, input)?);
            let (_, id) = PRIVILEGES
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(&text))
                .ok_or(ERROR_NO_SUCH_PRIVILEGE)?;
            if output == 0
                || !emu.maps.validate_write_range(output, 8)
                || !emu.maps.write_qword(output, u64::from(*id))
            {
                return Err(ERROR_NOACCESS);
            }
        } else {
            let id = emu.maps.read_qword(input).ok_or(ERROR_NOACCESS)?;
            let (text, _) = PRIVILEGES
                .iter()
                .find(|(_, v)| u64::from(*v) == id)
                .ok_or(ERROR_NO_SUCH_PRIVILEGE)?;
            let bytes = encoding.encode(&text.encode_utf16().chain([0]).collect::<Vec<_>>());
            let len = (bytes.len() / encoding.width()) as u32;
            let cap = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)?;
            if !emu.maps.validate_write_range(size, 4) {
                return Err(ERROR_NOACCESS);
            }
            if cap < len {
                emu.maps.write_dword(size, len);
                return Err(ERROR_INSUFFICIENT_BUFFER);
            }
            if output == 0
                || !emu.maps.validate_write_range(output, bytes.len() as u64)
                || !emu.maps.write_bytes(output, &bytes)
            {
                return Err(ERROR_NOACCESS);
            }
            emu.maps.write_dword(size, len - 1);
        }
        Ok(())
    })();
    finish_query(emu, result, if by_name { 3 } else { 4 });
}
pub fn LookupPrivilegeValueA(emu: &mut Emu) {
    privilege(emu, "LookupPrivilegeValueA", Encoding::Ansi, true);
}
pub fn LookupPrivilegeValueW(emu: &mut Emu) {
    privilege(emu, "LookupPrivilegeValueW", Encoding::Wide, true);
}
pub fn LookupPrivilegeNameA(emu: &mut Emu) {
    privilege(emu, "LookupPrivilegeNameA", Encoding::Ansi, false);
}
pub fn LookupPrivilegeNameW(emu: &mut Emu) {
    privilege(emu, "LookupPrivilegeNameW", Encoding::Wide, false);
}

fn lookup_account(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [
        system,
        account,
        output_sid,
        sid_size,
        domain,
        domain_size,
        sid_use,
    ] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    match encoding.local_computer(emu, system) {
        Ok(true) => {}
        Ok(false) => {
            policy::unsupported(emu, "advapi32", api, emu.pc());
            return;
        }
        Err(error) => {
            finish_query(emu, Err(error), 7);
            return;
        }
    }
    let result = (|| {
        let name = String::from_utf16_lossy(&encoding.read(emu, account)?);
        let (qualifier, user) = name.rsplit_once('\\').unwrap_or(("", &name));
        if !(qualifier.is_empty()
            || qualifier == "."
            || qualifier.eq_ignore_ascii_case(&emu.cfg.host_name))
            || !user.eq_ignore_ascii_case(&emu.cfg.user_name)
        {
            return Err(ERROR_NONE_MAPPED);
        }
        let sid = process_user_sid();
        let domain_name = encoding.encode(
            &emu.cfg
                .host_name
                .encode_utf16()
                .chain([0])
                .collect::<Vec<_>>(),
        );
        let domain_units = (domain_name.len() / encoding.width()) as u32;
        let sid_capacity = emu.maps.read_dword(sid_size).ok_or(ERROR_NOACCESS)?;
        let domain_capacity = emu.maps.read_dword(domain_size).ok_or(ERROR_NOACCESS)?;
        if !emu.maps.validate_write_range(sid_size, 4)
            || !emu.maps.validate_write_range(domain_size, 4)
        {
            return Err(ERROR_NOACCESS);
        }
        if sid_capacity < sid.len() as u32 || domain_capacity < domain_units {
            emu.maps.write_dword(sid_size, sid.len() as u32);
            emu.maps.write_dword(domain_size, domain_units);
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if output_sid == 0
            || domain == 0
            || sid_use == 0
            || !emu.maps.validate_write_range(output_sid, sid.len() as u64)
            || !emu
                .maps
                .validate_write_range(domain, domain_name.len() as u64)
            || !emu.maps.validate_write_range(sid_use, 4)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_bytes(output_sid, &sid);
        emu.maps.write_bytes(domain, &domain_name);
        emu.maps.write_dword(sid_size, sid.len() as u32);
        emu.maps.write_dword(domain_size, domain_units - 1);
        emu.maps.write_dword(sid_use, SID_TYPE_USER);
        Ok(())
    })();
    finish_query(emu, result, 7);
}
pub fn LookupAccountNameA(emu: &mut Emu) {
    lookup_account(emu, "LookupAccountNameA", Encoding::Ansi);
}
pub fn LookupAccountNameW(emu: &mut Emu) {
    lookup_account(emu, "LookupAccountNameW", Encoding::Wide);
}

fn lookup_account_sid(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [
        system,
        sid,
        account,
        account_size,
        domain,
        domain_size,
        sid_use,
    ] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    let local = encoding.local_computer(emu, system);
    if matches!(local, Ok(false)) {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    }
    let result = local.and_then(|_| {
        if read_sid(emu, sid).ok_or(ERROR_INVALID_SID)? != process_user_sid() {
            return Err(ERROR_NONE_MAPPED);
        }
        let name = encoding.terminated(&emu.cfg.user_name.encode_utf16().collect::<Vec<_>>());
        let host = encoding.terminated(&emu.cfg.host_name.encode_utf16().collect::<Vec<_>>());
        let name_units = (name.len() / encoding.width()) as u32;
        let host_units = (host.len() / encoding.width()) as u32;
        let name_capacity = emu.maps.read_dword(account_size).ok_or(ERROR_NOACCESS)?;
        let host_capacity = emu.maps.read_dword(domain_size).ok_or(ERROR_NOACCESS)?;
        if !emu.maps.validate_write_range(account_size, 4)
            || !emu.maps.validate_write_range(domain_size, 4)
        {
            return Err(ERROR_NOACCESS);
        }
        if name_capacity < name_units || host_capacity < host_units {
            emu.maps.write_dword(account_size, name_units);
            emu.maps.write_dword(domain_size, host_units);
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if account == 0
            || domain == 0
            || sid_use == 0
            || !emu.maps.validate_write_range(account, name.len() as u64)
            || !emu.maps.validate_write_range(domain, host.len() as u64)
            || !emu.maps.validate_write_range(sid_use, 4)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_bytes(account, &name);
        emu.maps.write_bytes(domain, &host);
        emu.maps.write_dword(account_size, name_units - 1);
        emu.maps.write_dword(domain_size, host_units - 1);
        emu.maps.write_dword(sid_use, SID_TYPE_USER);
        Ok(())
    });
    finish_query(emu, result, 7);
}
pub fn LookupAccountSidA(emu: &mut Emu) {
    lookup_account_sid(emu, "LookupAccountSidA", Encoding::Ansi);
}
pub fn LookupAccountSidW(emu: &mut Emu) {
    lookup_account_sid(emu, "LookupAccountSidW", Encoding::Wide);
}
