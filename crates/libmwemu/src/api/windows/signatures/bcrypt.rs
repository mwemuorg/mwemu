use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "BCryptExportKey" => {
            signature!(NtStatus, 7, 7; Pointer(h_key), Pointer(h_export_key), WideString(psz_blob_type), Pointer(pb_output), Dword(cb_output), Pointer(pcb_result), Dword(dw_flags))
        }
        "BCryptFinalizeKeyPair" => signature!(NtStatus, 2, 2; Pointer(h_key), Dword(dw_flags)),
        "BCryptGenerateKeyPair" => {
            signature!(NtStatus, 4, 4; Pointer(h_algorithm), Pointer(ph_key), Dword(dw_length), Dword(dw_flags))
        }
        "BCryptImportKeyPair" => {
            signature!(NtStatus, 7, 7; Pointer(h_algorithm), Pointer(h_import_key), WideString(psz_blob_type), Pointer(ph_key), Pointer(pb_input), Dword(cb_input), Dword(dw_flags))
        }
        "BCryptSetProperty" => {
            signature!(NtStatus, 5, 5; Pointer(h_object), WideString(psz_property), Pointer(pb_input), Dword(cb_input), Dword(dw_flags))
        }
        "BCryptSignHash" => {
            signature!(NtStatus, 8, 8; Pointer(h_key), Pointer(p_padding_info), Pointer(pb_input), Dword(cb_input), Pointer(pb_output), Dword(cb_output), Pointer(pcb_result), Dword(dw_flags))
        }
        "BCryptVerifySignature" => {
            signature!(NtStatus, 7, 7; Pointer(h_key), Pointer(p_padding_info), Pointer(pb_hash), Dword(cb_hash), Pointer(pb_signature), Dword(cb_signature), Dword(dw_flags))
        }
        _ => None,
    }
}
