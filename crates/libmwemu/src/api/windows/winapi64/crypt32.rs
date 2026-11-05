use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PkiInitializeCriticalSection" => PkiInitializeCriticalSection(emu),
        "CryptStringToBinaryA" => CryptStringToBinaryA(emu),
        "CryptDecodeObjectEx" => CryptDecodeObjectEx(emu),
        "CertOpenStore" => CertOpenStore(emu),
        "CertOpenSystemStoreA" => CertOpenSystemStoreA(emu),
        "CertOpenSystemStoreW" => CertOpenSystemStoreW(emu),
        "CertCloseStore" => CertCloseStore(emu),
        "CertFindCertificateInStore" => CertFindCertificateInStore(emu),
        "CertFreeCertificateContext" => CertFreeCertificateContext(emu),
        "CertAddCRLContextToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddCRLContextToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCRLLinkToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddCRLLinkToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCTLContextToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddCTLContextToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCTLLinkToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddCTLLinkToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCertificateContextToStore" => {
            let store = emu.regs().rcx;
            let cert_ctx = emu.regs().rdx;
            let add_disp = emu.regs().r8;
            let out_ptr = emu.regs().r9;
            let hndl = helper::handler_create("cert_context");
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, hndl);
            }
            log_red!(
                emu,
                "crypt32!CertAddCertificateContextToStore store:0x{:x} disp:{} => 0x{:x}",
                store,
                add_disp,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCertificateLinkToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddCertificateLinkToStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddEncodedCRLToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddEncodedCRLToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddEncodedCTLToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddEncodedCTLToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddEncodedCertificateToStore" => {
            let store = emu.regs().rcx;
            let encoding = emu.regs().rdx;
            let cert_data = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let add_disp = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let hndl = helper::handler_create("cert_encoded");
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, hndl);
            }
            log_red!(
                emu,
                "crypt32!CertAddEncodedCertificateToStore store:0x{:x} len:{}",
                store,
                cert_len
            );
            emu.regs_mut().rax = 1;
        }
        "CertAddEnhancedKeyUsageIdentifier" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddEnhancedKeyUsageIdentifier ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddRefServerOcspResponse" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddRefServerOcspResponse ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddRefServerOcspResponseContext" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddRefServerOcspResponseContext ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddSerializedElementToStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddSerializedElementToStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddStoreToCollection" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAddStoreToCollection ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAlgIdToOID" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAlgIdToOID ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertCloseServerOcspResponse" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertCloseServerOcspResponse ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertCompareCertificate" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertCompareCertificate ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertCompareCertificateName" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertCompareCertificateName ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertCompareIntegerBlob" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertCompareIntegerBlob ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertComparePublicKeyInfo" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertComparePublicKeyInfo ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertControlStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertControlStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertCreateCRLContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertCreateCRLContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateCRLContext");
        }
        "CertCreateCTLContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertCreateCTLContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateCTLContext");
        }
        "CertCreateCTLEntryFromCertificateContextProperties" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertCreateCTLEntryFromCertificateContextProperties store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle =
                helper::handler_create("certcreatectlentryfromcertificatecontextproperties");
            emu.regs_mut().rax = handle;
        }
        "CertCreateCertificateChainEngine" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertCreateCertificateChainEngine store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certcreatecertificatechainengine");
            emu.regs_mut().rax = handle;
        }
        "CertCreateCertificateContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertCreateCertificateContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateCertificateContext");
        }
        "CertCreateContext" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertCreateContext store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certcreatecontext");
            emu.regs_mut().rax = handle;
        }
        "CertCreateSelfSignCertificate" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertCreateSelfSignCertificate store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certcreateselfsigncertificate");
            emu.regs_mut().rax = handle;
        }
        "CertDeleteCRLFromStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDeleteCRLFromStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertDeleteCTLFromStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDeleteCTLFromStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertDeleteCertificateFromStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertDeleteCertificateFromStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertDuplicateCRLContext" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDuplicateCRLContext ctx:0x{:x}", context);
            emu.set_last_error(0);
            let handle = helper::handler_create("certduplicatecrlcontext");
            emu.regs_mut().rax = handle;
        }
        "CertDuplicateCTLContext" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDuplicateCTLContext ctx:0x{:x}", context);
            emu.set_last_error(0);
            let handle = helper::handler_create("certduplicatectlcontext");
            emu.regs_mut().rax = handle;
        }
        "CertDuplicateCertificateChain" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertDuplicateCertificateChain ctx:0x{:x}",
                context
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certduplicatecertificatechain");
            emu.regs_mut().rax = handle;
        }
        "CertDuplicateCertificateContext" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertDuplicateCertificateContext ctx:0x{:x}",
                context
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certduplicatecertificatecontext");
            emu.regs_mut().rax = handle;
        }
        "CertDuplicateStore" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDuplicateStore ctx:0x{:x}", context);
            emu.set_last_error(0);
            let handle = helper::handler_create("certduplicatestore");
            emu.regs_mut().rax = handle;
        }
        "CertEnumCRLContextProperties" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertEnumCRLContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertEnumCRLsInStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCRLsInStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumCRLsInStore");
        }
        "CertEnumCTLContextProperties" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertEnumCTLContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertEnumCTLsInStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCTLsInStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumCTLsInStore");
        }
        "CertEnumCertificateContextProperties" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertEnumCertificateContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertEnumCertificatesInStore" => {
            let store = emu.regs().rcx;
            let prev_ctx = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
                store,
                prev_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CertEnumPhysicalStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumPhysicalStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumPhysicalStore");
        }
        "CertEnumSubjectInSortedCTL" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumSubjectInSortedCTL arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumSubjectInSortedCTL");
        }
        "CertEnumSystemStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumSystemStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumSystemStore");
        }
        "CertEnumSystemStoreLocation" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumSystemStoreLocation arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumSystemStoreLocation");
        }
        "CertFindAttribute" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindAttribute store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindattribute");
            emu.regs_mut().rax = handle;
        }
        "CertFindCRLInStore" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindCRLInStore store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindcrlinstore");
            emu.regs_mut().rax = handle;
        }
        "CertFindCTLInStore" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindCTLInStore store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindctlinstore");
            emu.regs_mut().rax = handle;
        }
        "CertFindCertificateInCRL" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindCertificateInCRL store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindcertificateincrl");
            emu.regs_mut().rax = handle;
        }
        "CertFindChainInStore" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindChainInStore store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindchaininstore");
            emu.regs_mut().rax = handle;
        }
        "CertFindExtension" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindExtension store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindextension");
            emu.regs_mut().rax = handle;
        }
        "CertFindRDNAttr" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindRDNAttr store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindrdnattr");
            emu.regs_mut().rax = handle;
        }
        "CertFindSubjectInCTL" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindSubjectInCTL store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindsubjectinctl");
            emu.regs_mut().rax = handle;
        }
        "CertFindSubjectInSortedCTL" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertFindSubjectInSortedCTL store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certfindsubjectinsortedctl");
            emu.regs_mut().rax = handle;
        }
        "CertFreeCRLContext" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertFreeCRLContext ctx:0x{:x}", context);
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertFreeCTLContext" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertFreeCTLContext ctx:0x{:x}", context);
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertFreeCertificateChain" => {
            let chain_ctx = emu.regs().rcx;
            helper::handler_close(chain_ctx);
            log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
        }
        "CertFreeCertificateChainEngine" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertFreeCertificateChainEngine ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertFreeCertificateChainList" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertFreeCertificateChainList ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertGetCRLContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetCRLContextProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetCRLFromStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetCRLFromStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertGetCRLFromStore");
        }
        "CertGetCTLContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetCTLContextProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetCertificateChain" => {
            let _h_chain_engine = emu.regs().rcx;
            let _p_cert_context = emu.regs().rdx;
            let _p_time = emu.regs().r8;
            let _h_additional_store = emu.regs().r9;
            let _p_chain_para = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _dw_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let pp_chain_context = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

            let chain_h = helper::handler_create("cert_chain");
            log_red!(
                emu,
                "crypt32!CertGetCertificateChain => chain=0x{:x}",
                chain_h
            );
            if pp_chain_context != 0 {
                emu.maps.write_qword(pp_chain_context, chain_h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertGetCertificateContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetCertificateContextProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetEnhancedKeyUsage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetEnhancedKeyUsage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetIntendedKeyUsage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetIntendedKeyUsage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetIssuerCertificateFromStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetIssuerCertificateFromStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetNameString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9;
            log_red!(emu, "crypt32!CertGetNameString arg1:0x{:x}", arg1);
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "CN=localhost");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 13;
        }
        "CertGetPublicKeyLength" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetPublicKeyLength arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetServerOcspResponseContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetServerOcspResponseContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetStoreProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetStoreProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetSubjectCertificateFromStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetSubjectCertificateFromStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertGetValidUsages" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetValidUsages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CertIsRDNAttrsInCertificateName" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertIsRDNAttrsInCertificateName ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertIsStrongHashToSign" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertIsStrongHashToSign ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertIsValidCRLForCertificate" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertIsValidCRLForCertificate ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertModifyCertificatesToTrust" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertModifyCertificatesToTrust ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertNameToStr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9;
            log_red!(emu, "crypt32!CertNameToStr arg1:0x{:x}", arg1);
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "CN=localhost");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 13;
        }
        "CertOIDToAlgId" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertOIDToAlgId ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertOpenServerOcspResponse" => {
            let store = emu.regs().rcx;
            let find_type = emu.regs().rdx as u32;
            let find_para = emu.regs().r8;
            let prev = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertOpenServerOcspResponse store:0x{:x} type:0x{:x}",
                store,
                find_type
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("certopenserverocspresponse");
            emu.regs_mut().rax = handle;
        }
        "CertOpenSystemStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertOpenSystemStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertOpenSystemStore");
        }
        "CertRDNValueToStr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9;
            log_red!(emu, "crypt32!CertRDNValueToStr arg1:0x{:x}", arg1);
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "CN=localhost");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 13;
        }
        "CertRegisterPhysicalStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertRegisterPhysicalStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertRegisterSystemStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertRegisterSystemStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertRemoveEnhancedKeyUsageIdentifier" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertRemoveEnhancedKeyUsageIdentifier ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertRemoveStoreFromCollection" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertRemoveStoreFromCollection ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertRetrieveLogoOrBiometricInfo" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertRetrieveLogoOrBiometricInfo ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSaveStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSaveStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSelectCertificate" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSelectCertificate ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSelectCertificateChains" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSelectCertificateChains ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCRLStoreElement" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSerializeCRLStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCTLStoreElement" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSerializeCTLStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCertificateStoreElement" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSerializeCertificateStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCRLContextProperty" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSetCRLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCTLContextProperty" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSetCTLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCertificateContextPropertiesFromCTLEntry" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextPropertiesFromCTLEntry ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCertificateContextProperty" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetEnhancedKeyUsage" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSetEnhancedKeyUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetStoreProperty" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSetStoreProperty ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CertSrvBackupClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupEnd" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSrvBackupEnd ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupFree" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CertSrvBackupFree handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupGetBackupLogsW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvBackupGetBackupLogsW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupGetDatabaseNamesW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvBackupGetDatabaseNamesW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupGetDynamicFileListW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvBackupGetDynamicFileListW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupOpenFileW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            let hndl = helper::handler_create("CertSrvBackupOpenFileW");
            log_red!(
                emu,
                "crypt32!CertSrvBackupOpenFileW '{}' => 0x{:x}",
                s,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CertSrvBackupPrepareW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvBackupPrepareW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupRead" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSrvBackupRead ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvBackupTruncateLogs" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSrvBackupTruncateLogs ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvIsServerOnlineW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvIsServerOnlineW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestoreEnd" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSrvRestoreEnd ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestoreGetDatabaseLocationsW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvRestoreGetDatabaseLocationsW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestorePrepareW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvRestorePrepareW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestoreRegisterComplete" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterComplete ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestoreRegisterThroughFile" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterThroughFile ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvRestoreRegisterW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvRestoreRegisterW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSrvServerControlW" => {
            let str_ptr = emu.regs().rcx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "crypt32!CertSrvServerControlW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertStrToName" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertStrToName ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertUnregisterPhysicalStore" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertUnregisterPhysicalStore ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertUnregisterSystemStore" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CertUnregisterSystemStore ctx:0x{:x}", context);
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCRLRevocation" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertVerifyCRLRevocation ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCRLTimeValidity" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertVerifyCRLTimeValidity ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCTLUsage" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertVerifyCTLUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCertificateChainPolicy" => {
            let _policy_oid = emu.regs().rcx;
            let _p_chain_context = emu.regs().rdx;
            let _p_policy_para = emu.regs().r8;
            let p_policy_status = emu.regs().r9;

            log_red!(
                emu,
                "crypt32!CertVerifyCertificateChainPolicy status_out:0x{:x}",
                p_policy_status
            );

            if p_policy_status != 0 {
                emu.maps.write_dword(p_policy_status, 4);
                emu.maps.write_dword(p_policy_status + 4, 0);
                emu.maps.write_dword(p_policy_status + 8, 0);
                emu.maps.write_dword(p_policy_status + 12, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyRevocation" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertVerifyRevocation ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifySubjectCertificateContext" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertVerifySubjectCertificateContext ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyTimeValidity" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertVerifyTimeValidity ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyValidityNesting" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertVerifyValidityNesting ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptAcquireContext" => CryptAcquireContext(emu),
        "CryptBinaryToString" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptBinaryToString arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminAcquireContext" => {
            let cat_admin_ptr = emu.regs().rcx;
            let subsystem_ptr = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATAdminAcquireContext");
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptcatadminacquirecontext");
            if cat_admin_ptr != 0 {
                emu.maps.write_qword(cat_admin_ptr, handle);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminAcquireContext2" => {
            let cat_admin_ptr = emu.regs().rcx;
            let subsystem_ptr = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATAdminAcquireContext2");
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptcatadminacquirecontext2");
            if cat_admin_ptr != 0 {
                emu.maps.write_qword(cat_admin_ptr, handle);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminAddCatalog" => {
            let admin = emu.regs().rcx;
            let catalog_file = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATAdminAddCatalog admin:0x{:x}", admin);
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptcatadmincatalog");
            emu.regs_mut().rax = handle;
        }
        "CryptCATAdminCalcHashFromFileHandle" => {
            let file_handle = emu.regs().rcx;
            let hash_size_ptr = emu.regs().rdx;
            let hash_ptr = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle file:0x{:x}",
                file_handle
            );
            if hash_size_ptr != 0 {
                emu.maps.write_dword(hash_size_ptr, 20);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminCalcHashFromFileHandle2" => {
            let file_handle = emu.regs().rcx;
            let hash_size_ptr = emu.regs().rdx;
            let hash_ptr = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle2 file:0x{:x}",
                file_handle
            );
            if hash_size_ptr != 0 {
                emu.maps.write_dword(hash_size_ptr, 20);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminEnumCatalogFromHash" => {
            let admin = emu.regs().rcx;
            let hash_ptr = emu.regs().rdx;
            let hash_len = emu.regs().r8 as u32;
            log_red!(
                emu,
                "crypt32!CryptCATAdminEnumCatalogFromHash admin:0x{:x}",
                admin
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptcatenumcatalog");
            emu.regs_mut().rax = handle;
        }
        "CryptCATAdminReleaseCatalogContext" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseCatalogContext handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminReleaseContext" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseContext handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminRemoveCatalog" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminResolveCatalogPath" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATAdminResolveCatalogPath arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CryptCATCDFClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFEnumAttributesWithCDFTag" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumAttributesWithCDFTag arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFEnumCatAttributes" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumCatAttributes arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFEnumMembersByCDFTagEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumMembersByCDFTagEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFOpen" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let hndl = helper::handler_create("CryptCATCDFOpen");
            log_red!(
                emu,
                "crypt32!CryptCATCDFOpen arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
                arg1,
                arg2,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CryptCATCatalogInfoFromContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCATCatalogInfoFromContext arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CryptCATClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CryptCATEnumerateAttr" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATEnumerateAttr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATEnumerateCatAttr" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATEnumerateCatAttr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATEnumerateMember" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATEnumerateMember arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATGetAttrInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATGetAttrInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATGetMemberInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptCATGetMemberInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATOpen" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let hndl = helper::handler_create("CryptCATOpen");
            log_red!(
                emu,
                "crypt32!CryptCATOpen arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
                arg1,
                arg2,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CryptContextAddRef" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptContextAddRef arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptCreateKeyIdentifierFromCSP" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptCreateKeyIdentifierFromCSP arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecodeMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDecodeMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecodeObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDecodeObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecrypt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDecrypt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecryptAndVerifyMessageSignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptDecryptAndVerifyMessageSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecryptMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDecryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDeriveKey" => {
            let h_prov = emu.regs().rcx;
            let algid = emu.regs().rdx as u32;
            let h_hash = emu.regs().r8;
            let flags = emu.regs().r9;
            let h_key_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptDeriveKey prov: 0x{:x} alg: 0x{:x} hash: 0x{:x}",
                h_prov,
                algid,
                h_hash
            );
            let h_key = helper::handler_create(&format!("derivedkey:0x{:x}", algid));
            if h_key_ptr != 0 {
                emu.maps.write_qword(h_key_ptr, h_key);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDestroyHash" => {
            let h_hash = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDestroyHash hash: 0x{:x}", h_hash);
            if h_hash != 0 {
                helper::handler_close(h_hash);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDestroyKey" => {
            let h_key = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptDestroyKey key: 0x{:x}", h_key);
            if h_key != 0 {
                helper::handler_close(h_key);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDuplicateHash" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptDuplicateHash arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptduplicatehash");
            emu.regs_mut().rax = handle;
        }
        "CryptDuplicateKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptDuplicateKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptduplicatekey");
            emu.regs_mut().rax = handle;
        }
        "CryptEncodeObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEncodeObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEncodeObjectEx" => {
            let encoding = emu.regs().rcx;
            let struct_type = emu.regs().rdx;
            let struct_info = emu.regs().r8;
            let flags = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
                encoding,
                struct_type,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEncrypt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEncrypt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEncryptMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEncryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumKeyIdentifierProperties" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptEnumKeyIdentifierProperties arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumOIDFunction" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEnumOIDFunction arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumOIDInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEnumOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumProviderTypes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEnumProviderTypes arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumProviders" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptEnumProviders arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportKey" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptExportKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPKCS8" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptExportPKCS8 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPKCS8Ex" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptExportPKCS8Ex arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptExportPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPublicKeyInfoEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptExportPublicKeyInfoEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPublicKeyInfoFromBCryptKeyHandle" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfoFromBCryptKeyHandle arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptFindCertificateKeyProvInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptFindCertificateKeyProvInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptFindLocalizedName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptFindLocalizedName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptFindOIDInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptFindOIDInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptFormatObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptFormatObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptFreeOIDFunctionAddress" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptFreeOIDFunctionAddress ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptGenKey" => {
            let h_prov = emu.regs().rcx;
            let algid = emu.regs().rdx as u32;
            let flags = emu.regs().r8;
            let h_key_ptr = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
                h_prov,
                algid,
                flags
            );
            let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
            if h_key_ptr != 0 {
                emu.maps.write_qword(h_key_ptr, h_key);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptGenRandom" => {
            let h_prov = emu.regs().rcx;
            let len = emu.regs().rdx as u32;
            let buffer = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
                h_prov,
                len,
                buffer
            );
            // Fill buffer with deterministic pseudo-random bytes based on emu.pos
            if buffer != 0 {
                for i in 0..len {
                    let byte = ((emu.pos as u32)
                        .wrapping_mul(1103515245)
                        .wrapping_add(i * 12345)) as u8;
                    emu.maps.write_byte(buffer + i as u64, byte);
                }
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptGetDefaultOIDDllList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetDefaultOIDDllList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetDefaultOIDFunctionAddress" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetDefaultOIDFunctionAddress arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetDefaultProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptGetDefaultProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptGetHashParam" => CryptGetHashParam(emu),
        "CryptGetKeyIdentifierProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetKeyIdentifierProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetKeyParam" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptGetKeyParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptGetMessageCertificates" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetMessageCertificates arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetMessageSignerCount" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetMessageSignerCount arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetOIDFunctionAddress" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetOIDFunctionAddress arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetOIDFunctionValue" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptGetOIDFunctionValue arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetObjectUrl" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptGetObjectUrl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptGetProvParam" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptGetProvParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptGetUserKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptGetUserKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptgetuserkey");
            emu.regs_mut().rax = handle;
        }
        "CryptHashCertificate" => {
            let alg_id = emu.regs().rdx;
            let cert_ptr = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let hash_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hash_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            emu.set_last_error(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu,
                alg_id as u32,
                cert_ptr,
                cert_len,
                hash_ptr,
                hash_len_ptr,
            );
        }
        "CryptHashCertificate2" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashCertificate2 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashData" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashSessionKey" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashSessionKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashToBeSigned" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptHashToBeSigned arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportKey" => {
            let h_prov = emu.regs().rcx;
            let data_ptr = emu.regs().rdx;
            let data_len = emu.regs().r8 as u32;
            let h_pub_key = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptImportKey prov: 0x{:x} data: 0x{:x} len: {} pubkey: 0x{:x}",
                h_prov,
                data_ptr,
                data_len,
                h_pub_key
            );
            // 5th arg: flags, 6th arg: phKey
            let h_key_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let h_key = helper::handler_create("importedkey");
            if h_key_ptr != 0 {
                emu.maps.write_qword(h_key_ptr, h_key);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptImportPKCS8" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptImportPKCS8 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfoEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfoEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfoEx2" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfoEx2 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptInitOIDFunctionSet" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptInitOIDFunctionSet arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptInstallDefaultContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptInstallDefaultContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptInstallOIDFunctionAddress" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptInstallOIDFunctionAddress arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMemAlloc" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMemAlloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMemFree" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMemFree ctx:0x{:x}", context);
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMemRealloc" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMemRealloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgCalculateEncodedLength" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptMsgCalculateEncodedLength arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgClose" => {
            let msg = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgClose msg:0x{:x}", msg);
            if msg != 0 {
                helper::handler_close(msg);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgControl" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgControl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgCountersign" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgCountersign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgCountersignEncoded" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgCountersignEncoded arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgDuplicate" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgDuplicate ctx:0x{:x}", context);
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptmsgduplicate");
            emu.regs_mut().rax = handle;
        }
        "CryptMsgEncodeAndSignCTL" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgEncodeAndSignCTL arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgGetAndVerifySigner" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgGetAndVerifySigner arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgGetParam" => {
            let hmsg = emu.regs().rcx;
            let param_type = emu.regs().rdx;
            let index = emu.regs().r8;
            let pv_data = emu.regs().r9;
            let pcb_data = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgGetParam msg:0x{:x} type:{} idx:{}",
                hmsg,
                param_type,
                index
            );
            if pcb_data != 0 {
                emu.maps.write_dword(pcb_data, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptMsgOpenToDecode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CryptMsgOpenToDecode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptmsgopentodecode");
            emu.regs_mut().rax = handle;
        }
        "CryptMsgOpenToEncode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CryptMsgOpenToEncode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("cryptmsgopentoencode");
            emu.regs_mut().rax = handle;
        }
        "CryptMsgSignCTL" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMsgSignCTL arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgUpdate" => {
            let hmsg = emu.regs().rcx;
            let pb_data = emu.regs().rdx;
            let cb_data = emu.regs().r8;
            let f_final = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptMsgUpdate msg:0x{:x} data:0x{:x} len:{} final:{}",
                hmsg,
                pb_data,
                cb_data,
                f_final
            );
            if hmsg != 0 && pb_data != 0 && cb_data > 0 {
                let mut buf = vec![0u8; cb_data as usize];
                for i in 0..cb_data as u64 {
                    buf[i as usize] = emu.maps.read_byte(pb_data + i).unwrap_or(0);
                }
                helper::handler_put_bytes(hmsg, &buf);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptMsgVerifyCountersignatureEncoded" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncoded arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgVerifyCountersignatureEncodedEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncodedEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptProtectData" => {
            let p_data_in = emu.regs().rcx;
            let _sz_data_descr = emu.regs().rdx;
            let _p_optional_entropy = emu.regs().r8;
            let _pv_reserved = emu.regs().r9;
            let _p_prompt_struct = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _dw_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p_data_out = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

            let mut in_len = 0u32;
            let mut in_ptr = 0u64;
            if p_data_in != 0 {
                in_len = emu.maps.read_dword(p_data_in).unwrap_or(0);
                in_ptr = emu.maps.read_qword(p_data_in + 8).unwrap_or(0);
            }

            log_red!(
                emu,
                "crypt32!CryptProtectData in_len:{} in_ptr:0x{:x} out:0x{:x}",
                in_len,
                in_ptr,
                p_data_out
            );

            if in_ptr != 0 && in_len > 0 && p_data_out != 0 {
                let len = in_len.min(0x100_0000) as usize;
                let mut data = emu.maps.read_bytes(in_ptr, len).to_vec();
                for b in data.iter_mut() {
                    *b ^= 0x5A;
                }
                if let Some(alloc) = emu.maps.alloc(len as u64) {
                    emu.maps.write_bytes(alloc, &data);
                    emu.maps.write_dword(p_data_out, len as u32);
                    emu.maps.write_qword(p_data_out + 8, alloc);
                }
            }

            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptProtectMemory" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptProtectMemory arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptQueryObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptQueryObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptRegisterDefaultOIDFunction" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptRegisterDefaultOIDFunction arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptRegisterOIDFunction" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptRegisterOIDFunction arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptRegisterOIDInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptRegisterOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptReleaseContext" => {
            let h_prov = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptReleaseContext prov: 0x{:x}", h_prov);
            if h_prov != 0 {
                helper::handler_close(h_prov);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptRetrieveObjectByUrl" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptRetrieveObjectByUrl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPAddProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPAddProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPCreateIndirectData" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPCreateIndirectData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPGetCaps" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptSIPGetCaps arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSIPGetSignedDataMsg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptSIPGetSignedDataMsg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSIPLoad" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPLoad arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPPutSignedDataMsg" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPPutSignedDataMsg arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPRemoveProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPRemoveProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPRemoveSignedDataMsg" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPRemoveSignedDataMsg arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPRetrieveSubjectGuid" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptSIPRetrieveSubjectGuid arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSIPRetrieveSubjectGuidForCatalogFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptSIPRetrieveSubjectGuidForCatalogFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            if arg4 != 0 {
                emu.maps.write_dword(arg4, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSIPVerifyIndirectData" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPVerifyIndirectData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetHashParam" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetHashParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetKeyIdentifierProperty" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptSetKeyIdentifierProperty arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetKeyParam" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetKeyParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetOIDFunctionValue" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetOIDFunctionValue arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProvParam" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetProvParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProviderEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSetProviderEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignAndEncodeCertificate" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptSignAndEncodeCertificate arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignAndEncryptMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSignAndEncryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignCertificate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSignCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignHash" => {
            let h_hash = emu.regs().rcx;
            let key_spec = emu.regs().rdx as u32;
            let desc_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            log_red!(
                emu,
                "crypt32!CryptSignHash hash: 0x{:x} spec: {} flags: 0x{:x}",
                h_hash,
                key_spec,
                flags
            );
            // 5th arg: pbSignature, 6th arg: pdwSigLen
            let sig_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let sig_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if sig_ptr == 0 {
                // Query mode — return required size
                if sig_len_ptr != 0 {
                    emu.maps.write_dword(sig_len_ptr, 128);
                }
            } else {
                // Write fake signature
                for i in 0..128u64 {
                    emu.maps.write_byte(sig_ptr + i, 0xAA);
                }
                if sig_len_ptr != 0 {
                    emu.maps.write_dword(sig_len_ptr, 128);
                }
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptSignMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSignMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignMessageWithKey" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSignMessageWithKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptStringToBinary" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgCertMgr" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgCertMgr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgSelectCertificate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgSelectCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgSelectCertificateFromStore" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptUIDlgSelectCertificateFromStore arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgViewCertificate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgViewCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgViewContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgViewContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIDlgViewSignerInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgViewSignerInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIWizDigitalSign" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIWizDigitalSign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIWizExport" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIWizExport arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIWizFreeDigitalSignContext" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(
                emu,
                "crypt32!CryptUIWizFreeDigitalSignContext handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
        }
        "CryptUIWizImport" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIWizImport arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUninstallDefaultContext" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptUninstallDefaultContext ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUnprotectData" => {
            let p_data_in = emu.regs().rcx;
            let pp_sz_data_descr = emu.regs().rdx;
            let _p_optional_entropy = emu.regs().r8;
            let _pv_reserved = emu.regs().r9;
            let _p_prompt_struct = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _dw_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p_data_out = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

            let mut in_len = 0u32;
            let mut in_ptr = 0u64;
            if p_data_in != 0 {
                in_len = emu.maps.read_dword(p_data_in).unwrap_or(0);
                in_ptr = emu.maps.read_qword(p_data_in + 8).unwrap_or(0);
            }

            log_red!(
                emu,
                "crypt32!CryptUnprotectData in_len:{} in_ptr:0x{:x} out:0x{:x}",
                in_len,
                in_ptr,
                p_data_out
            );

            if in_ptr != 0 && in_len > 0 && p_data_out != 0 {
                let len = in_len.min(0x100_0000) as usize;
                let mut data = emu.maps.read_bytes(in_ptr, len).to_vec();
                for b in data.iter_mut() {
                    *b ^= 0x5A;
                }
                if let Some(alloc) = emu.maps.alloc(len as u64) {
                    emu.maps.write_bytes(alloc, &data);
                    emu.maps.write_dword(p_data_out, len as u32);
                    emu.maps.write_qword(p_data_out + 8, alloc);
                }
            }
            if pp_sz_data_descr != 0 {
                emu.maps.write_qword(pp_sz_data_descr, 0);
            }

            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUnprotectMemory" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUnprotectMemory arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUnregisterDefaultOIDFunction" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptUnregisterDefaultOIDFunction ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUnregisterOIDFunction" => {
            let context = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptUnregisterOIDFunction ctx:0x{:x}",
                context
            );
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUnregisterOIDInfo" => {
            let context = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUnregisterOIDInfo ctx:0x{:x}", context);
            if context != 0 {
                helper::handler_close(context);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUpdateProtectedState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUpdateProtectedState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyCertificateSignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyCertificateSignatureEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignatureEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyDetachedMessageHash" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageHash arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyDetachedMessageSignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageHash" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptVerifyMessageHash arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageSignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptVerifyMessageSignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageSignatureWithKey" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageSignatureWithKey arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifySignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptVerifySignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlAddObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlAddObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CryptXmlClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlCreateReference" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let hndl = helper::handler_create("CryptXmlCreateReference");
            log_red!(
                emu,
                "crypt32!CryptXmlCreateReference arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
                arg1,
                arg2,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CryptXmlDigestReference" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlDigestReference arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlEncode" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlEncode arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetAlgorithmInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetAlgorithmInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetDocContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetDocContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetReference" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetReference arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetSignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetSignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetStatus" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetStatus arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlGetTransforms" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlGetTransforms arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlImportPublicKey" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlImportPublicKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlOpenToDecode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let hndl = helper::handler_create("CryptXmlOpenToDecode");
            log_red!(
                emu,
                "crypt32!CryptXmlOpenToDecode arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
                arg1,
                arg2,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CryptXmlOpenToEncode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let hndl = helper::handler_create("CryptXmlOpenToEncode");
            log_red!(
                emu,
                "crypt32!CryptXmlOpenToEncode arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
                arg1,
                arg2,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CryptXmlSetHMACSecret" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlSetHMACSecret arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlSign" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlSign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptXmlVerifySignature" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptXmlVerifySignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PFXExportCertStore" => {
            let blob = emu.regs().rcx;
            log_red!(emu, "crypt32!PFXExportCertStore blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PFXExportCertStoreEx" => {
            let blob = emu.regs().rcx;
            log_red!(emu, "crypt32!PFXExportCertStoreEx blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PFXImportCertStore" => {
            let _p_pfx = emu.regs().rcx;
            let _sz_password = emu.regs().rdx;
            let _dw_flags = emu.regs().r8;
            let handle = helper::handler_create("pfx_cert_store");
            log_red!(emu, "crypt32!PFXImportCertStore => 0x{:x}", handle);
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "PFXIsPFXBlob" => {
            let blob = emu.regs().rcx;
            log_red!(emu, "crypt32!PFXIsPFXBlob blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PFXVerifyPassword" => {
            let blob = emu.regs().rcx;
            log_red!(emu, "crypt32!PFXVerifyPassword blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!(
                "attempt to call unimplemented crypt32 API 0x{:x} {}",
                addr,
                api
            );
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "crypt32!PkiInitializeCriticalSection flags: 0x{:x} addr: 0x{:x}",
        flags,
        addr
    );

    emu.regs_mut().rax = 1;
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    let string = emu.regs().rcx;
    let num_chars = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ptr = emu.regs().r9;
    let inout_sz = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_a(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
}

fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = emu.regs().rcx;
    let struct_type = emu.regs().rdx;
    let encoded = emu.regs().r8;
    let cb_encoded = emu.regs().r9;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let decode_para = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let pv_struct_info = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let pcb_struct_info = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} encoded: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        encoded,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    emu.regs_mut().rax = 1;
}

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = emu.regs().rcx;
    let encoding_type = emu.regs().rdx;
    let hcrypt_prov = emu.regs().r8;
    let flags = emu.regs().r9;
    let pv_para = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let subsystem = emu.regs().rdx;

    let name = emu.maps.read_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let subsystem = emu.regs().rdx;

    let name = emu.maps.read_wide_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store);
    emu.regs_mut().rax = 1;
}

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = emu.regs().rcx;
    let encoding_type = emu.regs().rdx;
    let find_flags = emu.regs().r8;
    let find_type = emu.regs().r9;
    let find_para = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let prev_cert = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    emu.regs_mut().rax = 0;
}

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = emu.regs().rcx;

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    emu.regs_mut().rax = 1;
}

fn CryptAcquireContext(emu: &mut emu::Emu) {
    let h_prov_ptr = emu.regs().rcx;
    let container_ptr = emu.regs().rdx;
    let provider_ptr = emu.regs().r8;
    let prov_type = emu.regs().r9 as u32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;

    let container = if container_ptr != 0 {
        emu.maps.read_string(container_ptr)
    } else {
        String::new()
    };
    let provider = if provider_ptr != 0 {
        emu.maps.read_string(provider_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "crypt32!CryptAcquireContext container: '{}' provider: '{}' type: {} flags: 0x{:x}",
        container,
        provider,
        prov_type,
        flags
    );

    let h_prov = helper::handler_create(&format!("cryptprov:{}", prov_type));
    if h_prov_ptr != 0 {
        emu.maps.write_qword(h_prov_ptr, h_prov);
    }
    emu.regs_mut().rax = 1; // TRUE
}

fn CryptCreateHash(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let h_key = emu.regs().r8;
    let _flags = emu.regs().r9;
    let h_hash_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let alg_name = match algid {
        0x8001 => "MD2",
        0x8002 => "MD4",
        0x8003 => "MD5",
        0x8004 => "SHA1",
        0x800C => "SHA256",
        0x800D => "SHA384",
        0x800E => "SHA512",
        _ => "UNKNOWN",
    };

    log_red!(
        emu,
        "crypt32!CryptCreateHash prov: 0x{:x} alg: 0x{:x} ({}) key: 0x{:x}",
        h_prov,
        algid,
        alg_name,
        h_key
    );

    let h_hash = helper::handler_create(&format!("hash:{}", alg_name));
    if h_hash_ptr != 0 {
        emu.maps.write_qword(h_hash_ptr, h_hash);
    }
    emu.regs_mut().rax = 1; // TRUE
}

fn CryptGetHashParam(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let param = emu.regs().rdx as u32;
    let data_ptr = emu.regs().r8;
    let data_len_ptr = emu.regs().r9;

    log_red!(
        emu,
        "crypt32!CryptGetHashParam hash: 0x{:x} param: {} data: 0x{:x}",
        h_hash,
        param,
        data_ptr
    );

    match param {
        1 => {
            // HP_ALGID — return algorithm ID
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 0x8004); // CALG_SHA1 as default
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
            emu.regs_mut().rax = 1;
        }
        2 => {
            // HP_HASHSIZE — return hash size
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 20); // SHA1 = 20 bytes
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
            emu.regs_mut().rax = 1;
        }
        3 => {
            // HP_HASHVAL — return the hash value
            let hash_size: u32 = 20; // SHA1 default
            if data_ptr != 0 {
                // Write deterministic hash bytes
                for i in 0..hash_size {
                    let byte = ((h_hash as u32).wrapping_mul(31).wrapping_add(i)) as u8;
                    emu.maps.write_byte(data_ptr + i as u64, byte);
                }
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, hash_size);
            }
            emu.regs_mut().rax = 1;
        }
        _ => {
            log_red!(emu, "  unsupported param {}", param);
            emu.regs_mut().rax = 0;
        }
    }
}
