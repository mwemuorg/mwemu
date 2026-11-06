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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddCRLContextToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddCRLContextToStore");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCRLLinkToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddCRLLinkToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddCRLLinkToStore");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCTLContextToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddCTLContextToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddCTLContextToStore");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddCTLLinkToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddCTLLinkToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddCTLLinkToStore");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddCertificateLinkToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddCertificateLinkToStore");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddEncodedCRLToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertAddEncodedCRLToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddEncodedCRLToStore");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddEncodedCTLToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertAddEncodedCTLToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddEncodedCTLToStore");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertAddEnhancedKeyUsageIdentifier 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddEnhancedKeyUsageIdentifier");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
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
            let handle = helper::handler_create("CertAddRefServerOcspResponse");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertAddRefServerOcspResponseContext" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertAddRefServerOcspResponseContext ctx:0x{:x}",
                cert_ctx
            );
            let handle = helper::handler_create("CertAddRefServerOcspResponseContext");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertAddSerializedElementToStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertAddSerializedElementToStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddSerializedElementToStore");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAddStoreToCollection" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertAddStoreToCollection 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CertAddStoreToCollection");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertAlgIdToOID" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertAlgIdToOID ctx:0x{:x}", cert_ctx);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertCompareCertificate 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertCompareCertificateName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertCompareCertificateName 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertCompareIntegerBlob" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertCompareIntegerBlob 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertComparePublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertComparePublicKeyInfo 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertControlStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CertControlStore 0x{:x} 0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertDeleteCTLFromStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertDeleteCTLFromStore ctx:0x{:x}", cert_ctx);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertDeleteCertificateFromStore" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertDeleteCertificateFromStore ctx:0x{:x}",
                cert_ctx
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCRLContextProperties 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCTLContextProperties 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertEnumCertificateContextProperties 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {}
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
            let store = emu.regs().rcx;
            let subject_ctx = emu.regs().rdx;
            let prev_issuer = emu.regs().r8;
            let flags_ptr = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertGetIssuerCertificateFromStore store:0x{:x} subj:0x{:x}",
                store,
                subject_ctx
            );
            emu.set_last_error(0);
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0);
            }
            let handle = helper::handler_create("cert_issuer_ctx");
            emu.regs_mut().rax = handle;
        }
        "CertGetNameString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9;
            log_red!(emu, "crypt32!CertGetNameString arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let server_ocsp_status = emu.regs().rcx;
            let _flags = emu.regs().rdx;
            let _reserved = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertGetServerOcspResponseContext status:0x{:x}",
                server_ocsp_status
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("ocsp_response_ctx");
            emu.regs_mut().rax = handle;
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
            let store = emu.regs().rcx;
            let cert_encoding = emu.regs().rdx as u32;
            let cert_id = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertGetSubjectCertificateFromStore store:0x{:x} enc:0x{:x}",
                store,
                cert_encoding
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("cert_subject_ctx");
            emu.regs_mut().rax = handle;
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertIsRDNAttrsInCertificateName 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertIsStrongHashToSign" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertIsStrongHashToSign 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertIsValidCRLForCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertIsValidCRLForCertificate 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CertModifyCertificatesToTrust" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertModifyCertificatesToTrust ctx:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertNameToStr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9;
            log_red!(emu, "crypt32!CertNameToStr arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "CN=localhost");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 13;
        }
        "CertOIDToAlgId" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertOIDToAlgId ctx:0x{:x}", cert_ctx);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "CN=localhost");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 13;
        }
        "CertRegisterPhysicalStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertRegisterPhysicalStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertRegisterSystemStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertRegisterSystemStore 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertRetrieveLogoOrBiometricInfo 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSaveStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CertSaveStore 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSelectCertificate" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSelectCertificate ctx:0x{:x}", cert_ctx);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertSelectCertificateChains" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertSelectCertificateChains 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCRLStoreElement" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSerializeCRLStoreElement 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCTLStoreElement" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSerializeCTLStoreElement 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSerializeCertificateStoreElement" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSerializeCertificateStoreElement 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCRLContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSetCRLContextProperty 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCTLContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSetCTLContextProperty 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCertificateContextPropertiesFromCTLEntry" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextPropertiesFromCTLEntry 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetCertificateContextProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextProperty 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetEnhancedKeyUsage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertSetEnhancedKeyUsage 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertSetStoreProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertSetStoreProperty 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let hbc = emu.regs().rcx;
            log_red!(emu, "crypt32!CertSrvBackupRead hbc:0x{:x}", hbc);
            let result = if helper::handler_exist(hbc) {
                0u64
            } else {
                1u64
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "CertSrvBackupTruncateLogs" => {
            let cert_ctx = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CertSrvBackupTruncateLogs ctx:0x{:x}",
                cert_ctx
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterComplete ctx:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CertSrvRestoreRegisterThroughFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterThroughFile ctx:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CertStrToName encoding:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CertVerifyCRLRevocation 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCRLTimeValidity" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertVerifyCRLTimeValidity 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyCTLUsage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CertVerifyCTLUsage 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CertVerifyRevocation 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifySubjectCertificateContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CertVerifySubjectCertificateContext 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyTimeValidity" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertVerifyTimeValidity 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CertVerifyValidityNesting" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CertVerifyValidityNesting 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptAcquireContext" => CryptAcquireContext(emu),
        "CryptBinaryToString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptBinaryToString arg1:0x{:x}", arg1);
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let h_cat_admin = emu.regs().rcx;
            let h_cat_info = emu.regs().rdx;
            let dw_flags = emu.regs().r8 as u32;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseCatalogContext admin:0x{:x} info:0x{:x} flags:0x{:x}",
                h_cat_admin,
                h_cat_info,
                dw_flags
            );
            if h_cat_info != 0 {
                helper::handler_close(h_cat_info);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminReleaseContext" => {
            let h_cat_admin = emu.regs().rcx;
            let dw_flags = emu.regs().rdx as u32;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseContext admin:0x{:x} flags:0x{:x}",
                h_cat_admin,
                dw_flags
            );
            if h_cat_admin != 0 {
                helper::handler_close(h_cat_admin);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptCATAdminRemoveCatalog" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATAdminResolveCatalogPath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptCATAdminResolveCatalogPath arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATCDFClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CryptCATCDFClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CryptCATCDFEnumAttributesWithCDFTag" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumAttributesWithCDFTag arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATCDFEnumCatAttributes" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumCatAttributes arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATCDFEnumMembersByCDFTagEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumMembersByCDFTagEx arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptCATCatalogInfoFromContext arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "crypt32!CryptCATClose handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CryptCATEnumerateAttr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptCATEnumerateAttr arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATEnumerateCatAttr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATEnumerateCatAttr arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATEnumerateMember" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATEnumerateMember arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATGetAttrInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptCATGetAttrInfo arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCATGetMemberInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptCATGetMemberInfo arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptContextAddRef arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptContextAddRef");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptCreateKeyIdentifierFromCSP" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptCreateKeyIdentifierFromCSP arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("CryptCreateKeyIdentifierFromCSP");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptDecodeMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(emu, "crypt32!CryptDecodeMessage arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptDecodeObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptDecodeObject 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecrypt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptDecrypt 0x{:x} 0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecryptAndVerifyMessageSignature" => {
            let p_decrypt_para = emu.regs().rcx;
            let p_verify_para = emu.regs().rdx;
            let dw_signer_index = emu.regs().r8 as u32;
            let pb_encrypted_blob = emu.regs().r9;
            let cb_encrypted_blob = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
            let ppb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let pcb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let pp_xchg_cert = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            let pp_signer_cert = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);

            let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
                let len = (cb_encrypted_blob as usize).min(256);
                emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
            } else {
                Vec::new()
            };

            if ppb_decrypted != 0 && !blob_data.is_empty() {
                let out_ptr = emu.maps.alloc(blob_data.len() as u64).expect("alloc");
                emu.maps.write_bytes(out_ptr, &blob_data);
                emu.maps.write_qword(ppb_decrypted, out_ptr);
                if pcb_decrypted != 0 {
                    emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
                }
            }

            log_red!(
                emu,
                "crypt32!CryptDecryptAndVerifyMessageSignature blob_sz:{}",
                cb_encrypted_blob
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptDecryptMessage" => {
            let p_decrypt_para = emu.regs().rcx;
            let pb_encrypted_blob = emu.regs().rdx;
            let cb_encrypted_blob = emu.regs().r8 as u32;
            let pb_decrypted = emu.regs().r9;
            let pcb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let pp_xchg_cert = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

            let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
                let len = (cb_encrypted_blob as usize).min(256);
                emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
            } else {
                Vec::new()
            };

            if pb_decrypted != 0 && !blob_data.is_empty() {
                emu.maps.write_bytes(pb_decrypted, &blob_data);
            }
            if pcb_decrypted != 0 {
                emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
            }

            log_red!(
                emu,
                "crypt32!CryptDecryptMessage blob_sz:{} out:0x{:x}",
                cb_encrypted_blob,
                pb_decrypted
            );
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptEncodeObject 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {}
        }
        "CryptEncrypt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(emu, "crypt32!CryptEncrypt 0x{:x} 0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEncryptMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptEncryptMessage 0x{:x} 0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumKeyIdentifierProperties" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptEnumKeyIdentifierProperties 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumOIDFunction" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptEnumOIDFunction 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumOIDInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptEnumOIDInfo 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptEnumProviderTypes" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptEnumProviderTypes arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptEnumProviders" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptEnumProviders arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptExportKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptExportKey arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptExportPKCS8" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(emu, "crypt32!CryptExportPKCS8 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPKCS8Ex" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(emu, "crypt32!CryptExportPKCS8Ex arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptExportPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfo 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPublicKeyInfoEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfoEx 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptExportPublicKeyInfoFromBCryptKeyHandle" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfoFromBCryptKeyHandle 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(emu, "crypt32!CryptFormatObject 0x{:x} 0x{:x}", arg1, arg2);
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
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
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptGetDefaultProvider arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptGetKeyParam 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(emu, "crypt32!CryptGetObjectUrl arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptGetProvParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptGetProvParam 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            helper::advance_tick(emu, 1);
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptHashCertificate2 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptHashData 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(emu, "crypt32!CryptHashMessage 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptHashPublicKeyInfo 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashSessionKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptHashSessionKey 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptHashToBeSigned" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptHashToBeSigned 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptImportPKCS8 0x{:x} 0x{:x}", arg1, arg2);
            let handle = helper::handler_create("CryptImportPKCS8");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptImportPublicKeyInfo 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptImportPublicKeyInfo");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfoEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptImportPublicKeyInfoEx 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptImportPublicKeyInfoEx");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptImportPublicKeyInfoEx2" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptImportPublicKeyInfoEx2 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptImportPublicKeyInfoEx2");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptInitOIDFunctionSet" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptInitOIDFunctionSet arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptInstallDefaultContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptInstallDefaultContext 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptInstallDefaultContext");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptInstallOIDFunctionAddress" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptInstallOIDFunctionAddress 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptInstallOIDFunctionAddress");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMemAlloc" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptMemAlloc arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptMemAlloc");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptMemRealloc 0x{:x} 0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgCalculateEncodedLength" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgCalculateEncodedLength 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptMsgControl 0x{:x} 0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CryptMsgCountersign" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptMsgCountersign 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgCountersignEncoded" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgCountersignEncoded 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptMsgEncodeAndSignCTL 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgGetAndVerifySigner" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgGetAndVerifySigner 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptMsgSignCTL 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            if hmsg != 0 && pb_data != 0 && cb_data > 0 {
                let mut buf = vec![0u8; cb_data as usize];
                for i in 0..cb_data {
                    buf[i as usize] = emu.maps.read_byte(pb_data + i).unwrap_or(0);
                }
                helper::handler_put_bytes(hmsg, &buf);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptMsgVerifyCountersignatureEncoded" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncoded 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptMsgVerifyCountersignatureEncodedEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncodedEx 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptProtectData" => {
            let p_data_in = emu.regs().rcx;
            let sz_data_descr = emu.regs().rdx;
            let _p_optional_entropy = emu.regs().r8;
            let _pv_reserved = emu.regs().r9;
            let _p_prompt_struct = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _dw_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p_data_out = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

            let descr = if sz_data_descr != 0 {
                emu.maps.read_wide_string(sz_data_descr)
            } else {
                String::new()
            };

            let mut in_len = 0u32;
            let mut in_ptr = 0u64;
            if p_data_in != 0 {
                in_len = emu.maps.read_dword(p_data_in).unwrap_or(0);
                in_ptr = emu.maps.read_qword(p_data_in + 8).unwrap_or(0);
            }

            log_red!(
                emu,
                "crypt32!CryptProtectData descr:'{}' in_len:{} in_ptr:0x{:x} out:0x{:x}",
                descr,
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptProtectMemory 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptRegisterDefaultOIDFunction 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptRegisterOIDFunction" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptRegisterOIDFunction 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptRegisterOIDInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CryptRegisterOIDInfo 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptReleaseContext" => {
            let h_prov = emu.regs().rcx;
            let dw_flags = emu.regs().rdx;
            log_red!(
                emu,
                "crypt32!CryptReleaseContext prov: 0x{:x} flags: 0x{:x}",
                h_prov,
                dw_flags
            );
            if h_prov != 0 {
                helper::handler_close(h_prov);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptRetrieveObjectByUrl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(emu, "crypt32!CryptRetrieveObjectByUrl arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSIPAddProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPAddProvider arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptSIPAddProvider");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSIPCreateIndirectData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptSIPCreateIndirectData 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("CryptSIPCreateIndirectData");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptSIPLoad 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("CryptSIPLoad");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPPutSignedDataMsg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSIPPutSignedDataMsg 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSIPRemoveProvider" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptSIPRemoveProvider arg1:0x{:x}", arg1);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSIPRemoveSignedDataMsg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptSIPRemoveSignedDataMsg 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptSIPVerifyIndirectData 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetHashParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptSetHashParam 0x{:x} 0x{:x}", arg1, arg2);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetKeyIdentifierProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSetKeyIdentifierProperty arg1:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSetKeyParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptSetKeyParam 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetOIDFunctionValue" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSetOIDFunctionValue 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProvParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptSetProvParam 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProvider" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptSetProvider arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSetProviderEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptSetProviderEx arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSignAndEncodeCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSignAndEncodeCertificate arg1:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptSignAndEncryptMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSignAndEncryptMessage 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSignCertificate 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptSignMessage 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSignMessageWithKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSignMessageWithKey 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptStringToBinary" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgCertMgr" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgCertMgr arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgSelectCertificate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "crypt32!CryptUIDlgSelectCertificate arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgSelectCertificateFromStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptUIDlgSelectCertificateFromStore arg1:0x{:x}",
                arg1
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgViewCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptUIDlgViewCertificate arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgViewContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptUIDlgViewContext arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIDlgViewSignerInfo" => {
            let psi = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptUIDlgViewSignerInfo pSignerInfo:0x{:x}",
                psi
            );
            if psi != 0 && emu.maps.is_mapped(psi) {
                let _ = emu.maps.read_dword(psi).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptUIWizDigitalSign" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptUIWizDigitalSign arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptUIWizExport" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptUIWizExport arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptUIWizImport arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptUIWizImport");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "crypt32!CryptUnprotectMemory 0x{:x} 0x{:x} 0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptUpdateProtectedState 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyCertificateSignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignature 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyCertificateSignatureEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignatureEx 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyDetachedMessageHash" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageHash 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyDetachedMessageSignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageSignature 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageHash" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageHash 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageSignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageSignature 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifyMessageSignatureWithKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageSignatureWithKey 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptVerifySignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptVerifySignature arg1:0x{:x}", arg1);
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlAddObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptXmlAddObject arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptXmlAddObject");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!CryptXmlDigestReference arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlEncode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(emu, "crypt32!CryptXmlEncode arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetAlgorithmInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptXmlGetAlgorithmInfo arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetDocContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptXmlGetDocContext arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetReference" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptXmlGetReference arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetSignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptXmlGetSignature arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetStatus" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!CryptXmlGetStatus arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlGetTransforms" => {
            let pp_transforms = emu.regs().rcx;
            log_red!(
                emu,
                "crypt32!CryptXmlGetTransforms ppTransforms:0x{:x}",
                pp_transforms
            );
            if pp_transforms != 0 {
                emu.maps.write_qword(pp_transforms, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CryptXmlImportPublicKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptXmlImportPublicKey arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptXmlImportPublicKey");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
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
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptXmlSetHMACSecret arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlSign" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(emu, "crypt32!CryptXmlSign arg1:0x{:x}", arg1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "CryptXmlVerifySignature" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(emu, "crypt32!CryptXmlVerifySignature arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {}
        }
        "PFXExportCertStore" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(emu, "crypt32!PFXExportCertStore 0x{:x} 0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PFXExportCertStoreEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!PFXExportCertStoreEx 0x{:x} 0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {}
        }
        "PFXVerifyPassword" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "crypt32!PFXVerifyPassword 0x{:x} 0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
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
