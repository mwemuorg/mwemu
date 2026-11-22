use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "CertCreateCertificateChainEngine" => {
            signature!(Win32Bool, 2, 2; Pointer(p_config), Pointer(ph_chain_engine))
        }
        "CertCreateCertificateContext" => {
            signature!(Generic, 3, 3; Dword(dw_cert_encoding_type), Pointer(pb_cert_encoded), Dword(cb_cert_encoded))
        }
        "CertCreateContext" => {
            signature!(Generic, 6, 6; Dword(dw_context_type), Dword(dw_encoding_type), Pointer(pb_encoded), Dword(cb_encoded), Dword(dw_flags), Pointer(p_create_para))
        }
        "CertCreateSelfSignCertificate" => {
            signature!(Generic, 8, 8; Pointer(h_crypt_prov_or_n_crypt_key), Pointer(p_subject_issuer_blob), Dword(dw_flags), Pointer(p_key_prov_info), Pointer(p_signature_algorithm), Pointer(p_start_time), Pointer(p_end_time), Pointer(p_extensions))
        }
        "CertDuplicateCRLContext" => signature!(Generic, 1, 1; Pointer(p_crl_context)),
        "CertDuplicateCTLContext" => signature!(Generic, 1, 1; Pointer(p_ctl_context)),
        "CertDuplicateCertificateChain" => signature!(Generic, 1, 1; Pointer(p_chain_context)),
        "CertDuplicateCertificateContext" => signature!(Generic, 1, 1; Pointer(p_cert_context)),
        "CertDuplicateStore" => signature!(Generic, 1, 1; Pointer(h_cert_store)),
        "CertFindAttribute" => {
            signature!(Generic, 3, 3; AnsiString(psz_obj_id), Dword(c_attr), Pointer(rg_attr))
        }
        "CertFindCertificateInCRL" => {
            signature!(Win32Bool, 5, 5; Pointer(p_cert), Pointer(p_crl_context), Dword(dw_flags), Pointer(pv_reserved), Pointer(pp_crl_entry))
        }
        "CertFindChainInStore" => {
            signature!(Generic, 6, 6; Pointer(h_cert_store), Dword(dw_cert_encoding_type), Dword(dw_find_flags), Dword(dw_find_type), Pointer(pv_find_para), Pointer(p_prev_chain_context))
        }
        "CertFindExtension" => {
            signature!(Generic, 3, 3; AnsiString(psz_obj_id), Dword(c_extensions), Pointer(rg_extensions))
        }
        "CertFindRDNAttr" => signature!(Generic, 2, 2; AnsiString(psz_obj_id), Pointer(p_name)),
        "CertFindSubjectInCTL" => {
            signature!(Generic, 5, 5; Dword(dw_encoding_type), Dword(dw_subject_type), Pointer(pv_subject), Pointer(p_ctl_context), Dword(dw_flags))
        }
        "CertFindSubjectInSortedCTL" => {
            signature!(Win32Bool, 5, 5; Pointer(p_subject_identifier), Pointer(p_ctl_context), Dword(dw_flags), Pointer(pv_reserved), Pointer(p_encoded_attributes))
        }
        "CertGetCRLContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_crl_context), Dword(dw_prop_id), Pointer(pv_data), Pointer(pcb_data))
        }
        "CertGetCTLContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_ctl_context), Dword(dw_prop_id), Pointer(pv_data), Pointer(pcb_data))
        }
        "CertGetCertificateContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_cert_context), Dword(dw_prop_id), Pointer(pv_data), Pointer(pcb_data))
        }
        "CertGetEnhancedKeyUsage" => {
            signature!(Win32Bool, 4, 4; Pointer(p_cert_context), Dword(dw_flags), Pointer(p_usage), Pointer(pcb_usage))
        }
        "CertGetIntendedKeyUsage" => {
            signature!(Win32Bool, 4, 4; Dword(dw_cert_encoding_type), Pointer(p_cert_info), Pointer(pb_key_usage), Dword(cb_key_usage))
        }
        "CertGetPublicKeyLength" => {
            signature!(Generic, 2, 2; Dword(dw_cert_encoding_type), Pointer(p_public_key))
        }
        "CertGetServerOcspResponseContext" => {
            signature!(Generic, 3, 3; Pointer(h_server_ocsp_response), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CertGetStoreProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Dword(dw_prop_id), Pointer(pv_data), Pointer(pcb_data))
        }
        "CertGetSubjectCertificateFromStore" => {
            signature!(Generic, 3, 3; Pointer(h_cert_store), Dword(dw_cert_encoding_type), Pointer(p_cert_id))
        }
        "CertGetValidUsages" => {
            signature!(Win32Bool, 5, 5; Dword(c_certs), Pointer(rgh_certs), Pointer(c_num_oi_ds), Pointer(rgh_oi_ds), Pointer(pcb_oi_ds))
        }
        "CertOpenServerOcspResponse" => {
            signature!(Generic, 3, 3; Pointer(p_chain_context), Dword(dw_flags), Pointer(p_open_para))
        }
        "CertSrvRestoreRegisterComplete" => {
            signature!(Hresult, 2, 2; Pointer(hbc), Dword(hr_restore_state))
        }
        "CertSrvRestoreRegisterThroughFile" => {
            signature!(Hresult, 8, 8; Pointer(hbc), Pointer(pwsz_check_point_file_path), Pointer(pwsz_log_path), Pointer(rgrstmap), Dword(crstmap), Pointer(pwsz_backup_log_path), Dword(gen_low), Dword(gen_high))
        }
        "CryptCATAdminRemoveCatalog" => {
            signature!(Win32Bool, 3, 3; Pointer(h_cat_admin), Pointer(pwsz_catalog_file), Dword(dw_flags))
        }
        "CryptCATAdminResolveCatalogPath" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cat_admin), WideString(pwsz_catalog_file), Pointer(ps_cat_info), Dword(dw_flags))
        }
        "CryptCATCDFClose" => signature!(Win32Bool, 1, 1; Pointer(p_cdf)),
        "CryptCATCDFEnumAttributesWithCDFTag" => {
            signature!(Generic, 5, 5; Pointer(pcdf), Pointer(pwszmembertag), Pointer(pmember), Pointer(pprevattr), Pointer(pfnparseerror))
        }
        "CryptCATCDFEnumCatAttributes" => {
            signature!(Generic, 3, 3; Pointer(p_cdf), Pointer(p_prev_attr), Pointer(pfn_parse_error))
        }
        "CryptCATCDFEnumMembersByCDFTagEx" => {
            signature!(Generic, 6, 6; Pointer(pcdf), Pointer(pwszprevcdftag), Pointer(pfnparseerror), Pointer(ppmember), Dword(fcontinueonerror), Pointer(pvreserved))
        }
        "CryptCATCDFOpen" => {
            signature!(Generic, 2, 2; WideString(pwsz_file_path), Pointer(pfn_parse_error))
        }
        "CryptCATCatalogInfoFromContext" => {
            signature!(Win32Bool, 3, 3; Pointer(h_cat_info), Pointer(ps_cat_info), Dword(dw_flags))
        }
        "CryptCATClose" => signature!(Win32Bool, 1, 1; Pointer(h_catalog)),
        "CryptCATEnumerateAttr" => {
            signature!(Generic, 3, 3; Pointer(h_catalog), Pointer(p_cat_member), Pointer(p_prev_attr))
        }
        "CryptCATEnumerateCatAttr" => {
            signature!(Generic, 2, 2; Pointer(h_catalog), Pointer(p_prev_attr))
        }
        "CryptCATEnumerateMember" => {
            signature!(Generic, 2, 2; Pointer(h_catalog), Pointer(p_prev_member))
        }
        "CryptCATGetAttrInfo" => {
            signature!(Generic, 3, 3; Pointer(h_catalog), Pointer(p_cat_member), WideString(pwsz_reference_tag))
        }
        "CryptCATGetMemberInfo" => {
            signature!(Generic, 2, 2; Pointer(h_catalog), WideString(pwsz_reference_tag))
        }
        "CryptCATOpen" => {
            signature!(Generic, 5, 5; WideString(pwsz_file_name), Dword(fdw_open_flags), Pointer(h_prov), Dword(dw_public_version), Dword(dw_encoding_type))
        }
        "CryptContextAddRef" => {
            signature!(Win32Bool, 3, 3; Pointer(h_prov), Pointer(pdw_reserved), Dword(dw_flags))
        }
        "CryptCreateKeyIdentifierFromCSP" => {
            signature!(Win32Bool, 8, 8; Dword(dw_cert_encoding_type), AnsiString(psz_pub_key_oid), Pointer(p_pub_key_struc), Dword(cb_pub_key_struc), Dword(dw_flags), Pointer(pv_reserved), Pointer(pb_hash), Pointer(pcb_hash))
        }
        "CryptDecodeMessage" => {
            signature!(Win32Bool, 13, 13; Dword(dw_msg_type_flags), Pointer(p_decrypt_para), Pointer(p_verify_para), Dword(dw_signer_index), Pointer(pb_encoded_blob), Dword(cb_encoded_blob), Dword(dw_prev_inner_content_type), Pointer(pdw_msg_type), Pointer(pdw_inner_content_type), Pointer(pb_decoded), Pointer(pcb_decoded), Pointer(pp_xchg_cert), Pointer(pp_signer_cert))
        }
        "CryptDuplicateHash" => {
            signature!(Win32Bool, 4, 4; Pointer(h_hash), Pointer(pdw_reserved), Dword(dw_flags), Pointer(ph_hash))
        }
        "CryptDuplicateKey" => {
            signature!(Win32Bool, 4, 4; Pointer(h_key), Pointer(pdw_reserved), Dword(dw_flags), Pointer(ph_key))
        }
        "CryptExportKey" => {
            signature!(Win32Bool, 6, 6; Pointer(h_key), Pointer(h_exp_key), Dword(dw_blob_type), Dword(dw_flags), Pointer(pb_data), Pointer(pdw_data_len))
        }
        "CryptExportPKCS8" => {
            signature!(Win32Bool, 7, 7; Pointer(h_crypt_prov), Dword(dw_key_spec), AnsiString(psz_private_key_obj_id), Dword(dw_flags), Pointer(pv_aux_info), Pointer(pb_private_key_blob), Pointer(pcb_private_key_blob))
        }
        "CryptExportPublicKeyInfoFromBCryptKeyHandle" => {
            signature!(Win32Bool, 7, 7; Pointer(h_b_crypt_key), Dword(dw_cert_encoding_type), AnsiString(psz_public_key_obj_id), Dword(dw_flags), Pointer(pv_aux_info), Pointer(p_info), Pointer(pcb_info))
        }
        "CryptFindCertificateKeyProvInfo" => {
            signature!(Win32Bool, 3, 3; Pointer(p_cert), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CryptFindLocalizedName" => signature!(Generic, 1, 1; WideString(pwsz_crypt_name)),
        "CryptFindOIDInfo" => {
            signature!(Generic, 3, 3; Dword(dw_key_type), Pointer(pv_key), Dword(dw_group_id))
        }
        "CryptFormatObject" => {
            signature!(Win32Bool, 9, 9; Dword(dw_cert_encoding_type), Dword(dw_format_type), Dword(dw_format_str_type), Pointer(p_format_struct), AnsiString(lpsz_struct_type), Pointer(pb_encoded), Dword(cb_encoded), Pointer(pb_format), Pointer(pcb_format))
        }
        "CryptFreeOIDFunctionAddress" => {
            signature!(Win32Bool, 2, 2; Pointer(h_func_addr), Dword(dw_flags))
        }
        "CryptGetDefaultOIDDllList" => {
            signature!(Win32Bool, 4, 4; Pointer(h_func_set), Dword(dw_encoding_type), Pointer(pwsz_dll_list), Pointer(pcch_dll_list))
        }
        "CryptGetDefaultOIDFunctionAddress" => {
            signature!(Win32Bool, 6, 6; Pointer(h_func_set), Dword(dw_encoding_type), WideString(pwsz_dll), Dword(dw_flags), Pointer(ppv_func_addr), Pointer(ph_func_addr))
        }
        "CryptGetKeyIdentifierProperty" => {
            signature!(Win32Bool, 7, 7; Pointer(p_key_identifier), Dword(dw_prop_id), Dword(dw_flags), WideString(pwsz_computer_name), Pointer(pv_reserved), Pointer(pv_data), Pointer(pcb_data))
        }
        "CryptGetMessageCertificates" => {
            signature!(Generic, 5, 5; Dword(dw_msg_and_cert_encoding_type), Pointer(h_crypt_prov), Dword(dw_flags), Pointer(pb_signed_blob), Dword(cb_signed_blob))
        }
        "CryptGetMessageSignerCount" => {
            signature!(Generic, 3, 3; Dword(dw_msg_encoding_type), Pointer(pb_signed_blob), Dword(cb_signed_blob))
        }
        "CryptGetOIDFunctionAddress" => {
            signature!(Win32Bool, 6, 6; Pointer(h_func_set), Dword(dw_encoding_type), AnsiString(psz_oid), Dword(dw_flags), Pointer(ppv_func_addr), Pointer(ph_func_addr))
        }
        "CryptGetOIDFunctionValue" => {
            signature!(Win32Bool, 7, 7; Dword(dw_encoding_type), AnsiString(psz_func_name), AnsiString(psz_oid), WideString(pwsz_value_name), Pointer(pdw_value_type), Pointer(pb_value_data), Pointer(pcb_value_data))
        }
        "CryptGetObjectUrl" => {
            signature!(Win32Bool, 8, 8; AnsiString(psz_url_oid), Pointer(pv_para), Dword(dw_flags), Pointer(p_url_array), Pointer(pcb_url_array), Pointer(p_url_info), Pointer(pcb_url_info), Pointer(pv_reserved))
        }
        "CryptGetProvParam" => {
            signature!(Win32Bool, 5, 5; Pointer(h_prov), Dword(dw_param), Pointer(pb_data), Pointer(pdw_data_len), Dword(dw_flags))
        }
        "CryptGetUserKey" => {
            signature!(Win32Bool, 3, 3; Pointer(h_prov), Dword(dw_key_spec), Pointer(ph_user_key))
        }
        "CryptHashSessionKey" => {
            signature!(Win32Bool, 3, 3; Pointer(h_hash), Pointer(h_key), Dword(dw_flags))
        }
        "CryptImportPKCS8" => signature!(Win32Bool, 4, 9),
        "CryptInitOIDFunctionSet" => {
            signature!(Generic, 2, 2; AnsiString(psz_func_name), Dword(dw_flags))
        }
        "CryptInstallDefaultContext" => {
            signature!(Win32Bool, 6, 6; Pointer(h_crypt_prov), Dword(dw_default_type), Pointer(pv_default_para), Dword(dw_flags), Pointer(pv_reserved), Pointer(ph_default_context))
        }
        "CryptInstallOIDFunctionAddress" => {
            signature!(Win32Bool, 6, 6; Pointer(h_module), Dword(dw_encoding_type), AnsiString(psz_func_name), Dword(c_func_entry), Pointer(rg_func_entry), Dword(dw_flags))
        }
        "CryptMsgCalculateEncodedLength" => {
            signature!(Generic, 6, 6; Dword(dw_msg_encoding_type), Dword(dw_flags), Dword(dw_msg_type), Pointer(pv_msg_encode_info), AnsiString(psz_inner_content_obj_id), Dword(cb_data))
        }
        "CryptMsgCountersign" => {
            signature!(Win32Bool, 4, 4; Pointer(h_crypt_msg), Dword(dw_index), Dword(c_countersigners), Pointer(rg_countersigners))
        }
        "CryptMsgCountersignEncoded" => {
            signature!(Win32Bool, 7, 7; Dword(dw_encoding_type), Pointer(pb_signer_info), Dword(cb_signer_info), Dword(c_countersigners), Pointer(rg_countersigners), Pointer(pb_countersignature), Pointer(pcb_countersignature))
        }
        "CryptMsgDuplicate" => signature!(Generic, 1, 1; Pointer(h_crypt_msg)),
        "CryptMsgGetAndVerifySigner" => {
            signature!(Win32Bool, 6, 6; Pointer(h_crypt_msg), Dword(c_signer_store), Pointer(rgh_signer_store), Dword(dw_flags), Pointer(pp_signer), Pointer(pdw_signer_index))
        }
        "CryptMsgOpenToDecode" => {
            signature!(Generic, 6, 6; Dword(dw_msg_encoding_type), Dword(dw_flags), Dword(dw_msg_type), Pointer(h_crypt_prov), Pointer(p_recipient_info), Pointer(p_stream_info))
        }
        "CryptMsgOpenToEncode" => {
            signature!(Generic, 6, 6; Dword(dw_msg_encoding_type), Dword(dw_flags), Dword(dw_msg_type), Pointer(pv_msg_encode_info), AnsiString(psz_inner_content_obj_id), Pointer(p_stream_info))
        }
        "CryptMsgVerifyCountersignatureEncoded" => {
            signature!(Win32Bool, 7, 7; Pointer(h_crypt_prov), Dword(dw_encoding_type), Pointer(pb_signer_info), Dword(cb_signer_info), Pointer(pb_signer_info_countersignature), Dword(cb_signer_info_countersignature), Pointer(pci_countersigner))
        }
        "CryptProtectMemory" => {
            signature!(Win32Bool, 3, 3; Pointer(p_data_in), Dword(cb_data_in), Dword(dw_flags))
        }
        "CryptQueryObject" => {
            signature!(Win32Bool, 11, 11; Dword(dw_object_type), Pointer(pv_object), Dword(dw_expected_content_type_flags), Dword(dw_expected_format_type_flags), Dword(dw_flags), Pointer(pdw_msg_and_cert_encoding_type), Pointer(pdw_content_type), Pointer(pdw_format_type), Pointer(ph_cert_store), Pointer(ph_msg), Pointer(ppv_context))
        }
        "CryptRegisterDefaultOIDFunction" => {
            signature!(Win32Bool, 4, 4; Dword(dw_encoding_type), AnsiString(psz_func_name), Dword(dw_index), WideString(pwsz_dll))
        }
        "CryptRegisterOIDFunction" => {
            signature!(Win32Bool, 5, 5; Dword(dw_encoding_type), AnsiString(psz_func_name), AnsiString(psz_oid), WideString(pwsz_dll), AnsiString(psz_override_func_name))
        }
        "CryptRegisterOIDInfo" => signature!(Win32Bool, 2, 2; Pointer(p_info), Dword(dw_flags)),
        "CryptSIPGetCaps" => signature!(Win32Bool, 2, 2; Pointer(p_subj_info), Pointer(p_caps)),
        "CryptSIPGetSignedDataMsg" => {
            signature!(Win32Bool, 5, 5; Pointer(p_subject_info), Pointer(pdw_encoding_type), Dword(dw_index), Pointer(pcb_signed_data_msg), Pointer(pb_signed_data_msg))
        }
        "CryptSIPRetrieveSubjectGuid" | "CryptSIPRetrieveSubjectGuidForCatalogFile" => {
            signature!(Win32Bool, 3, 3; Pointer(file_name), Pointer(h_file_in), Pointer(pg_subject))
        }
        "CryptSetHashParam" => {
            signature!(Win32Bool, 4, 4; Pointer(h_hash), Dword(dw_param), Pointer(pb_data), Dword(dw_flags))
        }
        "CryptSetKeyIdentifierProperty" => {
            signature!(Win32Bool, 6, 6; Pointer(p_key_identifier), Dword(dw_prop_id), Dword(dw_flags), WideString(pwsz_computer_name), Pointer(pv_reserved), Pointer(pv_data))
        }
        "CryptSetOIDFunctionValue" => {
            signature!(Win32Bool, 7, 7; Dword(dw_encoding_type), AnsiString(psz_func_name), AnsiString(psz_oid), WideString(pwsz_value_name), Dword(dw_value_type), Pointer(pb_value_data), Dword(cb_value_data))
        }
        "CryptSetProvParam" => {
            signature!(Win32Bool, 4, 4; Pointer(h_prov), Dword(dw_param), Pointer(pb_data), Dword(dw_flags))
        }
        "CryptSignAndEncodeCertificate" => {
            signature!(Win32Bool, 9, 9; Pointer(hcryptprovorncryptkey), Dword(dwkeyspec), Dword(dwcertencodingtype), Pointer(lpszstructtype), Pointer(pvstructinfo), Pointer(psignaturealgorithm), Pointer(pvhashauxinfo), Pointer(pbencoded), Pointer(pcbencoded))
        }
        "CryptSignAndEncryptMessage" => {
            signature!(Win32Bool, 8, 8; Pointer(p_sign_para), Pointer(p_encrypt_para), Dword(c_recipient_cert), Pointer(rgp_recipient_cert), Pointer(pb_to_be_signed_and_encrypted), Dword(cb_to_be_signed_and_encrypted), Pointer(pb_signed_and_encrypted_blob), Pointer(pcb_signed_and_encrypted_blob))
        }
        "CryptUIDlgSelectCertificateFromStore" => {
            signature!(Generic, 7, 7; Pointer(h_cert_store), Pointer(hwnd), WideString(pwsz_title), WideString(pwsz_display_string), Dword(dw_dont_use_column), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CryptUIDlgViewContext" => {
            signature!(Win32Bool, 6, 6; Dword(dw_context_type), Pointer(pv_context), Pointer(hwnd), WideString(pwsz_title), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CryptUIWizDigitalSign" => {
            signature!(Win32Bool, 5, 5; Dword(dw_flags), Pointer(hwnd_parent), WideString(pwsz_wizard_title), Pointer(p_digital_sign_info), Pointer(pp_sign_context))
        }
        "CryptUIWizExport" => {
            signature!(Win32Bool, 5, 5; Dword(dw_flags), Pointer(hwnd_parent), WideString(pwsz_wizard_title), Pointer(p_export_info), Pointer(pvoid))
        }
        "CryptUIWizFreeDigitalSignContext" => signature!(Win32Bool, 1, 1; Pointer(p_sign_context)),
        "CryptUIWizImport" => {
            signature!(Win32Bool, 5, 5; Dword(dw_flags), Pointer(hwnd_parent), WideString(pwsz_wizard_title), Pointer(p_import_src), Pointer(h_dest_cert_store))
        }
        "CryptUninstallDefaultContext" => {
            signature!(Win32Bool, 3, 3; Pointer(h_default_context), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CryptUnregisterDefaultOIDFunction" => {
            signature!(Win32Bool, 3, 3; Dword(dw_encoding_type), AnsiString(psz_func_name), WideString(pwsz_dll))
        }
        "CryptUnregisterOIDFunction" => {
            signature!(Win32Bool, 3, 3; Dword(dw_encoding_type), AnsiString(psz_func_name), AnsiString(psz_oid))
        }
        "CryptUnregisterOIDInfo" => signature!(Win32Bool, 1, 1; Pointer(p_info)),
        "CryptUpdateProtectedState" => {
            signature!(Win32Bool, 5, 5; Pointer(p_old_sid), Pointer(pwsz_old_password), Dword(dw_flags), Pointer(pdw_success_count), Pointer(pdw_failure_count))
        }
        "CryptVerifyMessageSignatureWithKey" => {
            signature!(Win32Bool, 6, 6; Pointer(p_verify_para), Pointer(p_public_key_info), Pointer(pb_signed_blob), Dword(cb_signed_blob), Pointer(pb_decoded), Pointer(pcb_decoded))
        }
        "CryptXmlAddObject" => {
            signature!(Hresult, 6, 6; Pointer(h_signature_or_object), Dword(dw_flags), Pointer(rg_property), Dword(c_property), Pointer(p_encoded), Pointer(pp_object))
        }
        "CryptXmlClose" => signature!(Hresult, 1, 1; Pointer(h_crypt_xml)),
        "CryptXmlCreateReference" => {
            signature!(Hresult, 9, 9; Pointer(h_crypt_xml), Dword(dw_flags), WideString(wsz_id), WideString(wsz_uri), WideString(wsz_type), Pointer(p_digest_method), Dword(c_transform), Pointer(rg_transform), Pointer(ph_reference))
        }
        "CryptXmlDigestReference" => {
            signature!(Hresult, 3, 3; Pointer(h_reference), Dword(dw_flags), Pointer(p_data_provider_in))
        }
        "CryptXmlEncode" => {
            signature!(Hresult, 6, 6; Pointer(h_crypt_xml), Dword(dw_charset), Pointer(rg_property), Dword(c_property), Pointer(pv_callback_state), Pointer(pfn_write))
        }
        "CryptXmlGetAlgorithmInfo" => {
            signature!(Hresult, 3, 3; Pointer(p_xml_algorithm), Dword(dw_flags), Pointer(pp_alg_info))
        }
        "CryptXmlGetDocContext" | "CryptXmlGetReference" | "CryptXmlGetSignature" => {
            signature!(Hresult, 2, 2; Pointer(h_crypt_xml), Pointer(pp_struct))
        }
        "CryptXmlGetStatus" => signature!(Hresult, 2, 2; Pointer(h_crypt_xml), Pointer(p_status)),
        "CryptXmlGetTransforms" => signature!(Hresult, 1, 1; Pointer(pp_config)),
        "CryptXmlImportPublicKey" => {
            signature!(Hresult, 3, 3; Dword(dw_flags), Pointer(p_key_value), Pointer(ph_key))
        }
        "CryptXmlOpenToDecode" => {
            signature!(Hresult, 6, 6; Pointer(p_config), Dword(dw_flags), Pointer(rg_property), Dword(c_property), Pointer(p_encoded), Pointer(ph_crypt_xml))
        }
        "CryptXmlOpenToEncode" => {
            signature!(Hresult, 7, 7; Pointer(p_config), Dword(dw_flags), WideString(wsz_id), Pointer(rg_property), Dword(c_property), Pointer(p_encoded), Pointer(ph_signature))
        }
        "CryptXmlSetHMACSecret" => {
            signature!(Hresult, 3, 3; Pointer(h_signature), Pointer(pb_secret), Dword(cb_secret))
        }
        "CryptXmlSign" => {
            signature!(Hresult, 8, 8; Pointer(h_signature), Pointer(h_key), Dword(dw_key_spec), Dword(dw_flags), Dword(dw_key_info_spec), Pointer(pv_key_info_spec), Pointer(p_signature_method), Pointer(p_canonicalization))
        }
        "CryptXmlVerifySignature" => {
            signature!(Hresult, 3, 3; Pointer(h_signature), Pointer(h_key), Dword(dw_flags))
        }
        "CertAddCRLContextToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_crl_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddCRLLinkToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_crl_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddCTLContextToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_ctl_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddCTLLinkToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_ctl_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddCertificateContextToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_cert_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddCertificateLinkToStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Pointer(p_cert_context), Dword(dw_add_disposition), Pointer(pp_store_context))
        }
        "CertAddEncodedCRLToStore" => {
            signature!(Win32Bool, 6, 6; Pointer(h_cert_store), Dword(dw_cert_encoding_type), Pointer(pb_crl_encoded), Dword(cb_crl_encoded), Dword(dw_add_disposition), Pointer(pp_crl_context))
        }
        "CertAddEncodedCTLToStore" => {
            signature!(Win32Bool, 6, 6; Pointer(h_cert_store), Dword(dw_msg_and_cert_encoding_type), Pointer(pb_ctl_encoded), Dword(cb_ctl_encoded), Dword(dw_add_disposition), Pointer(pp_ctl_context))
        }
        "CertAddEncodedCertificateToStore" => {
            signature!(Win32Bool, 6, 6; Pointer(h_cert_store), Dword(dw_cert_encoding_type), Pointer(pb_cert_encoded), Dword(cb_cert_encoded), Dword(dw_add_disposition), Pointer(pp_cert_context))
        }
        "CertAddEnhancedKeyUsageIdentifier" => {
            signature!(Win32Bool, 2, 2; Pointer(p_cert_context), AnsiString(psz_usage_identifier))
        }
        "CertAddRefServerOcspResponse" => signature!(Void, 1, 1; Pointer(h_server_ocsp_response)),
        "CertAddRefServerOcspResponseContext" => {
            signature!(Void, 1, 1; Pointer(p_server_ocsp_response_context))
        }
        "CertAddSerializedElementToStore" => {
            signature!(Win32Bool, 8, 8; Pointer(h_cert_store), Pointer(pb_element), Dword(cb_element), Dword(dw_add_disposition), Dword(dw_flags), Dword(dw_context_type_flags), Pointer(pdw_context_type), Pointer(ppv_context))
        }
        "CertAddStoreToCollection" => {
            signature!(Win32Bool, 4, 4; Pointer(h_collection_store), Pointer(h_sibling_store), Dword(dw_update_flags), Dword(dw_priority))
        }
        "CertAlgIdToOID" => signature!(Generic, 1, 1; Dword(dw_alg_id)),
        "CertCloseServerOcspResponse" => {
            signature!(Void, 2, 2; Pointer(h_server_ocsp_response), Dword(dw_flags))
        }
        "CertComparePublicKeyInfo" => {
            signature!(Win32Bool, 3, 3; Dword(dw_cert_encoding_type), Pointer(p_public_key1), Pointer(p_public_key2))
        }
        "CertControlStore" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Dword(dw_flags), Dword(dw_ctrl_type), Pointer(pv_ctrl_para))
        }
        "CertCreateCRLContext" => {
            signature!(Generic, 3, 3; Dword(dw_cert_encoding_type), Pointer(pb_crl_encoded), Dword(cb_crl_encoded))
        }
        "CertCreateCTLContext" => {
            signature!(Generic, 3, 3; Dword(dw_msg_and_cert_encoding_type), Pointer(pb_ctl_encoded), Dword(cb_ctl_encoded))
        }
        "CertCreateCTLEntryFromCertificateContextProperties" => {
            signature!(Win32Bool, 7, 7; Pointer(p_cert_context), Dword(c_opt_attr), Pointer(rg_opt_attr), Dword(dw_flags), Pointer(pv_reserved), Pointer(p_ctl_entry), Pointer(pcb_ctl_entry))
        }
        "CertDeleteCRLFromStore" => signature!(Win32Bool, 1, 1; Pointer(p_crl_context)),
        "CertDeleteCTLFromStore" => signature!(Win32Bool, 1, 1; Pointer(p_ctl_context)),
        "CertDeleteCertificateFromStore" => signature!(Win32Bool, 1, 1; Pointer(p_cert_context)),
        "CertEnumCRLContextProperties" => {
            signature!(Generic, 2, 2; Pointer(p_crl_context), Dword(dw_prop_id))
        }
        "CertEnumCRLsInStore" => {
            signature!(Generic, 2, 2; Pointer(h_cert_store), Pointer(p_prev_crl_context))
        }
        "CertEnumCTLContextProperties" => {
            signature!(Generic, 2, 2; Pointer(p_ctl_context), Dword(dw_prop_id))
        }
        "CertEnumCTLsInStore" => {
            signature!(Generic, 2, 2; Pointer(h_cert_store), Pointer(p_prev_ctl_context))
        }
        "CertEnumCertificateContextProperties" => {
            signature!(Generic, 2, 2; Pointer(p_cert_context), Dword(dw_prop_id))
        }
        "CertEnumPhysicalStore" => {
            signature!(Win32Bool, 4, 4; Pointer(pv_system_store), Dword(dw_flags), Pointer(pv_arg), Pointer(pfn_enum))
        }
        "CertEnumSubjectInSortedCTL" => {
            signature!(Win32Bool, 4, 4; Pointer(p_ctl_context), Pointer(ppv_next_subject), Pointer(p_subject_identifier), Pointer(p_encoded_attributes))
        }
        "CertEnumSystemStore" => {
            signature!(Win32Bool, 4, 4; Dword(dw_flags), Pointer(pv_system_store_location_para), Pointer(pv_arg), Pointer(pfn_enum))
        }
        "CertEnumSystemStoreLocation" => {
            signature!(Win32Bool, 3, 3; Dword(dw_flags), Pointer(pv_arg), Pointer(pfn_enum))
        }
        "CertFindCRLInStore" => {
            signature!(Generic, 6, 6; Pointer(h_cert_store), Dword(dw_cert_encoding_type), Dword(dw_find_flags), Dword(dw_find_type), Pointer(pv_find_para), Pointer(p_prev_crl_context))
        }
        "CertFindCTLInStore" => {
            signature!(Generic, 6, 6; Pointer(h_cert_store), Dword(dw_msg_and_cert_encoding_type), Dword(dw_find_flags), Dword(dw_find_type), Pointer(pv_find_para), Pointer(p_prev_ctl_context))
        }
        "CertFreeCRLContext" => signature!(Win32Bool, 1, 1; Pointer(p_crl_context)),
        "CertFreeCTLContext" => signature!(Win32Bool, 1, 1; Pointer(p_ctl_context)),
        "CertFreeCertificateChain" => signature!(Void, 1, 1; Pointer(p_chain_context)),
        "CertFreeCertificateChainEngine" => signature!(Void, 1, 1; Pointer(h_chain_engine)),
        "CertFreeCertificateChainList" => signature!(Void, 1, 1; Pointer(prgp_selection)),
        "CertGetCRLFromStore" => {
            signature!(Generic, 4, 4; Pointer(h_cert_store), Pointer(p_issuer_context), Pointer(p_prev_crl_context), Pointer(pdw_flags))
        }
        "CertGetCertificateChain" => {
            signature!(Win32Bool, 8, 8; Pointer(h_chain_engine), Pointer(p_cert_context), Pointer(p_time), Pointer(h_additional_store), Pointer(p_chain_para), Dword(dw_flags), Pointer(pv_reserved), Pointer(pp_chain_context))
        }
        "CertGetIssuerCertificateFromStore" => {
            signature!(Generic, 4, 4; Pointer(h_cert_store), Pointer(p_subject_context), Pointer(p_prev_issuer_context), Pointer(pdw_flags))
        }
        "CertGetNameString" => {
            signature!(Generic, 6, 6; Pointer(pcertcontext), Dword(dwtype), Dword(dwflags), Pointer(pvtypepara), Pointer(psznamestring), Dword(cchnamestring))
        }
        "CertGetNameStringA" => {
            signature!(Generic, 6, 6; Pointer(p_cert_context), Dword(dw_type), Dword(dw_flags), Pointer(pv_type_para), Pointer(psz_name_string), Dword(cch_name_string))
        }
        "CertGetNameStringW" => {
            signature!(Generic, 6, 6; Pointer(p_cert_context), Dword(dw_type), Dword(dw_flags), Pointer(pv_type_para), Pointer(psz_name_string), Dword(cch_name_string))
        }
        "CertIsRDNAttrsInCertificateName" => {
            signature!(Win32Bool, 4, 4; Dword(dw_cert_encoding_type), Dword(dw_flags), Pointer(p_cert_name), Pointer(p_rdn))
        }
        "CertIsStrongHashToSign" => {
            signature!(Win32Bool, 3, 3; Pointer(p_strong_sign_para), WideString(pwsz_cng_hash_algid), Pointer(p_signing_cert))
        }
        "CertIsValidCRLForCertificate" => {
            signature!(Win32Bool, 4, 4; Pointer(p_cert), Pointer(p_crl), Dword(dw_flags), Pointer(pv_reserved))
        }
        "CertNameToStr" => {
            signature!(Generic, 5, 5; Dword(dwcertencodingtype), Pointer(pname), Dword(dwstrtype), Pointer(psz), Dword(csz))
        }
        "CertNameToStrA" => {
            signature!(Generic, 5, 5; Dword(dw_cert_encoding_type), Pointer(p_name), Dword(dw_str_type), Pointer(psz), Dword(csz))
        }
        "CertNameToStrW" => {
            signature!(Generic, 5, 5; Dword(dw_cert_encoding_type), Pointer(p_name), Dword(dw_str_type), Pointer(psz), Dword(csz))
        }
        "CertOIDToAlgId" => signature!(Generic, 1, 1; AnsiString(psz_obj_id)),
        "CertRDNValueToStr" => {
            signature!(Generic, 4, 4; Dword(dwvaluetype), Pointer(pvalue), Pointer(psz), Dword(csz))
        }
        "CertRDNValueToStrA" => {
            signature!(Generic, 4, 4; Dword(dw_value_type), Pointer(p_value), Pointer(psz), Dword(csz))
        }
        "CertRDNValueToStrW" => {
            signature!(Generic, 4, 4; Dword(dw_value_type), Pointer(p_value), Pointer(psz), Dword(csz))
        }
        "CertRegisterPhysicalStore" => {
            signature!(Win32Bool, 5, 5; Pointer(pv_system_store), Dword(dw_flags), WideString(pwsz_store_name), Pointer(p_store_info), Pointer(pv_reserved))
        }
        "CertRegisterSystemStore" => {
            signature!(Win32Bool, 4, 4; Pointer(pv_system_store), Dword(dw_flags), Pointer(p_store_info), Pointer(pv_reserved))
        }
        "CertRemoveEnhancedKeyUsageIdentifier" => {
            signature!(Win32Bool, 2, 2; Pointer(p_cert_context), AnsiString(psz_usage_identifier))
        }
        "CertRemoveStoreFromCollection" => {
            signature!(Void, 2, 2; Pointer(h_collection_store), Pointer(h_sibling_store))
        }
        "CertRetrieveLogoOrBiometricInfo" => {
            signature!(Win32Bool, 9, 9; Pointer(p_cert_context), AnsiString(lpsz_logo_or_biometric_type), Dword(dw_retrieval_flags), Dword(dw_timeout), Dword(dw_flags), Pointer(pv_reserved), Pointer(ppb_data), Pointer(pcb_data), Pointer(ppwsz_mime_type))
        }
        "CertSaveStore" => {
            signature!(Win32Bool, 6, 6; Pointer(h_cert_store), Dword(dw_encoding_type), Dword(dw_save_as), Dword(dw_save_to), Pointer(pv_save_to_para), Dword(dw_flags))
        }
        "CertSerializeCRLStoreElement" => {
            signature!(Win32Bool, 4, 4; Pointer(p_crl_context), Dword(dw_flags), Pointer(pb_element), Pointer(pcb_element))
        }
        "CertSerializeCTLStoreElement" => {
            signature!(Win32Bool, 4, 4; Pointer(p_ctl_context), Dword(dw_flags), Pointer(pb_element), Pointer(pcb_element))
        }
        "CertSerializeCertificateStoreElement" => {
            signature!(Win32Bool, 4, 4; Pointer(p_cert_context), Dword(dw_flags), Pointer(pb_element), Pointer(pcb_element))
        }
        "CertSetCRLContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_crl_context), Dword(dw_prop_id), Dword(dw_flags), Pointer(pv_data))
        }
        "CertSetCTLContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_ctl_context), Dword(dw_prop_id), Dword(dw_flags), Pointer(pv_data))
        }
        "CertSetCertificateContextPropertiesFromCTLEntry" => {
            signature!(Win32Bool, 3, 3; Pointer(p_cert_context), Pointer(p_ctl_entry), Dword(dw_flags))
        }
        "CertSetCertificateContextProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(p_cert_context), Dword(dw_prop_id), Dword(dw_flags), Pointer(pv_data))
        }
        "CertSetEnhancedKeyUsage" => {
            signature!(Win32Bool, 2, 2; Pointer(p_cert_context), Pointer(p_usage))
        }
        "CertSetStoreProperty" => {
            signature!(Win32Bool, 4, 4; Pointer(h_cert_store), Dword(dw_prop_id), Dword(dw_flags), Pointer(pv_data))
        }
        "CertSrvBackupClose" => signature!(Hresult, 1, 1; Pointer(hbc)),
        "CertSrvBackupEnd" => signature!(Hresult, 1, 1; Pointer(hbc)),
        "CertSrvBackupFree" => signature!(Void, 1, 1; Pointer(pv)),
        "CertSrvBackupGetBackupLogsW" => {
            signature!(Hresult, 3, 3; Pointer(hbc), Pointer(ppwszz_backup_log_files), Pointer(pcb_size))
        }
        "CertSrvBackupGetDatabaseNamesW" => {
            signature!(Hresult, 3, 3; Pointer(hbc), Pointer(ppwszz_attachment_information), Pointer(pcb_size))
        }
        "CertSrvBackupGetDynamicFileListW" => {
            signature!(Hresult, 3, 3; Pointer(hbc), Pointer(ppwszz_file_list), Pointer(pcb_size))
        }
        "CertSrvBackupOpenFileW" => {
            signature!(Hresult, 4, 4; Pointer(hbc), Pointer(pwsz_attachment_name), Dword(cb_read_hint_size), Pointer(pli_file_size))
        }
        "CertSrvBackupPrepareW" => {
            signature!(Hresult, 4, 4; Pointer(pwsz_server_name), Dword(grbit_jet), Dword(dw_backup_flags), Pointer(phbc))
        }
        "CertSrvBackupRead" => {
            signature!(Hresult, 4, 4; Pointer(hbc), Pointer(pv_buffer), Dword(cb_buffer), Pointer(pcb_read))
        }
        "CertSrvBackupTruncateLogs" => signature!(Hresult, 1, 1; Pointer(hbc)),
        "CertSrvIsServerOnlineW" => {
            signature!(Hresult, 2, 2; Pointer(pwsz_server_name), Pointer(pf_server_online))
        }
        "CertSrvRestoreEnd" => signature!(Hresult, 1, 1; Pointer(hbc)),
        "CertSrvRestoreGetDatabaseLocationsW" => {
            signature!(Hresult, 3, 3; Pointer(hbc), Pointer(ppwszz_database_location_list), Pointer(pcb_size))
        }
        "CertSrvRestorePrepareW" => {
            signature!(Hresult, 3, 3; Pointer(pwsz_server_name), Dword(dw_restore_flags), Pointer(phbc))
        }
        "CertSrvRestoreRegisterW" => {
            signature!(Hresult, 8, 8; Pointer(hbc), Pointer(pwsz_check_point_file_path), Pointer(pwsz_log_path), Pointer(rgrstmap), Dword(crstmap), Pointer(pwsz_backup_log_path), Dword(gen_low), Dword(gen_high))
        }
        "CertSrvServerControlW" => {
            signature!(Hresult, 4, 4; Pointer(pwsz_server_name), Dword(dw_control_flags), Pointer(pcb_out), Pointer(ppb_out))
        }
        "CertStrToName" => {
            signature!(Win32Bool, 7, 7; Dword(dwcertencodingtype), Pointer(pszx500), Dword(dwstrtype), Pointer(pvreserved), Pointer(pbencoded), Pointer(pcbencoded), Pointer(ppszerror))
        }
        "CertStrToNameA" => {
            signature!(Win32Bool, 7, 7; Dword(dw_cert_encoding_type), AnsiString(psz_x500), Dword(dw_str_type), Pointer(pv_reserved), Pointer(pb_encoded), Pointer(pcb_encoded), Pointer(ppsz_error))
        }
        "CertStrToNameW" => {
            signature!(Win32Bool, 7, 7; Dword(dw_cert_encoding_type), WideString(psz_x500), Dword(dw_str_type), Pointer(pv_reserved), Pointer(pb_encoded), Pointer(pcb_encoded), Pointer(ppsz_error))
        }
        "CertUnregisterPhysicalStore" => {
            signature!(Win32Bool, 3, 3; Pointer(pv_system_store), Dword(dw_flags), WideString(pwsz_store_name))
        }
        "CertUnregisterSystemStore" => {
            signature!(Win32Bool, 2, 2; Pointer(pv_system_store), Dword(dw_flags))
        }
        "CertVerifyCRLRevocation" => {
            signature!(Win32Bool, 4, 4; Dword(dw_cert_encoding_type), Pointer(p_cert_id), Dword(c_crl_info), Pointer(rgp_crl_info))
        }
        "CertVerifyCRLTimeValidity" => {
            signature!(Generic, 2, 2; Pointer(p_time_to_verify), Pointer(p_crl_info))
        }
        "CertVerifyCertificateChainPolicy" => {
            signature!(Win32Bool, 4, 4; AnsiString(psz_policy_oid), Pointer(p_chain_context), Pointer(p_policy_para), Pointer(p_policy_status))
        }
        "CertVerifySubjectCertificateContext" => {
            signature!(Win32Bool, 3, 3; Pointer(p_subject), Pointer(p_issuer), Pointer(pdw_flags))
        }
        "CryptCATAdminAcquireContext" => {
            signature!(Win32Bool, 3, 3; Pointer(ph_cat_admin), Pointer(pg_subsystem), Dword(dw_flags))
        }
        "CryptCATAdminAcquireContext2" => {
            signature!(Win32Bool, 5, 5; Pointer(ph_cat_admin), Pointer(pg_subsystem), WideString(pwsz_hash_algorithm), Pointer(p_strong_hash_policy), Dword(dw_flags))
        }
        "CryptCATAdminAddCatalog" => {
            signature!(Generic, 4, 4; Pointer(h_cat_admin), WideString(pwsz_catalog_file), WideString(pwsz_select_base_name), Dword(dw_flags))
        }
        "CryptCATAdminCalcHashFromFileHandle" => {
            signature!(Win32Bool, 4, 4; Pointer(h_file), Pointer(pcb_hash), Pointer(pb_hash), Dword(dw_flags))
        }
        "CryptCATAdminCalcHashFromFileHandle2" => {
            signature!(Win32Bool, 5, 5; Pointer(h_cat_admin), Pointer(h_file), Pointer(pcb_hash), Pointer(pb_hash), Dword(dw_flags))
        }
        "CryptCATAdminEnumCatalogFromHash" => {
            signature!(Generic, 5, 5; Pointer(h_cat_admin), Pointer(pb_hash), Dword(cb_hash), Dword(dw_flags), Pointer(ph_prev_cat_info))
        }
        "CryptCATAdminReleaseCatalogContext" => {
            signature!(Win32Bool, 3, 3; Pointer(h_cat_admin), Pointer(h_cat_info), Dword(dw_flags))
        }
        "CryptCATAdminReleaseContext" => {
            signature!(Win32Bool, 2, 2; Pointer(h_cat_admin), Dword(dw_flags))
        }
        "CryptDecryptAndVerifyMessageSignature" => {
            signature!(Win32Bool, 9, 9; Pointer(p_decrypt_para), Pointer(p_verify_para), Dword(dw_signer_index), Pointer(pb_encrypted_blob), Dword(cb_encrypted_blob), Pointer(pb_decrypted), Pointer(pcb_decrypted), Pointer(pp_xchg_cert), Pointer(pp_signer_cert))
        }
        "CryptDecryptMessage" => {
            signature!(Win32Bool, 6, 6; Pointer(p_decrypt_para), Pointer(pb_encrypted_blob), Dword(cb_encrypted_blob), Pointer(pb_decrypted), Pointer(pcb_decrypted), Pointer(pp_xchg_cert))
        }
        "CryptEncodeObject" => {
            signature!(Win32Bool, 5, 5; Dword(dw_cert_encoding_type), AnsiString(lpsz_struct_type), Pointer(pv_struct_info), Pointer(pb_encoded), Pointer(pcb_encoded))
        }
        "CryptEncodeObjectEx" => {
            signature!(Win32Bool, 7, 7; Dword(dw_cert_encoding_type), AnsiString(lpsz_struct_type), Pointer(pv_struct_info), Dword(dw_flags), Pointer(p_encode_para), Pointer(pv_encoded), Pointer(pcb_encoded))
        }
        "CryptExportPublicKeyInfo" => {
            signature!(Win32Bool, 5, 5; Pointer(h_crypt_prov_or_n_crypt_key), Dword(dw_key_spec), Dword(dw_cert_encoding_type), Pointer(p_info), Pointer(pcb_info))
        }
        "CryptExportPublicKeyInfoEx" => {
            signature!(Win32Bool, 8, 8; Pointer(h_crypt_prov_or_n_crypt_key), Dword(dw_key_spec), Dword(dw_cert_encoding_type), AnsiString(psz_public_key_obj_id), Dword(dw_flags), Pointer(pv_aux_info), Pointer(p_info), Pointer(pcb_info))
        }
        "CryptHashCertificate2" => {
            signature!(Win32Bool, 7, 7; WideString(pwsz_cng_hash_algid), Dword(dw_flags), Pointer(pv_reserved), Pointer(pb_encoded), Dword(cb_encoded), Pointer(pb_computed_hash), Pointer(pcb_computed_hash))
        }
        "CryptHashMessage" => {
            signature!(Win32Bool, 9, 9; Pointer(p_hash_para), Dword(f_detached_hash), Dword(c_to_be_hashed), Pointer(rgpb_to_be_hashed), Pointer(rgcb_to_be_hashed), Pointer(pb_hashed_blob), Pointer(pcb_hashed_blob), Pointer(pb_computed_hash), Pointer(pcb_computed_hash))
        }
        "CryptHashToBeSigned" => {
            signature!(Win32Bool, 6, 6; Pointer(h_crypt_prov), Dword(dw_cert_encoding_type), Pointer(pb_encoded), Dword(cb_encoded), Pointer(pb_computed_hash), Pointer(pcb_computed_hash))
        }
        "CryptImportPublicKeyInfo" => {
            signature!(Win32Bool, 4, 4; Pointer(h_crypt_prov), Dword(dw_cert_encoding_type), Pointer(p_info), Pointer(ph_key))
        }
        "CryptMsgControl" => {
            signature!(Win32Bool, 4, 4; Pointer(h_crypt_msg), Dword(dw_flags), Dword(dw_ctrl_type), Pointer(pv_ctrl_para))
        }
        "CryptMsgEncodeAndSignCTL" => {
            signature!(Win32Bool, 6, 6; Dword(dw_msg_encoding_type), Pointer(p_ctl_info), Pointer(p_sign_info), Dword(dw_flags), Pointer(pb_encoded), Pointer(pcb_encoded))
        }
        "CryptMsgGetParam" => {
            signature!(Win32Bool, 5, 5; Pointer(h_crypt_msg), Dword(dw_param_type), Dword(dw_index), Pointer(pv_data), Pointer(pcb_data))
        }
        "CryptMsgSignCTL" => {
            signature!(Win32Bool, 7, 7; Dword(dw_msg_encoding_type), Pointer(pb_ctl_content), Dword(cb_ctl_content), Pointer(p_sign_info), Dword(dw_flags), Pointer(pb_encoded), Pointer(pcb_encoded))
        }
        "CryptSIPAddProvider" => signature!(Win32Bool, 1, 1; Pointer(ps_new_prov)),
        "CryptSIPPutSignedDataMsg" => {
            signature!(Win32Bool, 5, 5; Pointer(p_subject_info), Dword(dw_encoding_type), Pointer(pdw_index), Dword(cb_signed_data_msg), Pointer(pb_signed_data_msg))
        }
        "CryptSIPRemoveProvider" => signature!(Win32Bool, 1, 1; Pointer(pg_prov)),
        "CryptSIPRemoveSignedDataMsg" => {
            signature!(Win32Bool, 2, 2; Pointer(p_subject_info), Dword(dw_index))
        }
        "CryptSIPVerifyIndirectData" => {
            signature!(Win32Bool, 2, 2; Pointer(p_subject_info), Pointer(p_indirect_data))
        }
        "CryptSignCertificate" => {
            signature!(Win32Bool, 9, 9; Pointer(hcryptprovorncryptkey), Dword(dwkeyspec), Dword(dwcertencodingtype), Pointer(pbencodedtobesigned), Dword(cbencodedtobesigned), Pointer(psignaturealgorithm), Pointer(pvhashauxinfo), Pointer(pbsignature), Pointer(pcbsignature))
        }
        "CryptSignMessage" => {
            signature!(Win32Bool, 7, 7; Pointer(p_sign_para), Dword(f_detached_signature), Dword(c_to_be_signed), Pointer(rgpb_to_be_signed), Pointer(rgcb_to_be_signed), Pointer(pb_signed_blob), Pointer(pcb_signed_blob))
        }
        "CryptSignMessageWithKey" => {
            signature!(Win32Bool, 5, 5; Pointer(p_sign_para), Pointer(pb_to_be_signed), Dword(cb_to_be_signed), Pointer(pb_signed_blob), Pointer(pcb_signed_blob))
        }
        "CryptStringToBinary" => {
            signature!(Win32Bool, 7, 7; Pointer(pszstring), Dword(cchstring), Dword(dwflags), Pointer(pbbinary), Pointer(pcbbinary), Pointer(pdwskip), Pointer(pdwflags))
        }
        "CryptStringToBinaryA" => {
            signature!(Win32Bool, 7, 7; Pointer(psz_string), Dword(cch_string), Dword(dw_flags), Pointer(pb_binary), Pointer(pcb_binary), Pointer(pdw_skip), Pointer(pdw_flags))
        }
        "CryptStringToBinaryW" => {
            signature!(Win32Bool, 7, 7; Pointer(psz_string), Dword(cch_string), Dword(dw_flags), Pointer(pb_binary), Pointer(pcb_binary), Pointer(pdw_skip), Pointer(pdw_flags))
        }
        "CryptUIDlgCertMgr" => signature!(Win32Bool, 1, 1; Pointer(p_crypt_ui_cert_mgr)),
        "CryptVerifyCertificateSignature" => {
            signature!(Win32Bool, 5, 5; Pointer(h_crypt_prov), Dword(dw_cert_encoding_type), Pointer(pb_encoded), Dword(cb_encoded), Pointer(p_public_key))
        }
        "CryptVerifyDetachedMessageHash" => {
            signature!(Win32Bool, 8, 8; Pointer(p_hash_para), Pointer(pb_detached_hash_blob), Dword(cb_detached_hash_blob), Dword(c_to_be_hashed), Pointer(rgpb_to_be_hashed), Pointer(rgcb_to_be_hashed), Pointer(pb_computed_hash), Pointer(pcb_computed_hash))
        }
        "CryptVerifyMessageHash" => {
            signature!(Win32Bool, 7, 7; Pointer(p_hash_para), Pointer(pb_hashed_blob), Dword(cb_hashed_blob), Pointer(pb_to_be_hashed), Pointer(pcb_to_be_hashed), Pointer(pb_computed_hash), Pointer(pcb_computed_hash))
        }
        "CryptVerifyMessageSignature" => {
            signature!(Win32Bool, 7, 7; Pointer(p_verify_para), Dword(dw_signer_index), Pointer(pb_signed_blob), Dword(cb_signed_blob), Pointer(pb_decoded), Pointer(pcb_decoded), Pointer(pp_signer_cert))
        }
        "PFXExportCertStore" => {
            signature!(Generic, 4, 4; Pointer(h_store), Pointer(p_pfx), Pointer(sz_password), Dword(dw_flags))
        }
        "PFXExportCertStoreEx" => {
            signature!(Generic, 5, 5; Pointer(h_store), Pointer(p_pfx), Pointer(sz_password), Pointer(pv_para), Dword(dw_flags))
        }
        "PFXImportCertStore" => {
            signature!(Generic, 3, 3; Pointer(p_pfx), Pointer(sz_password), Dword(dw_flags))
        }
        "PFXIsPFXBlob" => signature!(Generic, 1, 1; Pointer(p_pfx)),
        "PFXVerifyPassword" => {
            signature!(Generic, 3, 3; Pointer(p_pfx), Pointer(sz_password), Dword(dw_flags))
        }
        _ => None,
    }
}
