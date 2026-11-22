mod dispatch;
pub(crate) use dispatch::dispatch;
pub mod exports;

pub use crate::winapi::libraries::advapi32::{
    CryptAcquireContextA as CryptAcquireContext, CryptCreateHash, CryptDecrypt, CryptDeriveKey,
    CryptDestroyHash, CryptDestroyKey, CryptEncrypt, CryptGenKey, CryptGetHashParam,
    CryptGetKeyParam, CryptHashData, CryptReleaseContext,
};
pub use certificates::{
    CertCloseStore, CertCompareCertificate, CertCompareCertificateName, CertCompareIntegerBlob,
    CertEnumCertificatesInStore, CertFindCertificateInStore, CertFreeCertificateContext,
    CertOpenStore, CertOpenSystemStore, CertOpenSystemStoreA, CertOpenSystemStoreW,
    CertVerifyTimeValidity, CertVerifyValidityNesting,
};
mod data_protection;
pub use crate::winapi::common::crypt32::CryptHashCertificate;
pub use data_protection::{CryptProtectData, CryptUnprotectData};
mod certificates;
use crate::winapi::common::fallback::unsupported_exports;
pub use crate::winapi::libraries::unsupported::crypt32::{
    CertAddRefServerOcspResponse, CertAddRefServerOcspResponseContext, CertAlgIdToOID,
    CertCreateCRLContext, CertCreateCTLContext, CertDeleteCRLFromStore, CertDeleteCTLFromStore,
    CertDeleteCertificateFromStore, CertEnumCRLsInStore, CertEnumCTLsInStore,
    CertEnumPhysicalStore, CertEnumSubjectInSortedCTL, CertEnumSystemStore,
    CertEnumSystemStoreLocation, CertFreeCertificateChain, CertGetCRLFromStore, CertOIDToAlgId,
    CertSelectCertificate, CertSrvBackupEnd, CertSrvBackupTruncateLogs, CertSrvRestoreEnd,
    CryptUIDlgCertMgr, PFXIsPFXBlob,
};

mod crypt_gen_random;
pub use crypt_gen_random::CryptGenRandom;

mod crypt_mem_alloc;
pub use crypt_mem_alloc::{CryptMemAlloc, CryptMemFree, CryptMemRealloc};

unsupported_exports!(
    "crypt32",
    CryptSIPRemoveProvider,
    CryptSIPAddProvider,
    CryptUIDlgSelectCertificate,
);
