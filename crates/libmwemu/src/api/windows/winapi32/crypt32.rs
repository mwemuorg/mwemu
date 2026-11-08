use crate::api::windows::common::crypto_state;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PkiInitializeCriticalSection" => PkiInitializeCriticalSection(emu),
        "CryptStringToBinaryA" => CryptStringToBinaryA(emu),
        "CryptStringToBinaryW" => CryptStringToBinaryW(emu),
        "CryptBinaryToStringA" => CryptBinaryToStringA(emu),
        "CryptBinaryToStringW" => CryptBinaryToStringW(emu),
        "CryptDecodeObjectEx" => CryptDecodeObjectEx(emu),
        "CertOpenStore" => CertOpenStore(emu),
        "CertOpenSystemStoreA" => CertOpenSystemStoreA(emu),
        "CertOpenSystemStoreW" => CertOpenSystemStoreW(emu),
        "CertCloseStore" => CertCloseStore(emu),
        "CertFindCertificateInStore" => CertFindCertificateInStore(emu),
        "CertFreeCertificateContext" => CertFreeCertificateContext(emu),
        "CertAddCRLContextToStore" => {
            let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_crl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_add_disposition =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pp_store_ctx = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCRLContextToStore 0x{:x}", h_cert_store);
            let handle = helper::handler_create("CertAddCRLContextToStore");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCRLLinkToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCRLLinkToStore ctx:0x{:x}", cert_ctx);
            let handle = helper::handler_create("CertAddCRLLinkToStore");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCTLContextToStore" => {
            let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_ctl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_add_disposition =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pp_store_ctx = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCTLContextToStore 0x{:x}", h_cert_store);
            let handle = helper::handler_create("CertAddCTLContextToStore");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CertAddCTLLinkToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddCTLLinkToStore ctx:0x{:x}", cert_ctx);
            let handle = helper::handler_create("CertAddCTLLinkToStore");
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
            let handle = helper::handler_create("CertAddCertificateLinkToStore");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEncodedCRLToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddEncodedCRLToStore ctx:0x{:x}", cert_ctx);
            let handle = helper::handler_create("CertAddEncodedCRLToStore");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddEncodedCTLToStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddEncodedCTLToStore ctx:0x{:x}", cert_ctx);
            let handle = helper::handler_create("CertAddEncodedCTLToStore");
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
            let handle = helper::handler_create("CertAddEnhancedKeyUsageIdentifier");
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
            let handle = helper::handler_create("CertAddRefServerOcspResponse");
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
            let handle = helper::handler_create("CertAddRefServerOcspResponseContext");
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
            let handle = helper::handler_create("CertAddSerializedElementToStore");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAddStoreToCollection" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAddStoreToCollection ctx:0x{:x}", cert_ctx);
            let handle = helper::handler_create("CertAddStoreToCollection");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertAlgIdToOID" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertAlgIdToOID ctx:0x{:x}", cert_ctx);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareCertificate" => {
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_cert_id1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_cert_id2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertCompareCertificate dw_cert_encoding:0x{:x} p_cert_id1:0x{:x} p_cert_id2:0x{:x}",
                dw_cert_encoding,
                p_cert_id1,
                p_cert_id2
            );
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareCertificateName" => {
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_name1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_name2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertCompareCertificateName dw_cert_encoding:0x{:x} p_name1:0x{:x} p_name2:0x{:x}",
                dw_cert_encoding,
                p_name1,
                p_name2
            );
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CertCompareIntegerBlob" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertCompareIntegerBlob ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertComparePublicKeyInfo" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertComparePublicKeyInfo ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertControlStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertControlStore ctx:0x{:x}", cert_ctx);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

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
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_cert_encoded = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_cert_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertCreateCertificateContext dw_cert_encoding:0x{:x} pb_cert_encoded:0x{:x} cb_cert_encoded:0x{:x}",
                dw_cert_encoding,
                pb_cert_encoded,
                cb_cert_encoded
            );
            let hndl = helper::handler_create("CertCreateCertificateContext");
            emu.regs_mut().rax = hndl;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertDeleteCTLFromStore" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertDeleteCTLFromStore ctx:0x{:x}", cert_ctx);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
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
            let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_find_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_find_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pv_find_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p_prev_crl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertFindCRLInStore 0x{:x}", h_cert_store);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CertFindCTLInStore" => {
            let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_msg_and_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_find_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_find_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pv_find_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p_prev_ctl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertFindCTLInStore 0x{:x}", h_cert_store);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertFreeCTLContext" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertFreeCTLContext ctx:0x{:x}", cert_ctx);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let chain_ctx_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
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
            let p_cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_prop_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pv_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pcb_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertGetCertificateContextProperty 0x{:x}",
                p_cert_ctx
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CertGetEnhancedKeyUsage" => {
            let p_cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_usage = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pcb_usage = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertGetEnhancedKeyUsage 0x{:x}", p_cert_ctx);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
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
            let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_subject_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_prev_issuer_ctx =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pdw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertGetIssuerCertificateFromStore 0x{:x}",
                h_cert_store
            );
            let hndl = helper::handler_create("CertGetIssuerCertificateFromStore");
            emu.regs_mut().rax = hndl;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertOIDToAlgId" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertOIDToAlgId ctx:0x{:x}", cert_ctx);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let p_cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_prop_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pv_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CertSetCertificateContextProperty 0x{:x}",
                p_cert_ctx
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupEnd" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupEnd ctx:0x{:x}", cert_ctx);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupFree" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupFree ctx:0x{:x}", cert_ctx);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let handle = helper::handler_create("CertSrvBackupOpenFileW");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvBackupPrepareW" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvBackupPrepareW ctx:0x{:x}", cert_ctx);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertSrvRestoreEnd" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertSrvRestoreEnd ctx:0x{:x}", cert_ctx);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CertStrToName" => {
            let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CertStrToName ctx:0x{:x}", cert_ctx);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
        "CryptBinaryToString" => CryptBinaryToStringA(emu),
        "CryptCATAdminAcquireContext" => {
            let ph_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pg_subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminAcquireContext ph_cat_admin:0x{:x} pg_subsystem:0x{:x} dw_flags:0x{:x}",
                ph_cat_admin,
                pg_subsystem,
                dw_flags
            );
            let hndl = helper::handler_create("CryptCATAdminAcquireContext");
            if ph_cat_admin != 0 {
                emu.maps.write_dword(ph_cat_admin, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminAcquireContext2" => {
            let ph_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pg_subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pwsz_hash_alg = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_strong_hash_policy =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminAcquireContext2 0x{:x}",
                ph_cat_admin
            );
            let hndl = helper::handler_create("CryptCATAdminAcquireContext2");
            if ph_cat_admin != 0 {
                emu.maps.write_dword(ph_cat_admin, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminAddCatalog" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pwsz_catalog_file =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pwsz_select_base_name =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminAddCatalog 0x{:x}", h_cat_admin);
            let hndl = helper::handler_create("CryptCATAdminAddCatalog");
            emu.regs_mut().rax = hndl;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminCalcHashFromFileHandle" => {
            let h_file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pcb_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle 0x{:x}",
                h_file
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminCalcHashFromFileHandle2" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let h_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pcb_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_hash = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminCalcHashFromFileHandle2 0x{:x}",
                h_cat_admin
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminEnumCatalogFromHash" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminEnumCatalogFromHash 0x{:x}",
                h_cat_admin
            );
            let hndl = helper::handler_create("CryptCATAdminEnumCatalogFromHash");
            emu.regs_mut().rax = hndl;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminReleaseCatalogContext" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let h_cat_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseCatalogContext h_cat_admin:0x{:x} h_cat_info:0x{:x} dw_flags:0x{:x}",
                h_cat_admin,
                h_cat_info,
                dw_flags
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminReleaseContext" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminReleaseContext h_cat_admin:0x{:x} dw_flags:0x{:x}",
                h_cat_admin,
                dw_flags
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminRemoveCatalog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATAdminResolveCatalogPath" => {
            let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pwsz_catalog_file =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_resolve_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATAdminResolveCatalogPath h_cat_admin:0x{:x} pwsz_catalog_file:0x{:x} p_resolve_info:0x{:x}",
                h_cat_admin,
                pwsz_catalog_file,
                p_resolve_info
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATCDFClose arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumAttributesWithCDFTag" => {
            let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pwsz_member_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_member = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pfn_parse_error =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumAttributesWithCDFTag 0x{:x}",
                p_cdf
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumCatAttributes" => {
            let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumCatAttributes p_cdf:0x{:x} p_prev_attr:0x{:x} pfn_parse_error:0x{:x}",
                p_cdf,
                p_prev_attr,
                pfn_parse_error
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFEnumMembersByCDFTagEx" => {
            let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pwsz_prev_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pp_member = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let f_continue_on_error =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFEnumMembersByCDFTagEx 0x{:x}",
                p_cdf
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCDFOpen" => {
            let pwsz_file_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCDFOpen pwsz_file_path:0x{:x} pfn_parse_error:0x{:x}",
                pwsz_file_path,
                pfn_parse_error
            );
            let hndl = helper::handler_create("CryptCATCDFOpen");
            emu.regs_mut().rax = hndl;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATCatalogInfoFromContext" => {
            let h_cat_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_cat_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATCatalogInfoFromContext h_cat_info:0x{:x} p_cat_info:0x{:x} dw_flags:0x{:x}",
                h_cat_info,
                p_cat_info,
                dw_flags
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATClose arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateAttr" => {
            let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_cat_member = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATEnumerateAttr h_catalog:0x{:x} p_cat_member:0x{:x} p_prev_attr:0x{:x}",
                h_catalog,
                p_cat_member,
                p_prev_attr
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateCatAttr" => {
            let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATEnumerateCatAttr h_catalog:0x{:x} p_prev_attr:0x{:x}",
                h_catalog,
                p_prev_attr
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATEnumerateMember" => {
            let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_prev_member = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATEnumerateMember h_catalog:0x{:x} p_prev_member:0x{:x}",
                h_catalog,
                p_prev_member
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATGetAttrInfo" => {
            let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_cat_member = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pwsz_ref_tag = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATGetAttrInfo h_catalog:0x{:x} p_cat_member:0x{:x} pwsz_ref_tag:0x{:x}",
                h_catalog,
                p_cat_member,
                pwsz_ref_tag
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATGetMemberInfo" => {
            let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pwsz_ref_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_cat_member = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptCATGetMemberInfo h_catalog:0x{:x} pwsz_ref_tag:0x{:x} p_cat_member:0x{:x}",
                h_catalog,
                pwsz_ref_tag,
                p_cat_member
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptCATOpen" => {
            let pwsz_file_name = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let fdw_open_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let h_prov = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_public_version =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_encoded_msg_type =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptCATOpen 0x{:x}", pwsz_file_name);
            let hndl = helper::handler_create("CryptCATOpen");
            emu.regs_mut().rax = hndl;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptContextAddRef" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptContextAddRef arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptContextAddRef");
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
            crypto_state::create_hash(h_hash, algid);
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
            let handle = helper::handler_create("CryptCreateKeyIdentifierFromCSP");
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
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lpsz_struct_type =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pv_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let pcb_struct_info =
                emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptDecodeObject 0x{:x}", dw_cert_encoding);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecrypt" => {
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let h_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let is_final = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pdw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptDecrypt key:0x{:x} hash:0x{:x} final:{} flags:0x{:x} data:0x{:x}",
                h_key,
                h_hash,
                is_final,
                dw_flags,
                pb_data
            );
            if pb_data != 0 && pdw_data_len != 0 {
                let data_len = emu.maps.read_dword(pdw_data_len).unwrap_or(0) as usize;
                let len = data_len.min(0x100_0000);
                if len > 0 {
                    let mut data = emu.maps.read_bytes(pb_data, len).to_vec();
                    crypto_state::decrypt_data(h_key, &mut data);
                    emu.maps.write_bytes(pb_data, &data);
                }
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecryptAndVerifyMessageSignature" => {
            let p_decrypt_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_verify_para = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_encrypted_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let cb_encrypted_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let ppb_decrypted = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptDecryptAndVerifyMessageSignature 0x{:x}",
                p_decrypt_para
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CryptDecryptMessage" => {
            let p_decrypt_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_encrypted_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_encrypted_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptDecryptMessage p_decrypt_para:0x{:x} pb_encrypted_blob:0x{:x} cb_encrypted_blob:0x{:x}",
                p_decrypt_para,
                pb_encrypted_blob,
                cb_encrypted_blob
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
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
            crypto_state::derive_key_from_hash(h_key, h_hash, algid);
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
                crypto_state::destroy_hash(h_hash);
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
                crypto_state::destroy_key(h_key);
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
            let handle = helper::handler_create("CryptDuplicateKey");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncodeObject" => {
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lpsz_struct_type =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pv_struct_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pcb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEncodeObject 0x{:x}", dw_cert_encoding);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncodeObjectEx" => {
            let encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let struct_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let struct_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p_encode_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pv_encoded = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let pcb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
                encoding,
                struct_type,
                flags
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncrypt" => {
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let h_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let is_final = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pdw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let dw_buf_len = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptEncrypt key:0x{:x} hash:0x{:x} final:{} flags:0x{:x} data:0x{:x} buf:{}",
                h_key,
                h_hash,
                is_final,
                dw_flags,
                pb_data,
                dw_buf_len
            );
            if pb_data != 0 && pdw_data_len != 0 {
                let data_len = emu.maps.read_dword(pdw_data_len).unwrap_or(0) as usize;
                let len = data_len.min(0x100_0000);
                if len > 0 {
                    let mut data = emu.maps.read_bytes(pb_data, len).to_vec();
                    crypto_state::encrypt_data(h_key, &mut data);
                    emu.maps.write_bytes(pb_data, &data);
                }
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEncryptMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptEncryptMessage arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let h_exp_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_blob_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pdw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptExportKey key:0x{:x} exp:0x{:x} type:{} flags:0x{:x}",
                h_key,
                h_exp_key,
                dw_blob_type,
                dw_flags
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6u32 {
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
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pcb_info = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptExportPublicKeyInfo 0x{:x}", h_crypt_prov);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportPublicKeyInfoEx" => {
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let psz_public_key_obj_id =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pv_aux_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptExportPublicKeyInfoEx 0x{:x}",
                h_crypt_prov
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7 {
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
            let dw_key_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pv_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_group_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptFindOIDInfo dw_key_type:0x{:x} pv_key:0x{:x} dw_group_id:0x{:x}",
                dw_key_type,
                pv_key,
                dw_group_id
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptFormatObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFormatObject arg1:0x{:x}", arg1);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptFreeOIDFunctionAddress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptFreeOIDFunctionAddress arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptFreeOIDFunctionAddress");
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
            let key_bits = flags >> 16;
            crypto_state::create_key(h_key, algid, key_bits);
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let handle = helper::handler_create("CryptGetDefaultOIDFunctionAddress");
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
            if let Some(result) = crypto_state::get_hash_param(h_hash, param) {
                if data_ptr != 0 {
                    emu.maps.write_bytes(data_ptr, &result);
                }
                if data_len_ptr != 0 {
                    emu.maps.write_dword(data_len_ptr, result.len() as u32);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
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
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptGetKeyParam key:0x{:x} param:{} data:0x{:x}",
                h_key,
                param,
                data_ptr
            );
            if let Some(result) = crypto_state::get_key_param(h_key, param) {
                if data_ptr != 0 {
                    emu.maps.write_bytes(data_ptr, &result);
                }
                if data_len_ptr != 0 {
                    emu.maps.write_dword(data_len_ptr, result.len() as u32);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..5u32 {
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
            let handle = helper::handler_create("CryptGetOIDFunctionAddress");
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
            let psz_url_oid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pv_para = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_url_array = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pcb_url_array = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p_url_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let pcb_url_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let pv_reserved = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptGetObjectUrl 0x{:x}", psz_url_oid);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..8 {
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
            let pwsz_hash_alg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pv_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pb_computed_hash =
                emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashCertificate2 0x{:x}", pwsz_hash_alg);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashData" => {
            let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptHashData hash:0x{:x} data:0x{:x} len:{} flags:0x{:x}",
                h_hash,
                pb_data,
                dw_data_len,
                dw_flags
            );
            if pb_data != 0 && dw_data_len > 0 && dw_data_len < 0x100000 {
                let data = emu.maps.read_bytes(pb_data, dw_data_len as usize).to_vec();
                crypto_state::hash_data(h_hash, &data);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashMessage" => {
            let p_hash_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let f_detached_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let c_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let rgb_pb_to_be_hashed =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let rgcb_to_be_hashed =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashMessage 0x{:x}", p_hash_para);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptHashPublicKeyInfo" => {
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pb_computed_hash =
                emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashPublicKeyInfo 0x{:x}", h_crypt_prov);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
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
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pb_computed_hash =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptHashToBeSigned 0x{:x}", h_crypt_prov);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
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
            crypto_state::create_key(h_key, 0x6801, data_len * 8);
            if h_key_ptr != 0 {
                emu.maps.write_dword(h_key_ptr, h_key as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptImportPKCS8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPKCS8 arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptImportPKCS8");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfo" => {
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptImportPublicKeyInfo 0x{:x}", h_crypt_prov);
            let hndl = helper::handler_create("CryptImportPublicKeyInfo");
            if ph_key != 0 {
                emu.maps.write_dword(ph_key, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfoEx" => {
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let ai_key_alg = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pv_aux_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptImportPublicKeyInfoEx 0x{:x}",
                h_crypt_prov
            );
            let handle = helper::handler_create("CryptImportPublicKeyInfoEx");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CryptImportPublicKeyInfoEx2" => {
            let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pv_aux_info = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptImportPublicKeyInfoEx2 0x{:x}",
                dw_cert_encoding
            );
            let handle = helper::handler_create("CryptImportPublicKeyInfoEx2");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
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
            let handle = helper::handler_create("CryptInstallDefaultContext");
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
            let handle = helper::handler_create("CryptInstallOIDFunctionAddress");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemAlloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemAlloc arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptMemAlloc");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemFree arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMemRealloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMemRealloc arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h_crypt_msg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_ctrl_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pv_ctrl_para = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgControl 0x{:x}", h_crypt_msg);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..4 {
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
            let handle = helper::handler_create("CryptMsgDuplicate");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgEncodeAndSignCTL" => {
            let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_ctl_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_sign_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptMsgEncodeAndSignCTL 0x{:x}",
                dw_msg_encoding
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
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
            let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_msg_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p_recipient_info =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgOpenToDecode 0x{:x}", dw_msg_encoding);
            let hndl = helper::handler_create("CryptMsgOpenToDecode");
            emu.regs_mut().rax = hndl;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgOpenToEncode" => {
            let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_msg_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pv_msg_encode_info =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let psz_inner_content_obj_id =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgOpenToEncode 0x{:x}", dw_msg_encoding);
            let hndl = helper::handler_create("CryptMsgOpenToEncode");
            emu.regs_mut().rax = hndl;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptMsgSignCTL" => {
            let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_ctl_content = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_ctl_content = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_sign_info = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptMsgSignCTL 0x{:x}", dw_msg_encoding);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
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
            let data_in = arg(emu, 0) as u64;
            let desc_ptr = arg(emu, 1) as u64;
            let _entropy = arg(emu, 2);
            let _reserved = arg(emu, 3);
            let _prompt = arg(emu, 4);
            let flags = arg(emu, 5);
            let data_out = arg(emu, 6) as u64;
            let desc = if desc_ptr != 0 {
                emu.maps.read_wide_string(desc_ptr)
            } else {
                String::new()
            };
            let mut in_len = 0u32;
            let mut in_ptr = 0u64;
            if data_in != 0 {
                in_len = emu.maps.read_dword(data_in).unwrap_or(0);
                in_ptr = emu.maps.read_dword(data_in + 4).unwrap_or(0) as u64;
            }
            log_red!(
                emu,
                "crypt32!CryptProtectData desc:'{}' in_len:{} in_ptr:0x{:x} flags:0x{:x}",
                desc,
                in_len,
                in_ptr,
                flags
            );
            if in_ptr != 0 && in_len > 0 && data_out != 0 {
                let len = in_len.min(0x100_0000) as usize;
                let data = emu.maps.read_bytes(in_ptr, len).to_vec();
                let protected = crypto_state::dpapi_protect(&data);
                if let Some(alloc) = emu.maps.alloc(protected.len() as u64) {
                    emu.maps.write_bytes(alloc, &protected);
                    emu.maps.write_dword(data_out, protected.len() as u32);
                    emu.maps.write_dword(data_out + 4, alloc as u32);
                }
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
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
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
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
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptRetrieveObjectByUrl" => {
            let psz_url = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let psz_object_oid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_retrieve_flags =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_timeout = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let ppv_object = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let h_async_retrieve =
                emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p_credentials = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptRetrieveObjectByUrl 0x{:x}", psz_url);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPAddProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPAddProvider arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptSIPAddProvider");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPCreateIndirectData" => {
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pcb_indirect_data =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_indirect_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPCreateIndirectData p_subject_info:0x{:x} pcb_indirect_data:0x{:x} p_indirect_data:0x{:x}",
                p_subject_info,
                pcb_indirect_data,
                p_indirect_data
            );
            let handle = helper::handler_create("CryptSIPCreateIndirectData");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPGetCaps" => {
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_caps = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPGetCaps p_subject_info:0x{:x} p_caps:0x{:x}",
                p_subject_info,
                p_caps
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPGetSignedDataMsg" => {
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pdw_encoding_type =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pcb_signed_data_msg =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pb_signed_data_msg =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPGetSignedDataMsg 0x{:x}",
                p_subject_info
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPLoad" => {
            let pg_subject = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_sip_dispatch = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPLoad pg_subject:0x{:x} dw_flags:0x{:x} p_sip_dispatch:0x{:x}",
                pg_subject,
                dw_flags,
                p_sip_dispatch
            );
            let handle = helper::handler_create("CryptSIPLoad");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPPutSignedDataMsg" => {
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_encoding_type =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pdw_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_signed_data_msg =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pb_signed_data_msg =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPPutSignedDataMsg 0x{:x}",
                p_subject_info
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRemoveProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSIPRemoveProvider arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptSIPRemoveSignedDataMsg" => {
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPRemoveSignedDataMsg p_subject_info:0x{:x} dw_index:0x{:x} dw_reserved:0x{:x}",
                p_subject_info,
                dw_index,
                dw_reserved
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
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
            let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_indirect_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptSIPVerifyIndirectData p_subject_info:0x{:x} p_indirect_data:0x{:x} dw_reserved:0x{:x}",
                p_subject_info,
                p_indirect_data,
                dw_reserved
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..3 {
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
            let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "crypt32!CryptSetKeyParam key:0x{:x} param:{} data:0x{:x}",
                h_key,
                param,
                pb_data
            );
            if param == crypto_state::KP_ALGID
                && pb_data != 0
                && let Some(key_len_data) =
                    crypto_state::get_key_param(h_key, crypto_state::KP_KEYLEN)
            {
                let key_len = u32::from_le_bytes([
                    key_len_data[0],
                    key_len_data[1],
                    key_len_data[2],
                    key_len_data[3],
                ]) / 8;
                let data = emu.maps.read_bytes(pb_data, key_len as usize).to_vec();
                crypto_state::set_key_data(h_key, &data);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
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
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_encoded_to_be_signed =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let cb_encoded_to_be_signed =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p_sign_algorithm =
                emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let pv_hash_aux_info =
                emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let pb_signature = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignCertificate 0x{:x}", h_crypt_prov);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..8 {
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
            let p_sign_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let f_detached_signature =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let c_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let rgb_pb_to_be_signed =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let rgcb_to_be_signed =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignMessage 0x{:x}", p_sign_para);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptSignMessageWithKey" => {
            let p_sign_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let ppb_signed_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptSignMessageWithKey 0x{:x}", p_sign_para);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptStringToBinary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgCertMgr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgCertMgr arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgSelectCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgSelectCertificate arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIDlgViewContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIDlgViewContext arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUIWizImport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUIWizImport arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptUIWizImport");
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnprotectData" => {
            let data_in = arg(emu, 0) as u64;
            let pp_desc = arg(emu, 1) as u64;
            let _entropy = arg(emu, 2);
            let _reserved = arg(emu, 3);
            let _prompt = arg(emu, 4);
            let flags = arg(emu, 5);
            let data_out = arg(emu, 6) as u64;
            let mut in_len = 0u32;
            let mut in_ptr = 0u64;
            if data_in != 0 {
                in_len = emu.maps.read_dword(data_in).unwrap_or(0);
                in_ptr = emu.maps.read_dword(data_in + 4).unwrap_or(0) as u64;
            }
            log_red!(
                emu,
                "crypt32!CryptUnprotectData in_len:{} in_ptr:0x{:x} flags:0x{:x}",
                in_len,
                in_ptr,
                flags
            );
            if in_ptr != 0 && in_len > 0 && data_out != 0 {
                let len = in_len.min(0x100_0000) as usize;
                let data = emu.maps.read_bytes(in_ptr, len).to_vec();
                let unprotected = crypto_state::dpapi_unprotect(&data);
                if let Some(alloc) = emu.maps.alloc(unprotected.len() as u64) {
                    emu.maps.write_bytes(alloc, &unprotected);
                    emu.maps.write_dword(data_out, unprotected.len() as u32);
                    emu.maps.write_dword(data_out + 4, alloc as u32);
                }
            }
            if pp_desc != 0 {
                emu.maps.write_dword(pp_desc, 0);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
        }
        "CryptUnprotectMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnprotectMemory arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnregisterOIDFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnregisterOIDFunction arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptUnregisterOIDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptUnregisterOIDInfo arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
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
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignature 0x{:x}",
                h_crypt_prov
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyCertificateSignatureEx" => {
            let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_cert_encoding =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dw_subject_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pv_subject = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let dw_issuer_type = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pv_issuer = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyCertificateSignatureEx 0x{:x}",
                h_crypt_prov
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyDetachedMessageHash" => {
            let p_hash_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_detached_hash_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_detached_hash_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let c_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let rgb_pb_to_be_hashed =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageHash 0x{:x}",
                p_hash_para
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyDetachedMessageSignature" => {
            let p_verify_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_detached_sign_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_detached_sign_blob =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let c_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let rgb_pb_to_be_signed =
                emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyDetachedMessageSignature 0x{:x}",
                p_verify_para
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyMessageHash" => {
            let p_hash_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pb_hashed_blob = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cb_hashed_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let pb_to_be_hashed =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let pcb_to_be_hashed =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptVerifyMessageHash 0x{:x}", p_hash_para);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CryptVerifyMessageSignature" => {
            let p_verify_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pb_signed_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let cb_signed_blob = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let ppb_decoded = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let pcb_decoded = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let pp_signer_cert = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "crypt32!CryptVerifyMessageSignature 0x{:x}",
                p_verify_para
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..7 {
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
            let handle = helper::handler_create("CryptXmlAddObject");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlClose arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlCreateReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlCreateReference arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptXmlCreateReference");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlDigestReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlDigestReference arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
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
            let handle = helper::handler_create("CryptXmlImportPublicKey");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlOpenToDecode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlOpenToDecode arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptXmlOpenToDecode");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CryptXmlOpenToEncode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!CryptXmlOpenToEncode arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CryptXmlOpenToEncode");
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
            let h_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_pfx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sz_password = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXExportCertStore 0x{:x}", h_store);
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PFXVerifyPassword" => {
            let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "crypt32!PFXVerifyPassword blob:0x{:x}", blob);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let addr = arg(emu, 0);
    let flags = arg(emu, 1);

    log_red!(
        emu,
        "crypt32!PkiInitializeCriticalSection flags: 0x{:x} addr: 0x{:x}",
        flags,
        addr
    );

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    let string = arg(emu, 0) as u64;
    let num_chars = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let ptr = arg(emu, 3) as u64;
    let inout_sz = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_a(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
    stdcall_ret(emu, 7);
}

fn CryptStringToBinaryW(emu: &mut emu::Emu) {
    let string = arg(emu, 0) as u64;
    let num_chars = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let ptr = arg(emu, 3) as u64;
    let inout_sz = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_w(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
    stdcall_ret(emu, 7);
}

fn CryptBinaryToStringA(emu: &mut emu::Emu) {
    let data_ptr = arg(emu, 0) as u64;
    let data_len = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let str_ptr = arg(emu, 3) as u64;
    let str_len_ptr = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_a(
        emu,
        data_ptr,
        data_len,
        flags,
        str_ptr,
        str_len_ptr,
    );
    stdcall_ret(emu, 5);
}

fn CryptBinaryToStringW(emu: &mut emu::Emu) {
    let data_ptr = arg(emu, 0) as u64;
    let data_len = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let str_ptr = arg(emu, 3) as u64;
    let str_len_ptr = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_w(
        emu,
        data_ptr,
        data_len,
        flags,
        str_ptr,
        str_len_ptr,
    );
    stdcall_ret(emu, 5);
}

fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = arg(emu, 0);
    let struct_type = arg(emu, 1);
    let _encoded = arg(emu, 2);
    let cb_encoded = arg(emu, 3);
    let flags = arg(emu, 4);
    let _decode_para = arg(emu, 5);
    let _pv_struct_info = arg(emu, 6);
    let pcb_struct_info = arg(emu, 7) as u64;

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = arg(emu, 0);
    let encoding_type = arg(emu, 1);
    let _hcrypt_prov = arg(emu, 2);
    let flags = arg(emu, 3);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 5);
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = emu.maps.read_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreA subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let _hprov = arg(emu, 0);
    let subsystem = arg(emu, 1) as u64;

    let name = emu.maps.read_wide_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 2);
}

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = arg(emu, 0) as u64;
    let flags = arg(emu, 1);

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store);

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = arg(emu, 0);
    let _encoding_type = arg(emu, 1);
    let _find_flags = arg(emu, 2);
    let find_type = arg(emu, 3);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = arg(emu, 0);

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
