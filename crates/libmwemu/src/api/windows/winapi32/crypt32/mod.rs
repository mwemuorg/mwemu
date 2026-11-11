use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod cert_add_crl_context_to_store;
pub use cert_add_crl_context_to_store::CertAddCRLContextToStore;
mod cert_add_crl_link_to_store;
pub use cert_add_crl_link_to_store::CertAddCRLLinkToStore;
mod cert_add_ctl_context_to_store;
pub use cert_add_ctl_context_to_store::CertAddCTLContextToStore;
mod cert_add_ctl_link_to_store;
pub use cert_add_ctl_link_to_store::CertAddCTLLinkToStore;
mod cert_add_certificate_context_to_store;
pub use cert_add_certificate_context_to_store::CertAddCertificateContextToStore;
mod cert_add_certificate_link_to_store;
pub use cert_add_certificate_link_to_store::CertAddCertificateLinkToStore;
mod cert_add_encoded_crl_to_store;
pub use cert_add_encoded_crl_to_store::CertAddEncodedCRLToStore;
mod cert_add_encoded_ctl_to_store;
pub use cert_add_encoded_ctl_to_store::CertAddEncodedCTLToStore;
mod cert_add_encoded_certificate_to_store;
pub use cert_add_encoded_certificate_to_store::CertAddEncodedCertificateToStore;
mod cert_add_enhanced_key_usage_identifier;
pub use cert_add_enhanced_key_usage_identifier::CertAddEnhancedKeyUsageIdentifier;
mod cert_add_ref_server_ocsp_response;
pub use cert_add_ref_server_ocsp_response::CertAddRefServerOcspResponse;
mod cert_add_ref_server_ocsp_response_context;
pub use cert_add_ref_server_ocsp_response_context::CertAddRefServerOcspResponseContext;
mod cert_add_serialized_element_to_store;
pub use cert_add_serialized_element_to_store::CertAddSerializedElementToStore;
mod cert_add_store_to_collection;
pub use cert_add_store_to_collection::CertAddStoreToCollection;
mod cert_alg_id_to_oid;
pub use cert_alg_id_to_oid::CertAlgIdToOID;
mod cert_close_server_ocsp_response;
pub use cert_close_server_ocsp_response::CertCloseServerOcspResponse;
mod cert_compare_certificate;
pub use cert_compare_certificate::CertCompareCertificate;
mod cert_compare_certificate_name;
pub use cert_compare_certificate_name::CertCompareCertificateName;
mod cert_compare_integer_blob;
pub use cert_compare_integer_blob::CertCompareIntegerBlob;
mod cert_compare_public_key_info;
pub use cert_compare_public_key_info::CertComparePublicKeyInfo;
mod cert_control_store;
pub use cert_control_store::CertControlStore;
mod cert_create_crl_context;
pub use cert_create_crl_context::CertCreateCRLContext;
mod cert_create_ctl_context;
pub use cert_create_ctl_context::CertCreateCTLContext;
mod cert_create_ctl_entry_from_certificate_context_properties;
pub use cert_create_ctl_entry_from_certificate_context_properties::CertCreateCTLEntryFromCertificateContextProperties;
mod cert_create_certificate_chain_engine;
pub use cert_create_certificate_chain_engine::CertCreateCertificateChainEngine;
mod cert_create_certificate_context;
pub use cert_create_certificate_context::CertCreateCertificateContext;
mod cert_create_context;
pub use cert_create_context::CertCreateContext;
mod cert_create_self_sign_certificate;
pub use cert_create_self_sign_certificate::CertCreateSelfSignCertificate;
mod cert_delete_crl_from_store;
pub use cert_delete_crl_from_store::CertDeleteCRLFromStore;
mod cert_delete_ctl_from_store;
pub use cert_delete_ctl_from_store::CertDeleteCTLFromStore;
mod cert_delete_certificate_from_store;
pub use cert_delete_certificate_from_store::CertDeleteCertificateFromStore;
mod cert_duplicate_crl_context;
pub use cert_duplicate_crl_context::CertDuplicateCRLContext;
mod cert_duplicate_ctl_context;
pub use cert_duplicate_ctl_context::CertDuplicateCTLContext;
mod cert_duplicate_certificate_chain;
pub use cert_duplicate_certificate_chain::CertDuplicateCertificateChain;
mod cert_duplicate_certificate_context;
pub use cert_duplicate_certificate_context::CertDuplicateCertificateContext;
mod cert_duplicate_store;
pub use cert_duplicate_store::CertDuplicateStore;
mod cert_enum_crl_context_properties;
pub use cert_enum_crl_context_properties::CertEnumCRLContextProperties;
mod cert_enum_cr_ls_in_store;
pub use cert_enum_cr_ls_in_store::CertEnumCRLsInStore;
mod cert_enum_ctl_context_properties;
pub use cert_enum_ctl_context_properties::CertEnumCTLContextProperties;
mod cert_enum_ct_ls_in_store;
pub use cert_enum_ct_ls_in_store::CertEnumCTLsInStore;
mod cert_enum_certificate_context_properties;
pub use cert_enum_certificate_context_properties::CertEnumCertificateContextProperties;
mod cert_enum_certificates_in_store;
pub use cert_enum_certificates_in_store::CertEnumCertificatesInStore;
mod cert_enum_physical_store;
pub use cert_enum_physical_store::CertEnumPhysicalStore;
mod cert_enum_subject_in_sorted_ctl;
pub use cert_enum_subject_in_sorted_ctl::CertEnumSubjectInSortedCTL;
mod cert_enum_system_store;
pub use cert_enum_system_store::CertEnumSystemStore;
mod cert_enum_system_store_location;
pub use cert_enum_system_store_location::CertEnumSystemStoreLocation;
mod cert_find_attribute;
pub use cert_find_attribute::CertFindAttribute;
mod cert_find_crl_in_store;
pub use cert_find_crl_in_store::CertFindCRLInStore;
mod cert_find_ctl_in_store;
pub use cert_find_ctl_in_store::CertFindCTLInStore;
mod cert_find_certificate_in_crl;
pub use cert_find_certificate_in_crl::CertFindCertificateInCRL;
mod cert_find_chain_in_store;
pub use cert_find_chain_in_store::CertFindChainInStore;
mod cert_find_extension;
pub use cert_find_extension::CertFindExtension;
mod cert_find_rdn_attr;
pub use cert_find_rdn_attr::CertFindRDNAttr;
mod cert_find_subject_in_ctl;
pub use cert_find_subject_in_ctl::CertFindSubjectInCTL;
mod cert_find_subject_in_sorted_ctl;
pub use cert_find_subject_in_sorted_ctl::CertFindSubjectInSortedCTL;
mod cert_free_crl_context;
pub use cert_free_crl_context::CertFreeCRLContext;
mod cert_free_ctl_context;
pub use cert_free_ctl_context::CertFreeCTLContext;
mod cert_free_certificate_chain;
pub use cert_free_certificate_chain::CertFreeCertificateChain;
mod cert_free_certificate_chain_engine;
pub use cert_free_certificate_chain_engine::CertFreeCertificateChainEngine;
mod cert_free_certificate_chain_list;
pub use cert_free_certificate_chain_list::CertFreeCertificateChainList;
mod cert_get_crl_context_property;
pub use cert_get_crl_context_property::CertGetCRLContextProperty;
mod cert_get_crl_from_store;
pub use cert_get_crl_from_store::CertGetCRLFromStore;
mod cert_get_ctl_context_property;
pub use cert_get_ctl_context_property::CertGetCTLContextProperty;
mod cert_get_certificate_chain;
pub use cert_get_certificate_chain::CertGetCertificateChain;
mod cert_get_certificate_context_property;
pub use cert_get_certificate_context_property::CertGetCertificateContextProperty;
mod cert_get_enhanced_key_usage;
pub use cert_get_enhanced_key_usage::CertGetEnhancedKeyUsage;
mod cert_get_intended_key_usage;
pub use cert_get_intended_key_usage::CertGetIntendedKeyUsage;
mod cert_get_issuer_certificate_from_store;
pub use cert_get_issuer_certificate_from_store::CertGetIssuerCertificateFromStore;
mod cert_get_name_string;
pub use cert_get_name_string::CertGetNameString;
mod cert_get_public_key_length;
pub use cert_get_public_key_length::CertGetPublicKeyLength;
mod cert_get_server_ocsp_response_context;
pub use cert_get_server_ocsp_response_context::CertGetServerOcspResponseContext;
mod cert_get_store_property;
pub use cert_get_store_property::CertGetStoreProperty;
mod cert_get_subject_certificate_from_store;
pub use cert_get_subject_certificate_from_store::CertGetSubjectCertificateFromStore;
mod cert_get_valid_usages;
pub use cert_get_valid_usages::CertGetValidUsages;
mod cert_is_rdn_attrs_in_certificate_name;
pub use cert_is_rdn_attrs_in_certificate_name::CertIsRDNAttrsInCertificateName;
mod cert_is_strong_hash_to_sign;
pub use cert_is_strong_hash_to_sign::CertIsStrongHashToSign;
mod cert_is_valid_crl_for_certificate;
pub use cert_is_valid_crl_for_certificate::CertIsValidCRLForCertificate;
mod cert_modify_certificates_to_trust;
pub use cert_modify_certificates_to_trust::CertModifyCertificatesToTrust;
mod cert_name_to_str;
pub use cert_name_to_str::CertNameToStr;
mod cert_oid_to_alg_id;
pub use cert_oid_to_alg_id::CertOIDToAlgId;
mod cert_open_server_ocsp_response;
pub use cert_open_server_ocsp_response::CertOpenServerOcspResponse;
mod cert_open_system_store;
pub use cert_open_system_store::CertOpenSystemStore;
mod cert_rdn_value_to_str;
pub use cert_rdn_value_to_str::CertRDNValueToStr;
mod cert_register_physical_store;
pub use cert_register_physical_store::CertRegisterPhysicalStore;
mod cert_register_system_store;
pub use cert_register_system_store::CertRegisterSystemStore;
mod cert_remove_enhanced_key_usage_identifier;
pub use cert_remove_enhanced_key_usage_identifier::CertRemoveEnhancedKeyUsageIdentifier;
mod cert_remove_store_from_collection;
pub use cert_remove_store_from_collection::CertRemoveStoreFromCollection;
mod cert_retrieve_logo_or_biometric_info;
pub use cert_retrieve_logo_or_biometric_info::CertRetrieveLogoOrBiometricInfo;
mod cert_save_store;
pub use cert_save_store::CertSaveStore;
mod cert_select_certificate;
pub use cert_select_certificate::CertSelectCertificate;
mod cert_select_certificate_chains;
pub use cert_select_certificate_chains::CertSelectCertificateChains;
mod cert_serialize_crl_store_element;
pub use cert_serialize_crl_store_element::CertSerializeCRLStoreElement;
mod cert_serialize_ctl_store_element;
pub use cert_serialize_ctl_store_element::CertSerializeCTLStoreElement;
mod cert_serialize_certificate_store_element;
pub use cert_serialize_certificate_store_element::CertSerializeCertificateStoreElement;
mod cert_set_crl_context_property;
pub use cert_set_crl_context_property::CertSetCRLContextProperty;
mod cert_set_ctl_context_property;
pub use cert_set_ctl_context_property::CertSetCTLContextProperty;
mod cert_set_certificate_context_properties_from_ctl_entry;
pub use cert_set_certificate_context_properties_from_ctl_entry::CertSetCertificateContextPropertiesFromCTLEntry;
mod cert_set_certificate_context_property;
pub use cert_set_certificate_context_property::CertSetCertificateContextProperty;
mod cert_set_enhanced_key_usage;
pub use cert_set_enhanced_key_usage::CertSetEnhancedKeyUsage;
mod cert_set_store_property;
pub use cert_set_store_property::CertSetStoreProperty;
mod cert_srv_backup_close;
pub use cert_srv_backup_close::CertSrvBackupClose;
mod cert_srv_backup_end;
pub use cert_srv_backup_end::CertSrvBackupEnd;
mod cert_srv_backup_free;
pub use cert_srv_backup_free::CertSrvBackupFree;
mod cert_srv_backup_get_backup_logs_w;
pub use cert_srv_backup_get_backup_logs_w::CertSrvBackupGetBackupLogsW;
mod cert_srv_backup_get_database_names_w;
pub use cert_srv_backup_get_database_names_w::CertSrvBackupGetDatabaseNamesW;
mod cert_srv_backup_get_dynamic_file_list_w;
pub use cert_srv_backup_get_dynamic_file_list_w::CertSrvBackupGetDynamicFileListW;
mod cert_srv_backup_open_file_w;
pub use cert_srv_backup_open_file_w::CertSrvBackupOpenFileW;
mod cert_srv_backup_prepare_w;
pub use cert_srv_backup_prepare_w::CertSrvBackupPrepareW;
mod cert_srv_backup_read;
pub use cert_srv_backup_read::CertSrvBackupRead;
mod cert_srv_backup_truncate_logs;
pub use cert_srv_backup_truncate_logs::CertSrvBackupTruncateLogs;
mod cert_srv_is_server_online_w;
pub use cert_srv_is_server_online_w::CertSrvIsServerOnlineW;
mod cert_srv_restore_end;
pub use cert_srv_restore_end::CertSrvRestoreEnd;
mod cert_srv_restore_get_database_locations_w;
pub use cert_srv_restore_get_database_locations_w::CertSrvRestoreGetDatabaseLocationsW;
mod cert_srv_restore_prepare_w;
pub use cert_srv_restore_prepare_w::CertSrvRestorePrepareW;
mod cert_srv_restore_register_complete;
pub use cert_srv_restore_register_complete::CertSrvRestoreRegisterComplete;
mod cert_srv_restore_register_through_file;
pub use cert_srv_restore_register_through_file::CertSrvRestoreRegisterThroughFile;
mod cert_srv_restore_register_w;
pub use cert_srv_restore_register_w::CertSrvRestoreRegisterW;
mod cert_srv_server_control_w;
pub use cert_srv_server_control_w::CertSrvServerControlW;
mod cert_str_to_name;
pub use cert_str_to_name::CertStrToName;
mod cert_unregister_physical_store;
pub use cert_unregister_physical_store::CertUnregisterPhysicalStore;
mod cert_unregister_system_store;
pub use cert_unregister_system_store::CertUnregisterSystemStore;
mod cert_verify_crl_revocation;
pub use cert_verify_crl_revocation::CertVerifyCRLRevocation;
mod cert_verify_crl_time_validity;
pub use cert_verify_crl_time_validity::CertVerifyCRLTimeValidity;
mod cert_verify_ctl_usage;
pub use cert_verify_ctl_usage::CertVerifyCTLUsage;
mod cert_verify_certificate_chain_policy;
pub use cert_verify_certificate_chain_policy::CertVerifyCertificateChainPolicy;
mod cert_verify_revocation;
pub use cert_verify_revocation::CertVerifyRevocation;
mod cert_verify_subject_certificate_context;
pub use cert_verify_subject_certificate_context::CertVerifySubjectCertificateContext;
mod cert_verify_time_validity;
pub use cert_verify_time_validity::CertVerifyTimeValidity;
mod cert_verify_validity_nesting;
pub use cert_verify_validity_nesting::CertVerifyValidityNesting;
mod crypt_acquire_context;
pub use crypt_acquire_context::CryptAcquireContext;
mod crypt_cat_admin_acquire_context;
pub use crypt_cat_admin_acquire_context::CryptCATAdminAcquireContext;
mod crypt_cat_admin_acquire_context2;
pub use crypt_cat_admin_acquire_context2::CryptCATAdminAcquireContext2;
mod crypt_cat_admin_add_catalog;
pub use crypt_cat_admin_add_catalog::CryptCATAdminAddCatalog;
mod crypt_cat_admin_calc_hash_from_file_handle;
pub use crypt_cat_admin_calc_hash_from_file_handle::CryptCATAdminCalcHashFromFileHandle;
mod crypt_cat_admin_calc_hash_from_file_handle2;
pub use crypt_cat_admin_calc_hash_from_file_handle2::CryptCATAdminCalcHashFromFileHandle2;
mod crypt_cat_admin_enum_catalog_from_hash;
pub use crypt_cat_admin_enum_catalog_from_hash::CryptCATAdminEnumCatalogFromHash;
mod crypt_cat_admin_release_catalog_context;
pub use crypt_cat_admin_release_catalog_context::CryptCATAdminReleaseCatalogContext;
mod crypt_cat_admin_release_context;
pub use crypt_cat_admin_release_context::CryptCATAdminReleaseContext;
mod crypt_cat_admin_remove_catalog;
pub use crypt_cat_admin_remove_catalog::CryptCATAdminRemoveCatalog;
mod crypt_cat_admin_resolve_catalog_path;
pub use crypt_cat_admin_resolve_catalog_path::CryptCATAdminResolveCatalogPath;
mod crypt_catcdf_close;
pub use crypt_catcdf_close::CryptCATCDFClose;
mod crypt_catcdf_enum_attributes_with_cdf_tag;
pub use crypt_catcdf_enum_attributes_with_cdf_tag::CryptCATCDFEnumAttributesWithCDFTag;
mod crypt_catcdf_enum_cat_attributes;
pub use crypt_catcdf_enum_cat_attributes::CryptCATCDFEnumCatAttributes;
mod crypt_catcdf_enum_members_by_cdf_tag_ex;
pub use crypt_catcdf_enum_members_by_cdf_tag_ex::CryptCATCDFEnumMembersByCDFTagEx;
mod crypt_catcdf_open;
pub use crypt_catcdf_open::CryptCATCDFOpen;
mod crypt_cat_catalog_info_from_context;
pub use crypt_cat_catalog_info_from_context::CryptCATCatalogInfoFromContext;
mod crypt_cat_close;
pub use crypt_cat_close::CryptCATClose;
mod crypt_cat_enumerate_attr;
pub use crypt_cat_enumerate_attr::CryptCATEnumerateAttr;
mod crypt_cat_enumerate_cat_attr;
pub use crypt_cat_enumerate_cat_attr::CryptCATEnumerateCatAttr;
mod crypt_cat_enumerate_member;
pub use crypt_cat_enumerate_member::CryptCATEnumerateMember;
mod crypt_cat_get_attr_info;
pub use crypt_cat_get_attr_info::CryptCATGetAttrInfo;
mod crypt_cat_get_member_info;
pub use crypt_cat_get_member_info::CryptCATGetMemberInfo;
mod crypt_cat_open;
pub use crypt_cat_open::CryptCATOpen;
mod crypt_context_add_ref;
pub use crypt_context_add_ref::CryptContextAddRef;
mod crypt_create_hash;
pub use crypt_create_hash::CryptCreateHash;
mod crypt_create_key_identifier_from_csp;
pub use crypt_create_key_identifier_from_csp::CryptCreateKeyIdentifierFromCSP;
mod crypt_decode_message;
pub use crypt_decode_message::CryptDecodeMessage;
mod crypt_decode_object;
pub use crypt_decode_object::CryptDecodeObject;
mod crypt_decrypt;
pub use crypt_decrypt::CryptDecrypt;
mod crypt_decrypt_and_verify_message_signature;
pub use crypt_decrypt_and_verify_message_signature::CryptDecryptAndVerifyMessageSignature;
mod crypt_decrypt_message;
pub use crypt_decrypt_message::CryptDecryptMessage;
mod crypt_derive_key;
pub use crypt_derive_key::CryptDeriveKey;
mod crypt_destroy_hash;
pub use crypt_destroy_hash::CryptDestroyHash;
mod crypt_destroy_key;
pub use crypt_destroy_key::CryptDestroyKey;
mod crypt_duplicate_hash;
pub use crypt_duplicate_hash::CryptDuplicateHash;
mod crypt_duplicate_key;
pub use crypt_duplicate_key::CryptDuplicateKey;
mod crypt_encode_object;
pub use crypt_encode_object::CryptEncodeObject;
mod crypt_encode_object_ex;
pub use crypt_encode_object_ex::CryptEncodeObjectEx;
mod crypt_encrypt;
pub use crypt_encrypt::CryptEncrypt;
mod crypt_encrypt_message;
pub use crypt_encrypt_message::CryptEncryptMessage;
mod crypt_enum_key_identifier_properties;
pub use crypt_enum_key_identifier_properties::CryptEnumKeyIdentifierProperties;
mod crypt_enum_oid_function;
pub use crypt_enum_oid_function::CryptEnumOIDFunction;
mod crypt_enum_oid_info;
pub use crypt_enum_oid_info::CryptEnumOIDInfo;
mod crypt_enum_provider_types;
pub use crypt_enum_provider_types::CryptEnumProviderTypes;
mod crypt_enum_providers;
pub use crypt_enum_providers::CryptEnumProviders;
mod crypt_export_key;
pub use crypt_export_key::CryptExportKey;
mod crypt_export_pkcs8;
pub use crypt_export_pkcs8::CryptExportPKCS8;
mod crypt_export_pkcs8_ex;
pub use crypt_export_pkcs8_ex::CryptExportPKCS8Ex;
mod crypt_export_public_key_info;
pub use crypt_export_public_key_info::CryptExportPublicKeyInfo;
mod crypt_export_public_key_info_ex;
pub use crypt_export_public_key_info_ex::CryptExportPublicKeyInfoEx;
mod crypt_export_public_key_info_from_b_crypt_key_handle;
pub use crypt_export_public_key_info_from_b_crypt_key_handle::CryptExportPublicKeyInfoFromBCryptKeyHandle;
mod crypt_find_certificate_key_prov_info;
pub use crypt_find_certificate_key_prov_info::CryptFindCertificateKeyProvInfo;
mod crypt_find_localized_name;
pub use crypt_find_localized_name::CryptFindLocalizedName;
mod crypt_find_oid_info;
pub use crypt_find_oid_info::CryptFindOIDInfo;
mod crypt_format_object;
pub use crypt_format_object::CryptFormatObject;
mod crypt_free_oid_function_address;
pub use crypt_free_oid_function_address::CryptFreeOIDFunctionAddress;
mod crypt_gen_key;
pub use crypt_gen_key::CryptGenKey;
mod crypt_gen_random;
pub use crypt_gen_random::CryptGenRandom;
mod crypt_get_default_oid_dll_list;
pub use crypt_get_default_oid_dll_list::CryptGetDefaultOIDDllList;
mod crypt_get_default_oid_function_address;
pub use crypt_get_default_oid_function_address::CryptGetDefaultOIDFunctionAddress;
mod crypt_get_default_provider;
pub use crypt_get_default_provider::CryptGetDefaultProvider;
mod crypt_get_hash_param;
pub use crypt_get_hash_param::CryptGetHashParam;
mod crypt_get_key_identifier_property;
pub use crypt_get_key_identifier_property::CryptGetKeyIdentifierProperty;
mod crypt_get_key_param;
pub use crypt_get_key_param::CryptGetKeyParam;
mod crypt_get_message_certificates;
pub use crypt_get_message_certificates::CryptGetMessageCertificates;
mod crypt_get_message_signer_count;
pub use crypt_get_message_signer_count::CryptGetMessageSignerCount;
mod crypt_get_oid_function_address;
pub use crypt_get_oid_function_address::CryptGetOIDFunctionAddress;
mod crypt_get_oid_function_value;
pub use crypt_get_oid_function_value::CryptGetOIDFunctionValue;
mod crypt_get_object_url;
pub use crypt_get_object_url::CryptGetObjectUrl;
mod crypt_get_prov_param;
pub use crypt_get_prov_param::CryptGetProvParam;
mod crypt_get_user_key;
pub use crypt_get_user_key::CryptGetUserKey;
mod crypt_hash_certificate;
pub use crypt_hash_certificate::CryptHashCertificate;
mod crypt_hash_certificate2;
pub use crypt_hash_certificate2::CryptHashCertificate2;
mod crypt_hash_data;
pub use crypt_hash_data::CryptHashData;
mod crypt_hash_message;
pub use crypt_hash_message::CryptHashMessage;
mod crypt_hash_public_key_info;
pub use crypt_hash_public_key_info::CryptHashPublicKeyInfo;
mod crypt_hash_session_key;
pub use crypt_hash_session_key::CryptHashSessionKey;
mod crypt_hash_to_be_signed;
pub use crypt_hash_to_be_signed::CryptHashToBeSigned;
mod crypt_import_key;
pub use crypt_import_key::CryptImportKey;
mod crypt_import_pkcs8;
pub use crypt_import_pkcs8::CryptImportPKCS8;
mod crypt_import_public_key_info;
pub use crypt_import_public_key_info::CryptImportPublicKeyInfo;
mod crypt_import_public_key_info_ex;
pub use crypt_import_public_key_info_ex::CryptImportPublicKeyInfoEx;
mod crypt_import_public_key_info_ex2;
pub use crypt_import_public_key_info_ex2::CryptImportPublicKeyInfoEx2;
mod crypt_init_oid_function_set;
pub use crypt_init_oid_function_set::CryptInitOIDFunctionSet;
mod crypt_install_default_context;
pub use crypt_install_default_context::CryptInstallDefaultContext;
mod crypt_install_oid_function_address;
pub use crypt_install_oid_function_address::CryptInstallOIDFunctionAddress;
mod crypt_mem_alloc;
pub use crypt_mem_alloc::CryptMemAlloc;
mod crypt_mem_free;
pub use crypt_mem_free::CryptMemFree;
mod crypt_mem_realloc;
pub use crypt_mem_realloc::CryptMemRealloc;
mod crypt_msg_calculate_encoded_length;
pub use crypt_msg_calculate_encoded_length::CryptMsgCalculateEncodedLength;
mod crypt_msg_close;
pub use crypt_msg_close::CryptMsgClose;
mod crypt_msg_control;
pub use crypt_msg_control::CryptMsgControl;
mod crypt_msg_countersign;
pub use crypt_msg_countersign::CryptMsgCountersign;
mod crypt_msg_countersign_encoded;
pub use crypt_msg_countersign_encoded::CryptMsgCountersignEncoded;
mod crypt_msg_duplicate;
pub use crypt_msg_duplicate::CryptMsgDuplicate;
mod crypt_msg_encode_and_sign_ctl;
pub use crypt_msg_encode_and_sign_ctl::CryptMsgEncodeAndSignCTL;
mod crypt_msg_get_and_verify_signer;
pub use crypt_msg_get_and_verify_signer::CryptMsgGetAndVerifySigner;
mod crypt_msg_get_param;
pub use crypt_msg_get_param::CryptMsgGetParam;
mod crypt_msg_open_to_decode;
pub use crypt_msg_open_to_decode::CryptMsgOpenToDecode;
mod crypt_msg_open_to_encode;
pub use crypt_msg_open_to_encode::CryptMsgOpenToEncode;
mod crypt_msg_sign_ctl;
pub use crypt_msg_sign_ctl::CryptMsgSignCTL;
mod crypt_msg_update;
pub use crypt_msg_update::CryptMsgUpdate;
mod crypt_msg_verify_countersignature_encoded;
pub use crypt_msg_verify_countersignature_encoded::CryptMsgVerifyCountersignatureEncoded;
mod crypt_msg_verify_countersignature_encoded_ex;
pub use crypt_msg_verify_countersignature_encoded_ex::CryptMsgVerifyCountersignatureEncodedEx;
mod crypt_protect_data;
pub use crypt_protect_data::CryptProtectData;
mod crypt_protect_memory;
pub use crypt_protect_memory::CryptProtectMemory;
mod crypt_query_object;
pub use crypt_query_object::CryptQueryObject;
mod crypt_register_default_oid_function;
pub use crypt_register_default_oid_function::CryptRegisterDefaultOIDFunction;
mod crypt_register_oid_function;
pub use crypt_register_oid_function::CryptRegisterOIDFunction;
mod crypt_register_oid_info;
pub use crypt_register_oid_info::CryptRegisterOIDInfo;
mod crypt_release_context;
pub use crypt_release_context::CryptReleaseContext;
mod crypt_retrieve_object_by_url;
pub use crypt_retrieve_object_by_url::CryptRetrieveObjectByUrl;
mod crypt_sip_add_provider;
pub use crypt_sip_add_provider::CryptSIPAddProvider;
mod crypt_sip_create_indirect_data;
pub use crypt_sip_create_indirect_data::CryptSIPCreateIndirectData;
mod crypt_sip_get_caps;
pub use crypt_sip_get_caps::CryptSIPGetCaps;
mod crypt_sip_get_signed_data_msg;
pub use crypt_sip_get_signed_data_msg::CryptSIPGetSignedDataMsg;
mod crypt_sip_load;
pub use crypt_sip_load::CryptSIPLoad;
mod crypt_sip_put_signed_data_msg;
pub use crypt_sip_put_signed_data_msg::CryptSIPPutSignedDataMsg;
mod crypt_sip_remove_provider;
pub use crypt_sip_remove_provider::CryptSIPRemoveProvider;
mod crypt_sip_remove_signed_data_msg;
pub use crypt_sip_remove_signed_data_msg::CryptSIPRemoveSignedDataMsg;
mod crypt_sip_retrieve_subject_guid;
pub use crypt_sip_retrieve_subject_guid::CryptSIPRetrieveSubjectGuid;
mod crypt_sip_retrieve_subject_guid_for_catalog_file;
pub use crypt_sip_retrieve_subject_guid_for_catalog_file::CryptSIPRetrieveSubjectGuidForCatalogFile;
mod crypt_sip_verify_indirect_data;
pub use crypt_sip_verify_indirect_data::CryptSIPVerifyIndirectData;
mod crypt_set_hash_param;
pub use crypt_set_hash_param::CryptSetHashParam;
mod crypt_set_key_identifier_property;
pub use crypt_set_key_identifier_property::CryptSetKeyIdentifierProperty;
mod crypt_set_key_param;
pub use crypt_set_key_param::CryptSetKeyParam;
mod crypt_set_oid_function_value;
pub use crypt_set_oid_function_value::CryptSetOIDFunctionValue;
mod crypt_set_prov_param;
pub use crypt_set_prov_param::CryptSetProvParam;
mod crypt_set_provider;
pub use crypt_set_provider::CryptSetProvider;
mod crypt_set_provider_ex;
pub use crypt_set_provider_ex::CryptSetProviderEx;
mod crypt_sign_and_encode_certificate;
pub use crypt_sign_and_encode_certificate::CryptSignAndEncodeCertificate;
mod crypt_sign_and_encrypt_message;
pub use crypt_sign_and_encrypt_message::CryptSignAndEncryptMessage;
mod crypt_sign_certificate;
pub use crypt_sign_certificate::CryptSignCertificate;
mod crypt_sign_hash;
pub use crypt_sign_hash::CryptSignHash;
mod crypt_sign_message;
pub use crypt_sign_message::CryptSignMessage;
mod crypt_sign_message_with_key;
pub use crypt_sign_message_with_key::CryptSignMessageWithKey;
mod crypt_string_to_binary;
pub use crypt_string_to_binary::CryptStringToBinary;
mod crypt_ui_dlg_cert_mgr;
pub use crypt_ui_dlg_cert_mgr::CryptUIDlgCertMgr;
mod crypt_ui_dlg_select_certificate;
pub use crypt_ui_dlg_select_certificate::CryptUIDlgSelectCertificate;
mod crypt_ui_dlg_select_certificate_from_store;
pub use crypt_ui_dlg_select_certificate_from_store::CryptUIDlgSelectCertificateFromStore;
mod crypt_ui_dlg_view_certificate;
pub use crypt_ui_dlg_view_certificate::CryptUIDlgViewCertificate;
mod crypt_ui_dlg_view_context;
pub use crypt_ui_dlg_view_context::CryptUIDlgViewContext;
mod crypt_ui_dlg_view_signer_info;
pub use crypt_ui_dlg_view_signer_info::CryptUIDlgViewSignerInfo;
mod crypt_ui_wiz_digital_sign;
pub use crypt_ui_wiz_digital_sign::CryptUIWizDigitalSign;
mod crypt_ui_wiz_export;
pub use crypt_ui_wiz_export::CryptUIWizExport;
mod crypt_ui_wiz_free_digital_sign_context;
pub use crypt_ui_wiz_free_digital_sign_context::CryptUIWizFreeDigitalSignContext;
mod crypt_ui_wiz_import;
pub use crypt_ui_wiz_import::CryptUIWizImport;
mod crypt_uninstall_default_context;
pub use crypt_uninstall_default_context::CryptUninstallDefaultContext;
mod crypt_unprotect_data;
pub use crypt_unprotect_data::CryptUnprotectData;
mod crypt_unprotect_memory;
pub use crypt_unprotect_memory::CryptUnprotectMemory;
mod crypt_unregister_default_oid_function;
pub use crypt_unregister_default_oid_function::CryptUnregisterDefaultOIDFunction;
mod crypt_unregister_oid_function;
pub use crypt_unregister_oid_function::CryptUnregisterOIDFunction;
mod crypt_unregister_oid_info;
pub use crypt_unregister_oid_info::CryptUnregisterOIDInfo;
mod crypt_update_protected_state;
pub use crypt_update_protected_state::CryptUpdateProtectedState;
mod crypt_verify_certificate_signature;
pub use crypt_verify_certificate_signature::CryptVerifyCertificateSignature;
mod crypt_verify_certificate_signature_ex;
pub use crypt_verify_certificate_signature_ex::CryptVerifyCertificateSignatureEx;
mod crypt_verify_detached_message_hash;
pub use crypt_verify_detached_message_hash::CryptVerifyDetachedMessageHash;
mod crypt_verify_detached_message_signature;
pub use crypt_verify_detached_message_signature::CryptVerifyDetachedMessageSignature;
mod crypt_verify_message_hash;
pub use crypt_verify_message_hash::CryptVerifyMessageHash;
mod crypt_verify_message_signature;
pub use crypt_verify_message_signature::CryptVerifyMessageSignature;
mod crypt_verify_message_signature_with_key;
pub use crypt_verify_message_signature_with_key::CryptVerifyMessageSignatureWithKey;
mod crypt_verify_signature;
pub use crypt_verify_signature::CryptVerifySignature;
mod crypt_xml_add_object;
pub use crypt_xml_add_object::CryptXmlAddObject;
mod crypt_xml_close;
pub use crypt_xml_close::CryptXmlClose;
mod crypt_xml_create_reference;
pub use crypt_xml_create_reference::CryptXmlCreateReference;
mod crypt_xml_digest_reference;
pub use crypt_xml_digest_reference::CryptXmlDigestReference;
mod crypt_xml_encode;
pub use crypt_xml_encode::CryptXmlEncode;
mod crypt_xml_get_algorithm_info;
pub use crypt_xml_get_algorithm_info::CryptXmlGetAlgorithmInfo;
mod crypt_xml_get_doc_context;
pub use crypt_xml_get_doc_context::CryptXmlGetDocContext;
mod crypt_xml_get_reference;
pub use crypt_xml_get_reference::CryptXmlGetReference;
mod crypt_xml_get_signature;
pub use crypt_xml_get_signature::CryptXmlGetSignature;
mod crypt_xml_get_status;
pub use crypt_xml_get_status::CryptXmlGetStatus;
mod crypt_xml_get_transforms;
pub use crypt_xml_get_transforms::CryptXmlGetTransforms;
mod crypt_xml_import_public_key;
pub use crypt_xml_import_public_key::CryptXmlImportPublicKey;
mod crypt_xml_open_to_decode;
pub use crypt_xml_open_to_decode::CryptXmlOpenToDecode;
mod crypt_xml_open_to_encode;
pub use crypt_xml_open_to_encode::CryptXmlOpenToEncode;
mod crypt_xml_set_hmac_secret;
pub use crypt_xml_set_hmac_secret::CryptXmlSetHMACSecret;
mod crypt_xml_sign;
pub use crypt_xml_sign::CryptXmlSign;
mod crypt_xml_verify_signature;
pub use crypt_xml_verify_signature::CryptXmlVerifySignature;
mod pfx_export_cert_store;
pub use pfx_export_cert_store::PFXExportCertStore;
mod pfx_export_cert_store_ex;
pub use pfx_export_cert_store_ex::PFXExportCertStoreEx;
mod pfx_import_cert_store;
pub use pfx_import_cert_store::PFXImportCertStore;
mod pfx_is_pfx_blob;
pub use pfx_is_pfx_blob::PFXIsPFXBlob;
mod pfx_verify_password;
pub use pfx_verify_password::PFXVerifyPassword;
mod cert_close_store;
pub use cert_close_store::CertCloseStore;
mod cert_find_certificate_in_store;
pub use cert_find_certificate_in_store::CertFindCertificateInStore;
mod cert_free_certificate_context;
pub use cert_free_certificate_context::CertFreeCertificateContext;
mod cert_open_store;
pub use cert_open_store::CertOpenStore;
pub use cert_open_system_store::CertOpenSystemStoreA;
pub use cert_open_system_store::CertOpenSystemStoreW;
mod crypt_binary_to_string;
use crypt_binary_to_string::CryptBinaryToStringA;
use crypt_binary_to_string::CryptBinaryToStringW;
mod crypt_decode_object_ex;
use crypt_decode_object_ex::CryptDecodeObjectEx;
pub use crypt_string_to_binary::CryptStringToBinaryA;
use crypt_string_to_binary::CryptStringToBinaryW;
mod pki_initialize_critical_section;
use pki_initialize_critical_section::PkiInitializeCriticalSection;

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
        "CertAddCRLContextToStore" => CertAddCRLContextToStore(emu),
        "CertAddCRLLinkToStore" => CertAddCRLLinkToStore(emu),
        "CertAddCTLContextToStore" => CertAddCTLContextToStore(emu),
        "CertAddCTLLinkToStore" => CertAddCTLLinkToStore(emu),
        "CertAddCertificateContextToStore" => CertAddCertificateContextToStore(emu),
        "CertAddCertificateLinkToStore" => CertAddCertificateLinkToStore(emu),
        "CertAddEncodedCRLToStore" => CertAddEncodedCRLToStore(emu),
        "CertAddEncodedCTLToStore" => CertAddEncodedCTLToStore(emu),
        "CertAddEncodedCertificateToStore" => CertAddEncodedCertificateToStore(emu),
        "CertAddEnhancedKeyUsageIdentifier" => CertAddEnhancedKeyUsageIdentifier(emu),
        "CertAddRefServerOcspResponse" => CertAddRefServerOcspResponse(emu),
        "CertAddRefServerOcspResponseContext" => CertAddRefServerOcspResponseContext(emu),
        "CertAddSerializedElementToStore" => CertAddSerializedElementToStore(emu),
        "CertAddStoreToCollection" => CertAddStoreToCollection(emu),
        "CertAlgIdToOID" => CertAlgIdToOID(emu),
        "CertCloseServerOcspResponse" => CertCloseServerOcspResponse(emu),
        "CertCompareCertificate" => CertCompareCertificate(emu),
        "CertCompareCertificateName" => CertCompareCertificateName(emu),
        "CertCompareIntegerBlob" => CertCompareIntegerBlob(emu),
        "CertComparePublicKeyInfo" => CertComparePublicKeyInfo(emu),
        "CertControlStore" => CertControlStore(emu),
        "CertCreateCRLContext" => CertCreateCRLContext(emu),
        "CertCreateCTLContext" => CertCreateCTLContext(emu),
        "CertCreateCTLEntryFromCertificateContextProperties" => {
            CertCreateCTLEntryFromCertificateContextProperties(emu)
        }
        "CertCreateCertificateChainEngine" => CertCreateCertificateChainEngine(emu),
        "CertCreateCertificateContext" => CertCreateCertificateContext(emu),
        "CertCreateContext" => CertCreateContext(emu),
        "CertCreateSelfSignCertificate" => CertCreateSelfSignCertificate(emu),
        "CertDeleteCRLFromStore" => CertDeleteCRLFromStore(emu),
        "CertDeleteCTLFromStore" => CertDeleteCTLFromStore(emu),
        "CertDeleteCertificateFromStore" => CertDeleteCertificateFromStore(emu),
        "CertDuplicateCRLContext" => CertDuplicateCRLContext(emu),
        "CertDuplicateCTLContext" => CertDuplicateCTLContext(emu),
        "CertDuplicateCertificateChain" => CertDuplicateCertificateChain(emu),
        "CertDuplicateCertificateContext" => CertDuplicateCertificateContext(emu),
        "CertDuplicateStore" => CertDuplicateStore(emu),
        "CertEnumCRLContextProperties" => CertEnumCRLContextProperties(emu),
        "CertEnumCRLsInStore" => CertEnumCRLsInStore(emu),
        "CertEnumCTLContextProperties" => CertEnumCTLContextProperties(emu),
        "CertEnumCTLsInStore" => CertEnumCTLsInStore(emu),
        "CertEnumCertificateContextProperties" => CertEnumCertificateContextProperties(emu),
        "CertEnumCertificatesInStore" => CertEnumCertificatesInStore(emu),
        "CertEnumPhysicalStore" => CertEnumPhysicalStore(emu),
        "CertEnumSubjectInSortedCTL" => CertEnumSubjectInSortedCTL(emu),
        "CertEnumSystemStore" => CertEnumSystemStore(emu),
        "CertEnumSystemStoreLocation" => CertEnumSystemStoreLocation(emu),
        "CertFindAttribute" => CertFindAttribute(emu),
        "CertFindCRLInStore" => CertFindCRLInStore(emu),
        "CertFindCTLInStore" => CertFindCTLInStore(emu),
        "CertFindCertificateInCRL" => CertFindCertificateInCRL(emu),
        "CertFindChainInStore" => CertFindChainInStore(emu),
        "CertFindExtension" => CertFindExtension(emu),
        "CertFindRDNAttr" => CertFindRDNAttr(emu),
        "CertFindSubjectInCTL" => CertFindSubjectInCTL(emu),
        "CertFindSubjectInSortedCTL" => CertFindSubjectInSortedCTL(emu),
        "CertFreeCRLContext" => CertFreeCRLContext(emu),
        "CertFreeCTLContext" => CertFreeCTLContext(emu),
        "CertFreeCertificateChain" => CertFreeCertificateChain(emu),
        "CertFreeCertificateChainEngine" => CertFreeCertificateChainEngine(emu),
        "CertFreeCertificateChainList" => CertFreeCertificateChainList(emu),
        "CertGetCRLContextProperty" => CertGetCRLContextProperty(emu),
        "CertGetCRLFromStore" => CertGetCRLFromStore(emu),
        "CertGetCTLContextProperty" => CertGetCTLContextProperty(emu),
        "CertGetCertificateChain" => CertGetCertificateChain(emu),
        "CertGetCertificateContextProperty" => CertGetCertificateContextProperty(emu),
        "CertGetEnhancedKeyUsage" => CertGetEnhancedKeyUsage(emu),
        "CertGetIntendedKeyUsage" => CertGetIntendedKeyUsage(emu),
        "CertGetIssuerCertificateFromStore" => CertGetIssuerCertificateFromStore(emu),
        "CertGetNameString" => CertGetNameString(emu),
        "CertGetPublicKeyLength" => CertGetPublicKeyLength(emu),
        "CertGetServerOcspResponseContext" => CertGetServerOcspResponseContext(emu),
        "CertGetStoreProperty" => CertGetStoreProperty(emu),
        "CertGetSubjectCertificateFromStore" => CertGetSubjectCertificateFromStore(emu),
        "CertGetValidUsages" => CertGetValidUsages(emu),
        "CertIsRDNAttrsInCertificateName" => CertIsRDNAttrsInCertificateName(emu),
        "CertIsStrongHashToSign" => CertIsStrongHashToSign(emu),
        "CertIsValidCRLForCertificate" => CertIsValidCRLForCertificate(emu),
        "CertModifyCertificatesToTrust" => CertModifyCertificatesToTrust(emu),
        "CertNameToStr" => CertNameToStr(emu),
        "CertOIDToAlgId" => CertOIDToAlgId(emu),
        "CertOpenServerOcspResponse" => CertOpenServerOcspResponse(emu),
        "CertOpenSystemStore" => CertOpenSystemStore(emu),
        "CertRDNValueToStr" => CertRDNValueToStr(emu),
        "CertRegisterPhysicalStore" => CertRegisterPhysicalStore(emu),
        "CertRegisterSystemStore" => CertRegisterSystemStore(emu),
        "CertRemoveEnhancedKeyUsageIdentifier" => CertRemoveEnhancedKeyUsageIdentifier(emu),
        "CertRemoveStoreFromCollection" => CertRemoveStoreFromCollection(emu),
        "CertRetrieveLogoOrBiometricInfo" => CertRetrieveLogoOrBiometricInfo(emu),
        "CertSaveStore" => CertSaveStore(emu),
        "CertSelectCertificate" => CertSelectCertificate(emu),
        "CertSelectCertificateChains" => CertSelectCertificateChains(emu),
        "CertSerializeCRLStoreElement" => CertSerializeCRLStoreElement(emu),
        "CertSerializeCTLStoreElement" => CertSerializeCTLStoreElement(emu),
        "CertSerializeCertificateStoreElement" => CertSerializeCertificateStoreElement(emu),
        "CertSetCRLContextProperty" => CertSetCRLContextProperty(emu),
        "CertSetCTLContextProperty" => CertSetCTLContextProperty(emu),
        "CertSetCertificateContextPropertiesFromCTLEntry" => {
            CertSetCertificateContextPropertiesFromCTLEntry(emu)
        }
        "CertSetCertificateContextProperty" => CertSetCertificateContextProperty(emu),
        "CertSetEnhancedKeyUsage" => CertSetEnhancedKeyUsage(emu),
        "CertSetStoreProperty" => CertSetStoreProperty(emu),
        "CertSrvBackupClose" => CertSrvBackupClose(emu),
        "CertSrvBackupEnd" => CertSrvBackupEnd(emu),
        "CertSrvBackupFree" => CertSrvBackupFree(emu),
        "CertSrvBackupGetBackupLogsW" => CertSrvBackupGetBackupLogsW(emu),
        "CertSrvBackupGetDatabaseNamesW" => CertSrvBackupGetDatabaseNamesW(emu),
        "CertSrvBackupGetDynamicFileListW" => CertSrvBackupGetDynamicFileListW(emu),
        "CertSrvBackupOpenFileW" => CertSrvBackupOpenFileW(emu),
        "CertSrvBackupPrepareW" => CertSrvBackupPrepareW(emu),
        "CertSrvBackupRead" => CertSrvBackupRead(emu),
        "CertSrvBackupTruncateLogs" => CertSrvBackupTruncateLogs(emu),
        "CertSrvIsServerOnlineW" => CertSrvIsServerOnlineW(emu),
        "CertSrvRestoreEnd" => CertSrvRestoreEnd(emu),
        "CertSrvRestoreGetDatabaseLocationsW" => CertSrvRestoreGetDatabaseLocationsW(emu),
        "CertSrvRestorePrepareW" => CertSrvRestorePrepareW(emu),
        "CertSrvRestoreRegisterComplete" => CertSrvRestoreRegisterComplete(emu),
        "CertSrvRestoreRegisterThroughFile" => CertSrvRestoreRegisterThroughFile(emu),
        "CertSrvRestoreRegisterW" => CertSrvRestoreRegisterW(emu),
        "CertSrvServerControlW" => CertSrvServerControlW(emu),
        "CertStrToName" => CertStrToName(emu),
        "CertUnregisterPhysicalStore" => CertUnregisterPhysicalStore(emu),
        "CertUnregisterSystemStore" => CertUnregisterSystemStore(emu),
        "CertVerifyCRLRevocation" => CertVerifyCRLRevocation(emu),
        "CertVerifyCRLTimeValidity" => CertVerifyCRLTimeValidity(emu),
        "CertVerifyCTLUsage" => CertVerifyCTLUsage(emu),
        "CertVerifyCertificateChainPolicy" => CertVerifyCertificateChainPolicy(emu),
        "CertVerifyRevocation" => CertVerifyRevocation(emu),
        "CertVerifySubjectCertificateContext" => CertVerifySubjectCertificateContext(emu),
        "CertVerifyTimeValidity" => CertVerifyTimeValidity(emu),
        "CertVerifyValidityNesting" => CertVerifyValidityNesting(emu),
        "CryptAcquireContext" => CryptAcquireContext(emu),
        "CryptBinaryToString" => CryptBinaryToStringA(emu),
        "CryptCATAdminAcquireContext" => CryptCATAdminAcquireContext(emu),
        "CryptCATAdminAcquireContext2" => CryptCATAdminAcquireContext2(emu),
        "CryptCATAdminAddCatalog" => CryptCATAdminAddCatalog(emu),
        "CryptCATAdminCalcHashFromFileHandle" => CryptCATAdminCalcHashFromFileHandle(emu),
        "CryptCATAdminCalcHashFromFileHandle2" => CryptCATAdminCalcHashFromFileHandle2(emu),
        "CryptCATAdminEnumCatalogFromHash" => CryptCATAdminEnumCatalogFromHash(emu),
        "CryptCATAdminReleaseCatalogContext" => CryptCATAdminReleaseCatalogContext(emu),
        "CryptCATAdminReleaseContext" => CryptCATAdminReleaseContext(emu),
        "CryptCATAdminRemoveCatalog" => CryptCATAdminRemoveCatalog(emu),
        "CryptCATAdminResolveCatalogPath" => CryptCATAdminResolveCatalogPath(emu),
        "CryptCATCDFClose" => CryptCATCDFClose(emu),
        "CryptCATCDFEnumAttributesWithCDFTag" => CryptCATCDFEnumAttributesWithCDFTag(emu),
        "CryptCATCDFEnumCatAttributes" => CryptCATCDFEnumCatAttributes(emu),
        "CryptCATCDFEnumMembersByCDFTagEx" => CryptCATCDFEnumMembersByCDFTagEx(emu),
        "CryptCATCDFOpen" => CryptCATCDFOpen(emu),
        "CryptCATCatalogInfoFromContext" => CryptCATCatalogInfoFromContext(emu),
        "CryptCATClose" => CryptCATClose(emu),
        "CryptCATEnumerateAttr" => CryptCATEnumerateAttr(emu),
        "CryptCATEnumerateCatAttr" => CryptCATEnumerateCatAttr(emu),
        "CryptCATEnumerateMember" => CryptCATEnumerateMember(emu),
        "CryptCATGetAttrInfo" => CryptCATGetAttrInfo(emu),
        "CryptCATGetMemberInfo" => CryptCATGetMemberInfo(emu),
        "CryptCATOpen" => CryptCATOpen(emu),
        "CryptContextAddRef" => CryptContextAddRef(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptCreateKeyIdentifierFromCSP" => CryptCreateKeyIdentifierFromCSP(emu),
        "CryptDecodeMessage" => CryptDecodeMessage(emu),
        "CryptDecodeObject" => CryptDecodeObject(emu),
        "CryptDecrypt" => CryptDecrypt(emu),
        "CryptDecryptAndVerifyMessageSignature" => CryptDecryptAndVerifyMessageSignature(emu),
        "CryptDecryptMessage" => CryptDecryptMessage(emu),
        "CryptDeriveKey" => CryptDeriveKey(emu),
        "CryptDestroyHash" => CryptDestroyHash(emu),
        "CryptDestroyKey" => CryptDestroyKey(emu),
        "CryptDuplicateHash" => CryptDuplicateHash(emu),
        "CryptDuplicateKey" => CryptDuplicateKey(emu),
        "CryptEncodeObject" => CryptEncodeObject(emu),
        "CryptEncodeObjectEx" => CryptEncodeObjectEx(emu),
        "CryptEncrypt" => CryptEncrypt(emu),
        "CryptEncryptMessage" => CryptEncryptMessage(emu),
        "CryptEnumKeyIdentifierProperties" => CryptEnumKeyIdentifierProperties(emu),
        "CryptEnumOIDFunction" => CryptEnumOIDFunction(emu),
        "CryptEnumOIDInfo" => CryptEnumOIDInfo(emu),
        "CryptEnumProviderTypes" => CryptEnumProviderTypes(emu),
        "CryptEnumProviders" => CryptEnumProviders(emu),
        "CryptExportKey" => CryptExportKey(emu),
        "CryptExportPKCS8" => CryptExportPKCS8(emu),
        "CryptExportPKCS8Ex" => CryptExportPKCS8Ex(emu),
        "CryptExportPublicKeyInfo" => CryptExportPublicKeyInfo(emu),
        "CryptExportPublicKeyInfoEx" => CryptExportPublicKeyInfoEx(emu),
        "CryptExportPublicKeyInfoFromBCryptKeyHandle" => {
            CryptExportPublicKeyInfoFromBCryptKeyHandle(emu)
        }
        "CryptFindCertificateKeyProvInfo" => CryptFindCertificateKeyProvInfo(emu),
        "CryptFindLocalizedName" => CryptFindLocalizedName(emu),
        "CryptFindOIDInfo" => CryptFindOIDInfo(emu),
        "CryptFormatObject" => CryptFormatObject(emu),
        "CryptFreeOIDFunctionAddress" => CryptFreeOIDFunctionAddress(emu),
        "CryptGenKey" => CryptGenKey(emu),
        "CryptGenRandom" => CryptGenRandom(emu),
        "CryptGetDefaultOIDDllList" => CryptGetDefaultOIDDllList(emu),
        "CryptGetDefaultOIDFunctionAddress" => CryptGetDefaultOIDFunctionAddress(emu),
        "CryptGetDefaultProvider" => CryptGetDefaultProvider(emu),
        "CryptGetHashParam" => CryptGetHashParam(emu),
        "CryptGetKeyIdentifierProperty" => CryptGetKeyIdentifierProperty(emu),
        "CryptGetKeyParam" => CryptGetKeyParam(emu),
        "CryptGetMessageCertificates" => CryptGetMessageCertificates(emu),
        "CryptGetMessageSignerCount" => CryptGetMessageSignerCount(emu),
        "CryptGetOIDFunctionAddress" => CryptGetOIDFunctionAddress(emu),
        "CryptGetOIDFunctionValue" => CryptGetOIDFunctionValue(emu),
        "CryptGetObjectUrl" => CryptGetObjectUrl(emu),
        "CryptGetProvParam" => CryptGetProvParam(emu),
        "CryptGetUserKey" => CryptGetUserKey(emu),
        "CryptHashCertificate" => CryptHashCertificate(emu),
        "CryptHashCertificate2" => CryptHashCertificate2(emu),
        "CryptHashData" => CryptHashData(emu),
        "CryptHashMessage" => CryptHashMessage(emu),
        "CryptHashPublicKeyInfo" => CryptHashPublicKeyInfo(emu),
        "CryptHashSessionKey" => CryptHashSessionKey(emu),
        "CryptHashToBeSigned" => CryptHashToBeSigned(emu),
        "CryptImportKey" => CryptImportKey(emu),
        "CryptImportPKCS8" => CryptImportPKCS8(emu),
        "CryptImportPublicKeyInfo" => CryptImportPublicKeyInfo(emu),
        "CryptImportPublicKeyInfoEx" => CryptImportPublicKeyInfoEx(emu),
        "CryptImportPublicKeyInfoEx2" => CryptImportPublicKeyInfoEx2(emu),
        "CryptInitOIDFunctionSet" => CryptInitOIDFunctionSet(emu),
        "CryptInstallDefaultContext" => CryptInstallDefaultContext(emu),
        "CryptInstallOIDFunctionAddress" => CryptInstallOIDFunctionAddress(emu),
        "CryptMemAlloc" => CryptMemAlloc(emu),
        "CryptMemFree" => CryptMemFree(emu),
        "CryptMemRealloc" => CryptMemRealloc(emu),
        "CryptMsgCalculateEncodedLength" => CryptMsgCalculateEncodedLength(emu),
        "CryptMsgClose" => CryptMsgClose(emu),
        "CryptMsgControl" => CryptMsgControl(emu),
        "CryptMsgCountersign" => CryptMsgCountersign(emu),
        "CryptMsgCountersignEncoded" => CryptMsgCountersignEncoded(emu),
        "CryptMsgDuplicate" => CryptMsgDuplicate(emu),
        "CryptMsgEncodeAndSignCTL" => CryptMsgEncodeAndSignCTL(emu),
        "CryptMsgGetAndVerifySigner" => CryptMsgGetAndVerifySigner(emu),
        "CryptMsgGetParam" => CryptMsgGetParam(emu),
        "CryptMsgOpenToDecode" => CryptMsgOpenToDecode(emu),
        "CryptMsgOpenToEncode" => CryptMsgOpenToEncode(emu),
        "CryptMsgSignCTL" => CryptMsgSignCTL(emu),
        "CryptMsgUpdate" => CryptMsgUpdate(emu),
        "CryptMsgVerifyCountersignatureEncoded" => CryptMsgVerifyCountersignatureEncoded(emu),
        "CryptMsgVerifyCountersignatureEncodedEx" => CryptMsgVerifyCountersignatureEncodedEx(emu),
        "CryptProtectData" => CryptProtectData(emu),
        "CryptProtectMemory" => CryptProtectMemory(emu),
        "CryptQueryObject" => CryptQueryObject(emu),
        "CryptRegisterDefaultOIDFunction" => CryptRegisterDefaultOIDFunction(emu),
        "CryptRegisterOIDFunction" => CryptRegisterOIDFunction(emu),
        "CryptRegisterOIDInfo" => CryptRegisterOIDInfo(emu),
        "CryptReleaseContext" => CryptReleaseContext(emu),
        "CryptRetrieveObjectByUrl" => CryptRetrieveObjectByUrl(emu),
        "CryptSIPAddProvider" => CryptSIPAddProvider(emu),
        "CryptSIPCreateIndirectData" => CryptSIPCreateIndirectData(emu),
        "CryptSIPGetCaps" => CryptSIPGetCaps(emu),
        "CryptSIPGetSignedDataMsg" => CryptSIPGetSignedDataMsg(emu),
        "CryptSIPLoad" => CryptSIPLoad(emu),
        "CryptSIPPutSignedDataMsg" => CryptSIPPutSignedDataMsg(emu),
        "CryptSIPRemoveProvider" => CryptSIPRemoveProvider(emu),
        "CryptSIPRemoveSignedDataMsg" => CryptSIPRemoveSignedDataMsg(emu),
        "CryptSIPRetrieveSubjectGuid" => CryptSIPRetrieveSubjectGuid(emu),
        "CryptSIPRetrieveSubjectGuidForCatalogFile" => {
            CryptSIPRetrieveSubjectGuidForCatalogFile(emu)
        }
        "CryptSIPVerifyIndirectData" => CryptSIPVerifyIndirectData(emu),
        "CryptSetHashParam" => CryptSetHashParam(emu),
        "CryptSetKeyIdentifierProperty" => CryptSetKeyIdentifierProperty(emu),
        "CryptSetKeyParam" => CryptSetKeyParam(emu),
        "CryptSetOIDFunctionValue" => CryptSetOIDFunctionValue(emu),
        "CryptSetProvParam" => CryptSetProvParam(emu),
        "CryptSetProvider" => CryptSetProvider(emu),
        "CryptSetProviderEx" => CryptSetProviderEx(emu),
        "CryptSignAndEncodeCertificate" => CryptSignAndEncodeCertificate(emu),
        "CryptSignAndEncryptMessage" => CryptSignAndEncryptMessage(emu),
        "CryptSignCertificate" => CryptSignCertificate(emu),
        "CryptSignHash" => CryptSignHash(emu),
        "CryptSignMessage" => CryptSignMessage(emu),
        "CryptSignMessageWithKey" => CryptSignMessageWithKey(emu),
        "CryptStringToBinary" => CryptStringToBinary(emu),
        "CryptUIDlgCertMgr" => CryptUIDlgCertMgr(emu),
        "CryptUIDlgSelectCertificate" => CryptUIDlgSelectCertificate(emu),
        "CryptUIDlgSelectCertificateFromStore" => CryptUIDlgSelectCertificateFromStore(emu),
        "CryptUIDlgViewCertificate" => CryptUIDlgViewCertificate(emu),
        "CryptUIDlgViewContext" => CryptUIDlgViewContext(emu),
        "CryptUIDlgViewSignerInfo" => CryptUIDlgViewSignerInfo(emu),
        "CryptUIWizDigitalSign" => CryptUIWizDigitalSign(emu),
        "CryptUIWizExport" => CryptUIWizExport(emu),
        "CryptUIWizFreeDigitalSignContext" => CryptUIWizFreeDigitalSignContext(emu),
        "CryptUIWizImport" => CryptUIWizImport(emu),
        "CryptUninstallDefaultContext" => CryptUninstallDefaultContext(emu),
        "CryptUnprotectData" => CryptUnprotectData(emu),
        "CryptUnprotectMemory" => CryptUnprotectMemory(emu),
        "CryptUnregisterDefaultOIDFunction" => CryptUnregisterDefaultOIDFunction(emu),
        "CryptUnregisterOIDFunction" => CryptUnregisterOIDFunction(emu),
        "CryptUnregisterOIDInfo" => CryptUnregisterOIDInfo(emu),
        "CryptUpdateProtectedState" => CryptUpdateProtectedState(emu),
        "CryptVerifyCertificateSignature" => CryptVerifyCertificateSignature(emu),
        "CryptVerifyCertificateSignatureEx" => CryptVerifyCertificateSignatureEx(emu),
        "CryptVerifyDetachedMessageHash" => CryptVerifyDetachedMessageHash(emu),
        "CryptVerifyDetachedMessageSignature" => CryptVerifyDetachedMessageSignature(emu),
        "CryptVerifyMessageHash" => CryptVerifyMessageHash(emu),
        "CryptVerifyMessageSignature" => CryptVerifyMessageSignature(emu),
        "CryptVerifyMessageSignatureWithKey" => CryptVerifyMessageSignatureWithKey(emu),
        "CryptVerifySignature" => CryptVerifySignature(emu),
        "CryptXmlAddObject" => CryptXmlAddObject(emu),
        "CryptXmlClose" => CryptXmlClose(emu),
        "CryptXmlCreateReference" => CryptXmlCreateReference(emu),
        "CryptXmlDigestReference" => CryptXmlDigestReference(emu),
        "CryptXmlEncode" => CryptXmlEncode(emu),
        "CryptXmlGetAlgorithmInfo" => CryptXmlGetAlgorithmInfo(emu),
        "CryptXmlGetDocContext" => CryptXmlGetDocContext(emu),
        "CryptXmlGetReference" => CryptXmlGetReference(emu),
        "CryptXmlGetSignature" => CryptXmlGetSignature(emu),
        "CryptXmlGetStatus" => CryptXmlGetStatus(emu),
        "CryptXmlGetTransforms" => CryptXmlGetTransforms(emu),
        "CryptXmlImportPublicKey" => CryptXmlImportPublicKey(emu),
        "CryptXmlOpenToDecode" => CryptXmlOpenToDecode(emu),
        "CryptXmlOpenToEncode" => CryptXmlOpenToEncode(emu),
        "CryptXmlSetHMACSecret" => CryptXmlSetHMACSecret(emu),
        "CryptXmlSign" => CryptXmlSign(emu),
        "CryptXmlVerifySignature" => CryptXmlVerifySignature(emu),
        "PFXExportCertStore" => PFXExportCertStore(emu),
        "PFXExportCertStoreEx" => PFXExportCertStoreEx(emu),
        "PFXImportCertStore" => PFXImportCertStore(emu),
        "PFXIsPFXBlob" => PFXIsPFXBlob(emu),
        "PFXVerifyPassword" => PFXVerifyPassword(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
