use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_array, read_guest_bytes, read_pointer};
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::libraries::kernel32::file_time::guest_file_time;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crypto::{
    CERT_STORE_PROV_MEMORY, CERT_STORE_PROV_SYSTEM_A, CERT_STORE_PROV_SYSTEM_W,
};
use crate::windows::constants::errors::{
    CRYPT_E_NOT_FOUND, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER,
};

struct Blob {
    address: u64,
    length: usize,
}

impl Blob {
    fn read(emu: &Emu, blob: u64) -> Option<Self> {
        let length = emu.maps.read_dword(blob)? as usize;
        let address = read_pointer(emu, blob.checked_add(pointer_size(emu))?)?;
        (length == 0
            || address != 0 && emu.maps.can_access_range(address, length, Permission::READ))
        .then_some(Self { address, length })
    }

    fn trim_sign(&mut self, emu: &Emu) -> Option<()> {
        while self.length > 1 {
            let end = self.address.checked_add(self.length as u64 - 2)?;
            let [next, high] = read_array(emu, end)?;
            let extension = if (next as i8) < 0 { u8::MAX } else { 0 };
            if extension != high {
                break;
            }
            self.length -= 1;
        }
        Some(())
    }
}

fn equal_blob(emu: &Emu, left: u64, right: u64, signed: bool) -> Option<bool> {
    let mut left = Blob::read(emu, left)?;
    let mut right = Blob::read(emu, right)?;
    if signed {
        left.trim_sign(emu)?;
        right.trim_sign(emu)?;
    }
    if left.length != right.length {
        return Some(false);
    }
    const CHUNK: usize = 4096;
    for pos in (0..left.length).step_by(CHUNK) {
        let size = (left.length - pos).min(CHUNK);
        if read_guest_bytes(emu, left.address + pos as u64, size)?
            != read_guest_bytes(emu, right.address + pos as u64, size)?
        {
            return Some(false);
        }
    }
    Some(true)
}

pub fn CertCompareIntegerBlob(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [left, right] = api_try!(emu, call.args(emu));
    let equal = equal_blob(emu, left, right, true).unwrap_or(false);
    log_red!(
        emu,
        "crypt32!CertCompareIntegerBlob left={left:#x} right={right:#x} equal={equal}"
    );
    call.set_return(emu, u64::from(equal));
    call.cleanup_args(emu, 2);
}

pub fn CertCompareCertificateName(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [encoding, left, right] = api_try!(emu, call.args(emu));
    let equal = equal_blob(emu, left, right, false).unwrap_or(false);
    log_red!(
        emu,
        "crypt32!CertCompareCertificateName encoding={encoding:#x} left={left:#x} right={right:#x} equal={equal}"
    );
    call.set_return(emu, u64::from(equal));
    call.cleanup_args(emu, 3);
}

struct CertInfo {
    serial: u64,
    issuer: u64,
    not_before: u64,
}
impl CertInfo {
    fn at(emu: &Emu, address: u64) -> Option<Self> {
        let pointer = pointer_size(emu);
        let serial = address.checked_add(pointer)?;
        let algorithm = serial.checked_add(2 * pointer)?;
        let issuer = algorithm.checked_add(3 * pointer)?;
        Some(Self {
            serial,
            issuer,
            not_before: issuer.checked_add(2 * pointer)?,
        })
    }

    fn validity(&self, emu: &Emu) -> Option<(u64, u64)> {
        Some((
            u64::from_le_bytes(read_array(emu, self.not_before)?),
            u64::from_le_bytes(read_array(emu, self.not_before.checked_add(8)?)?),
        ))
    }
}

pub fn CertCompareCertificate(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [encoding, left, right] = api_try!(emu, call.args(emu));
    let equal = CertInfo::at(emu, left)
        .zip(CertInfo::at(emu, right))
        .is_some_and(|(a, b)| {
            equal_blob(emu, a.issuer, b.issuer, false) == Some(true)
                && equal_blob(emu, a.serial, b.serial, true) == Some(true)
        });
    log_red!(
        emu,
        "crypt32!CertCompareCertificate encoding={encoding:#x} left={left:#x} right={right:#x} equal={equal}"
    );
    call.set_return(emu, u64::from(equal));
    call.cleanup_args(emu, 3);
}

pub fn CertVerifyTimeValidity(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [time, certificate] = api_try!(emu, call.args(emu));
    let value = if time == 0 {
        Some(guest_file_time(emu))
    } else {
        read_array(emu, time).map(u64::from_le_bytes)
    };
    let period = CertInfo::at(emu, certificate).and_then(|info| info.validity(emu));
    let result = match value.zip(period) {
        Some((now, (start, _))) if now < start => -1_i32,
        Some((now, (_, end))) if now > end => 1,
        Some(_) => 0,
        None => 1,
    };
    log_red!(
        emu,
        "crypt32!CertVerifyTimeValidity time={time:#x} certificate={certificate:#x} result={result}"
    );
    call.set_return(emu, u64::from(result as u32));
    call.cleanup_args(emu, 2);
}

pub fn CertVerifyValidityNesting(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [subject, issuer] = api_try!(emu, call.args(emu));
    let period = |address| CertInfo::at(emu, address).and_then(|info| info.validity(emu));
    let valid = period(subject).zip(period(issuer)).is_some_and(
        |((sub_begin, sub_end), (ca_begin, ca_end))| sub_begin >= ca_begin && sub_end <= ca_end,
    );
    log_red!(
        emu,
        "crypt32!CertVerifyValidityNesting subject={subject:#x} issuer={issuer:#x} valid={valid}"
    );
    call.set_return(emu, u64::from(valid));
    call.cleanup_args(emu, 2);
}

fn is_store(handle: u64) -> bool {
    let uri = helper::handler_get_uri(handle);
    uri == "cert_store" || uri == "CertOpenSystemStore" || uri.starts_with("cert_sys_store://")
}

pub fn CertCloseStore(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [store, flags] = api_try!(emu, call.args(emu));
    let ok = store == 0 || is_store(store);
    if ok && store != 0 {
        helper::handler_close(store);
    } else if !ok {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    log_red!(
        emu,
        "crypt32!CertCloseStore store={store:#x} flags={flags:#x} closed={ok}"
    );
    call.set_return(emu, u64::from(ok));
    call.cleanup_args(emu, 2);
}

pub fn CertEnumCertificatesInStore(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [store, previous] = api_try!(emu, call.args(emu));
    emu.set_last_error(if is_store(store) {
        CRYPT_E_NOT_FOUND
    } else {
        ERROR_INVALID_HANDLE
    });
    log_red!(
        emu,
        "crypt32!CertEnumCertificatesInStore store={store:#x} previous={previous:#x}"
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

fn system_store(emu: &mut Emu, name: u64, wide: bool) -> u64 {
    if let Some(name) = strings::read_text_checked(emu, name, wide).filter(|s| !s.is_empty()) {
        helper::handler_create(&format!("cert_sys_store://{name}"))
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    }
}

pub fn CertOpenStore(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, encoding, crypt_provider, flags, parameter] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "crypt32!CertOpenStore provider={provider:#x} encoding={encoding:#x} crypt_provider={crypt_provider:#x} flags={flags:#x} parameter={parameter:#x}"
    );
    let store = match provider {
        CERT_STORE_PROV_MEMORY if flags == 0 && crypt_provider == 0 => {
            helper::handler_create("cert_store")
        }
        CERT_STORE_PROV_SYSTEM_A | CERT_STORE_PROV_SYSTEM_W if parameter == 0 => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            0
        }
        CERT_STORE_PROV_SYSTEM_A | CERT_STORE_PROV_SYSTEM_W
            if flags == CERT_SYSTEM_STORE_CURRENT_USER
                || flags == CERT_SYSTEM_STORE_LOCAL_MACHINE =>
        {
            system_store(emu, parameter, provider == CERT_STORE_PROV_SYSTEM_W)
        }
        _ => {
            policy::unsupported(emu, "crypt32", "CertOpenStore", emu.pc());
            return;
        }
    };
    call.set_return(emu, store);
    call.cleanup_args(emu, 5);
}
const CERT_SYSTEM_STORE_CURRENT_USER: u64 = 0x0001_0000;
const CERT_SYSTEM_STORE_LOCAL_MACHINE: u64 = 0x0002_0000;

fn open_system_store(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [provider, name] = api_try!(emu, call.args(emu));
    let store = system_store(emu, name, wide);
    log_red!(
        emu,
        "crypt32!{api} provider={provider:#x} system_store={name:#x} store={store:#x}"
    );
    call.set_return(emu, store);
    call.cleanup_args(emu, 2);
}
pub fn CertOpenSystemStore(emu: &mut Emu) {
    open_system_store(emu, "CertOpenSystemStore", false);
}
pub fn CertOpenSystemStoreA(emu: &mut Emu) {
    open_system_store(emu, "CertOpenSystemStoreA", false);
}
pub fn CertOpenSystemStoreW(emu: &mut Emu) {
    open_system_store(emu, "CertOpenSystemStoreW", true);
}

pub fn CertFindCertificateInStore(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [store, encoding, flags, find_type, parameter, previous] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store={store:#x} encoding={encoding:#x} flags={flags:#x} find_type={find_type:#x} parameter={parameter:#x} previous={previous:#x}"
    );
    if previous != 0 {
        policy::unsupported(emu, "crypt32", "CertFindCertificateInStore", emu.pc());
        return;
    }
    emu.set_last_error(if is_store(store) {
        CRYPT_E_NOT_FOUND
    } else {
        ERROR_INVALID_HANDLE
    });
    call.set_return(emu, 0);
    call.cleanup_args(emu, 6);
}
pub fn CertFreeCertificateContext(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let context = api_try!(emu, call.arg(emu, 0));
    if context != 0 {
        policy::unsupported(emu, "crypt32", "CertFreeCertificateContext", emu.pc());
        return;
    }
    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext context={context:#x}"
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
