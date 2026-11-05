use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
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
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCRLContextToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCRLLinkToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCRLLinkToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCTLContextToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCTLContextToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCTLLinkToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCTLLinkToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCertificateContextToStore" => {
            let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let add_disp = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let hndl = helper::handler_create("cert_context");
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "crypt32!CertAddCertificateContextToStore store:0x{:x} disp:{} => 0x{:x}",
                store,
                add_disp,
                hndl
            );
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCertificateLinkToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertAddCertificateLinkToStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEncodedCRLToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddEncodedCRLToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEncodedCTLToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddEncodedCTLToStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEncodedCertificateToStore" => {
            let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cert_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cert_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let add_disp = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let hndl = helper::handler_create("cert_encoded");
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "crypt32!CertAddEncodedCertificateToStore store:0x{:x} len:{}",
                store,
                cert_len
            );
            emu.regs_mut().rax = 1;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEnhancedKeyUsageIdentifier" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertAddEnhancedKeyUsageIdentifier ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddRefServerOcspResponse" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertAddRefServerOcspResponse ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddRefServerOcspResponseContext" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertAddRefServerOcspResponseContext ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddSerializedElementToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertAddSerializedElementToStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddStoreToCollection" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddStoreToCollection ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAlgIdToOID" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAlgIdToOID ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCloseServerOcspResponse" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertCloseServerOcspResponse ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareCertificate" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertCompareCertificate ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareCertificateName" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertCompareCertificateName ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareIntegerBlob" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertCompareIntegerBlob ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertComparePublicKeyInfo" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertComparePublicKeyInfo ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertControlStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertControlStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCreateCRLContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
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
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertCreateCTLEntryFromCertificateContextProperties"
            );
            emu.regs_mut().rax =
                helper::handler_create("CertCreateCTLEntryFromCertificateContextProperties");
        }
        "CertCreateCertificateChainEngine" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertCreateCertificateChainEngine arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateCertificateChainEngine");
        }
        "CertCreateCertificateContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
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
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertCreateContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateContext");
        }
        "CertCreateSelfSignCertificate" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertCreateSelfSignCertificate arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            emu.regs_mut().rax = helper::handler_create("CertCreateSelfSignCertificate");
        }
        "CertDeleteCRLFromStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertDeleteCRLFromStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertDeleteCTLFromStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertDeleteCTLFromStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertDeleteCertificateFromStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertDeleteCertificateFromStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertDuplicateCRLContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CertDuplicateCRLContext arg1:0x{:x}", arg1);
            emu.regs_mut().rax = helper::handler_create("CertDuplicateCRLContext");
        }
        "CertDuplicateCTLContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CertDuplicateCTLContext arg1:0x{:x}", arg1);
            emu.regs_mut().rax = helper::handler_create("CertDuplicateCTLContext");
        }
        "CertDuplicateCertificateChain" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertDuplicateCertificateChain arg1:0x{:x}",
                arg1
            );
            emu.regs_mut().rax = helper::handler_create("CertDuplicateCertificateChain");
        }
        "CertDuplicateCertificateContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertDuplicateCertificateContext arg1:0x{:x}",
                arg1
            );
            emu.regs_mut().rax = helper::handler_create("CertDuplicateCertificateContext");
        }
        "CertDuplicateStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CertDuplicateStore arg1:0x{:x}", arg1);
            emu.regs_mut().rax = helper::handler_create("CertDuplicateStore");
        }
        "CertEnumCRLContextProperties" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumCRLContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertEnumCRLsInStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertEnumCRLsInStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumCRLsInStore");
        }
        "CertEnumCTLContextProperties" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumCTLContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertEnumCTLsInStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertEnumCTLsInStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertEnumCTLsInStore");
        }
        "CertEnumCertificateContextProperties" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumCertificateContextProperties ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertEnumCertificatesInStore" => {
            let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let prev_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
                store,
                prev_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CertEnumPhysicalStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
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
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindAttribute arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertFindAttribute");
        }
        "CertFindCRLInStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindCRLInStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            emu.regs_mut().rax = helper::handler_create("CertFindCRLInStore");
        }
        "CertFindCTLInStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindCTLInStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            emu.regs_mut().rax = helper::handler_create("CertFindCTLInStore");
        }
        "CertFindCertificateInCRL" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindCertificateInCRL arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            emu.regs_mut().rax = helper::handler_create("CertFindCertificateInCRL");
        }
        "CertFindChainInStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
            let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindChainInStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            emu.regs_mut().rax = helper::handler_create("CertFindChainInStore");
        }
        "CertFindExtension" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindExtension arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertFindExtension");
        }
        "CertFindRDNAttr" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindRDNAttr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertFindRDNAttr");
        }
        "CertFindSubjectInCTL" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindSubjectInCTL arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertFindSubjectInCTL");
        }
        "CertFindSubjectInSortedCTL" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertFindSubjectInSortedCTL arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertFindSubjectInSortedCTL");
        }
        "CertFreeCRLContext" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertFreeCRLContext ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertFreeCTLContext" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertFreeCTLContext ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertFreeCertificateChain" => {
            let chain_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(chain_ctx);
            log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
        }
        "CertFreeCertificateChainEngine" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertFreeCertificateChainEngine ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertFreeCertificateChainList" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertFreeCertificateChainList ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetCRLContextProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertGetCRLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetCRLFromStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
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
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertGetCTLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetCertificateChain" => {
            let engine = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let time = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let add_store = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let chain_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let chain_ctx_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let hndl = helper::handler_create("cert_chain");
            if chain_ctx_ptr != 0 {
                emu.maps.write_dword(chain_ctx_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "crypt32!CertGetCertificateChain cert:0x{:x} => 0x{:x}",
                cert_ctx,
                hndl
            );
            emu.regs_mut().rax = 1;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "CertGetCertificateContextProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertGetCertificateContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetEnhancedKeyUsage" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetEnhancedKeyUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetIntendedKeyUsage" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetIntendedKeyUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetIssuerCertificateFromStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertGetIssuerCertificateFromStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CertGetIssuerCertificateFromStore");
        }
        "CertGetNameString" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetNameString ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetPublicKeyLength" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetPublicKeyLength ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetServerOcspResponseContext" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertGetServerOcspResponseContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertGetServerOcspResponseContext");
        }
        "CertGetStoreProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetStoreProperty ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertGetSubjectCertificateFromStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertGetSubjectCertificateFromStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertGetSubjectCertificateFromStore");
        }
        "CertGetValidUsages" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetValidUsages ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertIsRDNAttrsInCertificateName" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertIsRDNAttrsInCertificateName ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertIsStrongHashToSign" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertIsStrongHashToSign ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertIsValidCRLForCertificate" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertIsValidCRLForCertificate ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertModifyCertificatesToTrust" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertModifyCertificatesToTrust ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertNameToStr" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertNameToStr ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertOIDToAlgId" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertOIDToAlgId ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertOpenServerOcspResponse" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertOpenServerOcspResponse arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            emu.regs_mut().rax = helper::handler_create("CertOpenServerOcspResponse");
        }
        "CertOpenSystemStore" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CertOpenSystemStore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.regs_mut().rax = helper::handler_create("CertOpenSystemStore");
        }
        "CertRDNValueToStr" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertRDNValueToStr ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertRegisterPhysicalStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertRegisterPhysicalStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertRegisterSystemStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertRegisterSystemStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertRemoveEnhancedKeyUsageIdentifier" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertRemoveEnhancedKeyUsageIdentifier ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertRemoveStoreFromCollection" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertRemoveStoreFromCollection ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertRetrieveLogoOrBiometricInfo" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertRetrieveLogoOrBiometricInfo ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSaveStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSaveStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSelectCertificate" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSelectCertificate ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSelectCertificateChains" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSelectCertificateChains ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSerializeCRLStoreElement" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSerializeCRLStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSerializeCTLStoreElement" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSerializeCTLStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSerializeCertificateStoreElement" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSerializeCertificateStoreElement ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetCRLContextProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSetCRLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetCTLContextProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSetCTLContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetCertificateContextPropertiesFromCTLEntry" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextPropertiesFromCTLEntry ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetCertificateContextProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextProperty ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetEnhancedKeyUsage" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSetEnhancedKeyUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSetStoreProperty" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSetStoreProperty ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupClose" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupClose ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupEnd" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupEnd ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupFree" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupFree ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupGetBackupLogsW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvBackupGetBackupLogsW ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupGetDatabaseNamesW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvBackupGetDatabaseNamesW ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupGetDynamicFileListW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvBackupGetDynamicFileListW ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupOpenFileW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupOpenFileW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupPrepareW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupPrepareW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupRead" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupRead ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupTruncateLogs" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvBackupTruncateLogs ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvIsServerOnlineW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvIsServerOnlineW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreEnd" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvRestoreEnd ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreGetDatabaseLocationsW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreGetDatabaseLocationsW ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestorePrepareW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvRestorePrepareW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreRegisterComplete" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterComplete ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreRegisterThroughFile" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSrvRestoreRegisterThroughFile ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreRegisterW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvRestoreRegisterW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvServerControlW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvServerControlW ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertStrToName" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertStrToName ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertUnregisterPhysicalStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertUnregisterPhysicalStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertUnregisterSystemStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertUnregisterSystemStore ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyCRLRevocation" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertVerifyCRLRevocation ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyCRLTimeValidity" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertVerifyCRLTimeValidity ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyCTLUsage" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertVerifyCTLUsage ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyCertificateChainPolicy" => {
            let policy_oid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let chain_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let policy_para = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let policy_status = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if policy_status != 0 {
                emu.maps.write_dword(policy_status, 0); // cbSize
                emu.maps.write_dword(policy_status + 4, 0); // dwError = 0 (valid)
            }
            log_red!(
                emu,
                "crypt32!CertVerifyCertificateChainPolicy chain:0x{:x} => valid",
                chain_ctx
            );
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyRevocation" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertVerifyRevocation ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifySubjectCertificateContext" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertVerifySubjectCertificateContext ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyTimeValidity" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertVerifyTimeValidity ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertVerifyValidityNesting" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertVerifyValidityNesting ctx:0x{:x}",
                cert_ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptAcquireContext" => {
            let h_prov_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let container_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let provider_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let container = if container_ptr != 0 {
                emu.maps.read_string(container_ptr)
            } else {
                String::new()
            };
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptAcquireContext container: '{}' type: {} flags: 0x{:x}",
                container,
                prov_type,
                flags
            );
            let h_prov = helper::handler_create(&format!("cryptprov:{}", prov_type));
            if h_prov_ptr != 0 {
                emu.maps.write_dword(h_prov_ptr, h_prov as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptBinaryToString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptBinaryToString arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminAcquireContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminAcquireContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminAcquireContext2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminAcquireContext2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminAddCatalog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminAddCatalog arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminCalcHashFromFileHandle" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminCalcHashFromFileHandle2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminEnumCatalogFromHash" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminEnumCatalogFromHash arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminReleaseCatalogContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseCatalogContext arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminReleaseContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminReleaseContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminRemoveCatalog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminResolveCatalogPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminResolveCatalogPath arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATCDFClose arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumAttributesWithCDFTag" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumAttributesWithCDFTag arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumCatAttributes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumCatAttributes arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumMembersByCDFTagEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumMembersByCDFTagEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFOpen" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATCDFOpen arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCatalogInfoFromContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCatalogInfoFromContext arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATClose arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateAttr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATEnumerateAttr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateCatAttr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATEnumerateCatAttr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateMember" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATEnumerateMember arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATGetAttrInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATGetAttrInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATGetMemberInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATGetMemberInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATOpen" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATOpen arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptContextAddRef" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptContextAddRef arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCreateHash" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let h_key = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let h_hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
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
                emu.maps.write_dword(h_hash_ptr, h_hash as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptCreateKeyIdentifierFromCSP" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCreateKeyIdentifierFromCSP arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecodeMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDecodeMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecodeObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDecodeObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecrypt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDecrypt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecryptAndVerifyMessageSignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptDecryptAndVerifyMessageSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecryptMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDecryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDeriveKey" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let h_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let h_key_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptDeriveKey prov: 0x{:x} alg: 0x{:x} hash: 0x{:x}",
                h_prov,
                algid,
                h_hash
            );
            let h_key = helper::handler_create(&format!("derivedkey:0x{:x}", algid));
            if h_key_ptr != 0 {
                emu.maps.write_dword(h_key_ptr, h_key as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDestroyHash" => {
            let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CryptDestroyHash hash: 0x{:x}", h_hash);
            if h_hash != 0 {
                helper::handler_close(h_hash);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDestroyKey" => {
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CryptDestroyKey key: 0x{:x}", h_key);
            if h_key != 0 {
                helper::handler_close(h_key);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptDuplicateHash" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDuplicateHash arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptDuplicateKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDuplicateKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncodeObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEncodeObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncodeObjectEx" => {
            let encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let struct_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let struct_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
                encoding,
                struct_type,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncrypt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEncrypt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncryptMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEncryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumKeyIdentifierProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptEnumKeyIdentifierProperties arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEnumOIDFunction arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumOIDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEnumOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumProviderTypes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEnumProviderTypes arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumProviders" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEnumProviders arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPKCS8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportPKCS8 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPKCS8Ex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportPKCS8Ex arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPublicKeyInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPublicKeyInfoEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportPublicKeyInfoEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPublicKeyInfoFromBCryptKeyHandle" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfoFromBCryptKeyHandle arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFindCertificateKeyProvInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptFindCertificateKeyProvInfo arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFindLocalizedName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFindLocalizedName arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFindOIDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFindOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFormatObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFormatObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFreeOIDFunctionAddress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFreeOIDFunctionAddress arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGenKey" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let h_key_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
                h_prov,
                algid,
                flags
            );
            let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
            if h_key_ptr != 0 {
                emu.maps.write_dword(h_key_ptr, h_key as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptGenRandom" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
                h_prov,
                len,
                buffer
            );
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
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetDefaultOIDDllList arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetDefaultOIDFunctionAddress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptGetDefaultOIDFunctionAddress arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetDefaultProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetDefaultProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetHashParam" => {
            let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptGetHashParam hash: 0x{:x} param: {} data: 0x{:x}",
                h_hash,
                param,
                data_ptr
            );
            match param {
                1 => {
                    // HP_ALGID
                    if data_ptr != 0 {
                        emu.maps.write_dword(data_ptr, 0x8004);
                    }
                    if data_len_ptr != 0 {
                        emu.maps.write_dword(data_len_ptr, 4);
                    }
                    emu.regs_mut().rax = 1;
                }
                2 => {
                    // HP_HASHSIZE
                    if data_ptr != 0 {
                        emu.maps.write_dword(data_ptr, 20);
                    }
                    if data_len_ptr != 0 {
                        emu.maps.write_dword(data_len_ptr, 4);
                    }
                    emu.regs_mut().rax = 1;
                }
                3 => {
                    // HP_HASHVAL
                    let hash_size: u32 = 20;
                    if data_ptr != 0 {
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
                    emu.regs_mut().rax = 0;
                }
            }
        }
        "CryptGetKeyIdentifierProperty" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptGetKeyIdentifierProperty arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetKeyParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetKeyParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetMessageCertificates" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetMessageCertificates arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetMessageSignerCount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetMessageSignerCount arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetOIDFunctionAddress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetOIDFunctionAddress arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetOIDFunctionValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetOIDFunctionValue arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetObjectUrl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetObjectUrl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetProvParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetProvParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptGetUserKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetUserKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashCertificate" => {
            let alg_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let cert_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cert_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let hash_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu,
                alg_id,
                cert_ptr,
                cert_len,
                hash_ptr,
                hash_len_ptr,
            );
        }
        "CryptHashCertificate2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashCertificate2 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashPublicKeyInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashSessionKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashSessionKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashToBeSigned" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashToBeSigned arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportKey" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let h_pub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let h_key_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptImportKey prov: 0x{:x} data: 0x{:x} len: {} pubkey: 0x{:x}",
                h_prov,
                data_ptr,
                data_len,
                h_pub_key
            );
            let h_key = helper::handler_create("importedkey");
            if h_key_ptr != 0 {
                emu.maps.write_dword(h_key_ptr, h_key as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptImportPKCS8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPKCS8 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfoEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfoEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfoEx2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfoEx2 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptInitOIDFunctionSet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptInitOIDFunctionSet arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptInstallDefaultContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptInstallDefaultContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptInstallOIDFunctionAddress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptInstallOIDFunctionAddress arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemAlloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemAlloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemFree arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemRealloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemRealloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgCalculateEncodedLength" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptMsgCalculateEncodedLength arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgClose" => {
            let msg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(msg);
            log_red!(emu, "crypt32!CryptMsgClose 0x{:x}", msg);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgControl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgCountersign" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgCountersign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgCountersignEncoded" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgCountersignEncoded arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgDuplicate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgDuplicate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgEncodeAndSignCTL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgEncodeAndSignCTL arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgGetAndVerifySigner" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgGetAndVerifySigner arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgGetParam" => {
            let hmsg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let param_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pv_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pcb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptMsgOpenToDecode" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptMsgOpenToDecode arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CryptMsgOpenToDecode");
        }
        "CryptMsgOpenToEncode" => {
            let esp = emu.regs().get_esp();
            let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
            let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
            let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptMsgOpenToEncode arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            emu.regs_mut().rax = helper::handler_create("CryptMsgOpenToEncode");
        }
        "CryptMsgSignCTL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgSignCTL arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgUpdate" => {
            let hmsg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let f_final = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptMsgVerifyCountersignatureEncoded" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncoded arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgVerifyCountersignatureEncodedEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptMsgVerifyCountersignatureEncodedEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptProtectData" => {
            let data_in = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let entropy = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let _prompt = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let data_out = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let desc = if desc_ptr != 0 {
                emu.maps.read_wide_string(desc_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "crypt32!CryptProtectData desc:'{}' flags:0x{:x}",
                desc,
                flags
            );
            if data_out != 0 && data_in != 0 {
                let in_size = emu.maps.read_dword(data_in).unwrap_or(0);
                let in_ptr = emu.maps.read_dword(data_in + 4).unwrap_or(0) as u64;
                if in_size > 0 && in_ptr != 0 {
                    let alloc = emu.maps.alloc(in_size as u64);
                    if let Some(buf) = alloc {
                        for i in 0..in_size as u64 {
                            let b = emu.maps.read_byte(in_ptr + i).unwrap_or(0);
                            emu.maps.write_byte(buf + i, b);
                        }
                        emu.maps.write_dword(data_out, in_size);
                        emu.maps.write_dword(data_out + 4, buf as u32);
                    }
                }
            }
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptProtectMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptProtectMemory arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptQueryObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptQueryObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptRegisterDefaultOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptRegisterDefaultOIDFunction arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptRegisterOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptRegisterOIDFunction arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptRegisterOIDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptRegisterOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptReleaseContext" => {
            let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "crypt32!CryptReleaseContext prov: 0x{:x}", h_prov);
            if h_prov != 0 {
                helper::handler_close(h_prov);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptRetrieveObjectByUrl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptRetrieveObjectByUrl arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPAddProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPAddProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPCreateIndirectData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPCreateIndirectData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPGetCaps" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPGetCaps arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPGetSignedDataMsg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPGetSignedDataMsg arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPLoad" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPLoad arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPPutSignedDataMsg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPPutSignedDataMsg arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRemoveProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPRemoveProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRemoveSignedDataMsg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPRemoveSignedDataMsg arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRetrieveSubjectGuid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPRetrieveSubjectGuid arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRetrieveSubjectGuidForCatalogFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPRetrieveSubjectGuidForCatalogFile arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPVerifyIndirectData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPVerifyIndirectData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetHashParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetHashParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetKeyIdentifierProperty" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSetKeyIdentifierProperty arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetKeyParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetKeyParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetOIDFunctionValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetOIDFunctionValue arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetProvParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetProvParam arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetProvider arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSetProviderEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSetProviderEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignAndEncodeCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSignAndEncodeCertificate arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignAndEncryptMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignAndEncryptMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignHash" => {
            let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!CryptSignHash hash: 0x{:x} spec: {}",
                h_hash,
                key_spec
            );
            if sig_ptr == 0 {
                if sig_len_ptr != 0 {
                    emu.maps.write_dword(sig_len_ptr, 128);
                }
            } else {
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
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignMessage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignMessageWithKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignMessageWithKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptStringToBinary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgCertMgr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgCertMgr arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgSelectCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgSelectCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgSelectCertificateFromStore" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptUIDlgSelectCertificateFromStore arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgViewCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgViewCertificate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgViewContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgViewContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgViewSignerInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgViewSignerInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIWizDigitalSign" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIWizDigitalSign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIWizExport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIWizExport arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIWizFreeDigitalSignContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptUIWizFreeDigitalSignContext arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIWizImport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIWizImport arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUninstallDefaultContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptUninstallDefaultContext arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnprotectData" => {
            let data_in = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let desc_ptr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let entropy = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let _prompt = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let data_out = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnprotectData flags:0x{:x}", flags);
            if data_out != 0 && data_in != 0 {
                let in_size = emu.maps.read_dword(data_in).unwrap_or(4);
                let in_ptr = emu.maps.read_dword(data_in + 8).unwrap_or(0);
                emu.maps.write_dword(data_out, in_size);
                emu.maps.write_dword(data_out + 8, in_ptr);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnprotectMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnprotectMemory arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnregisterDefaultOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptUnregisterDefaultOIDFunction arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnregisterOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnregisterOIDFunction arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnregisterOIDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnregisterOIDInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUpdateProtectedState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUpdateProtectedState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyCertificateSignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyCertificateSignatureEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignatureEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyDetachedMessageHash" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageHash arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyDetachedMessageSignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageSignature arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyMessageHash" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptVerifyMessageHash arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyMessageSignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptVerifyMessageSignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyMessageSignatureWithKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageSignatureWithKey arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifySignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptVerifySignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlAddObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlAddObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlClose arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlCreateReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlCreateReference arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlDigestReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlDigestReference arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlEncode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlEncode arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetAlgorithmInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetAlgorithmInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetDocContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetDocContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetReference arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetSignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetSignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetStatus arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlGetTransforms" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlGetTransforms arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlImportPublicKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlImportPublicKey arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlOpenToDecode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlOpenToDecode arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlOpenToEncode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlOpenToEncode arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlSetHMACSecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlSetHMACSecret arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlSign" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlSign arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlVerifySignature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlVerifySignature arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PFXExportCertStore" => {
            let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXExportCertStore blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PFXExportCertStoreEx" => {
            let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXExportCertStoreEx blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PFXImportCertStore" => {
            let pfx_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _password_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(
                emu,
                "crypt32!PFXImportCertStore pfx: 0x{:x} flags: 0x{:x}",
                pfx_ptr,
                flags
            );
            emu.regs_mut().rax = helper::handler_create("certstore:pfx");
        }
        "PFXIsPFXBlob" => {
            let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXIsPFXBlob blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PFXVerifyPassword" => {
            let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXVerifyPassword blob:0x{:x}", blob);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
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
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!Pki_InitializeCriticalSection flags: {:x} addr: 0x{:x}",
        flags,
        addr
    );

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    let string = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let num_chars = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let inout_sz = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_a(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
}

fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let struct_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let decode_para = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let pv_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let pcb_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);

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
        emu.maps.write_dword(pcb_struct_info as u64, 0);
    }

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hcrypt_prov = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let pv_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let name = emu.maps.read_string(subsystem as u64);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let name = emu.maps.read_wide_string(subsystem as u64);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store as u64);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let find_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let find_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let find_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let prev_cert = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
