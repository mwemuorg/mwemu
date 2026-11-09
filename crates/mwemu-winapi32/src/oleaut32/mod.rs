use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::common::bstr;
use crate::winapi::winapi32::kernel32;

mod bstr_user_free;
pub use bstr_user_free::BSTR_UserFree;
mod bstr_user_free64;
pub use bstr_user_free64::BSTR_UserFree64;
mod bstr_user_marshal;
pub use bstr_user_marshal::BSTR_UserMarshal;
mod bstr_user_marshal64;
pub use bstr_user_marshal64::BSTR_UserMarshal64;
mod bstr_user_size;
pub use bstr_user_size::BSTR_UserSize;
mod bstr_user_size64;
pub use bstr_user_size64::BSTR_UserSize64;
mod bstr_user_unmarshal;
pub use bstr_user_unmarshal::BSTR_UserUnmarshal;
mod bstr_user_unmarshal64;
pub use bstr_user_unmarshal64::BSTR_UserUnmarshal64;
mod bstr_from_vector;
pub use bstr_from_vector::BstrFromVector;
mod clear_cust_data;
pub use clear_cust_data::ClearCustData;
mod create_disp_type_info;
pub use create_disp_type_info::CreateDispTypeInfo;
mod create_error_info;
pub use create_error_info::CreateErrorInfo;
mod create_std_dispatch;
pub use create_std_dispatch::CreateStdDispatch;
mod create_type_lib;
pub use create_type_lib::CreateTypeLib;
mod create_type_lib2;
pub use create_type_lib2::CreateTypeLib2;
mod disp_call_func;
pub use disp_call_func::DispCallFunc;
mod disp_get_i_ds_of_names;
pub use disp_get_i_ds_of_names::DispGetIDsOfNames;
mod disp_get_param;
pub use disp_get_param::DispGetParam;
mod disp_invoke;
pub use disp_invoke::DispInvoke;
mod dll_can_unload_now;
pub use dll_can_unload_now::DllCanUnloadNow;
mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;
mod dll_register_server;
pub use dll_register_server::DllRegisterServer;
mod dll_unregister_server;
pub use dll_unregister_server::DllUnregisterServer;
mod dos_date_time_to_variant_time;
pub use dos_date_time_to_variant_time::DosDateTimeToVariantTime;
mod get_active_object;
pub use get_active_object::GetActiveObject;
mod get_alt_month_names;
pub use get_alt_month_names::GetAltMonthNames;
mod get_error_info;
pub use get_error_info::GetErrorInfo;
mod get_record_info_from_guids;
pub use get_record_info_from_guids::GetRecordInfoFromGuids;
mod get_record_info_from_type_info;
pub use get_record_info_from_type_info::GetRecordInfoFromTypeInfo;
mod get_var_conversion_locale_setting;
pub use get_var_conversion_locale_setting::GetVarConversionLocaleSetting;
mod hwnd_user_free;
pub use hwnd_user_free::HWND_UserFree;
mod hwnd_user_free64;
pub use hwnd_user_free64::HWND_UserFree64;
mod hwnd_user_marshal;
pub use hwnd_user_marshal::HWND_UserMarshal;
mod hwnd_user_marshal64;
pub use hwnd_user_marshal64::HWND_UserMarshal64;
mod hwnd_user_size;
pub use hwnd_user_size::HWND_UserSize;
mod hwnd_user_size64;
pub use hwnd_user_size64::HWND_UserSize64;
mod hwnd_user_unmarshal;
pub use hwnd_user_unmarshal::HWND_UserUnmarshal;
mod hwnd_user_unmarshal64;
pub use hwnd_user_unmarshal64::HWND_UserUnmarshal64;
mod l_hash_val_of_name_sys;
pub use l_hash_val_of_name_sys::LHashValOfNameSys;
mod l_hash_val_of_name_sys_a;
pub use l_hash_val_of_name_sys_a::LHashValOfNameSysA;
mod lpsafearray_marshal;
pub use lpsafearray_marshal::LPSAFEARRAY_Marshal;
mod lpsafearray_size;
pub use lpsafearray_size::LPSAFEARRAY_Size;
mod lpsafearray_unmarshal;
pub use lpsafearray_unmarshal::LPSAFEARRAY_Unmarshal;
mod lpsafearray_user_free;
pub use lpsafearray_user_free::LPSAFEARRAY_UserFree;
mod lpsafearray_user_free64;
pub use lpsafearray_user_free64::LPSAFEARRAY_UserFree64;
mod lpsafearray_user_marshal;
pub use lpsafearray_user_marshal::LPSAFEARRAY_UserMarshal;
mod lpsafearray_user_marshal64;
pub use lpsafearray_user_marshal64::LPSAFEARRAY_UserMarshal64;
mod lpsafearray_user_size;
pub use lpsafearray_user_size::LPSAFEARRAY_UserSize;
mod lpsafearray_user_size64;
pub use lpsafearray_user_size64::LPSAFEARRAY_UserSize64;
mod lpsafearray_user_unmarshal;
pub use lpsafearray_user_unmarshal::LPSAFEARRAY_UserUnmarshal;
mod lpsafearray_user_unmarshal64;
pub use lpsafearray_user_unmarshal64::LPSAFEARRAY_UserUnmarshal64;
mod load_reg_type_lib;
pub use load_reg_type_lib::LoadRegTypeLib;
mod load_type_lib;
pub use load_type_lib::LoadTypeLib;
mod load_type_lib_ex;
pub use load_type_lib_ex::LoadTypeLibEx;
mod oa_cleanup;
pub use oa_cleanup::OACleanup;
mod oa_create_type_lib2;
pub use oa_create_type_lib2::OACreateTypeLib2;
mod oa_build_version;
pub use oa_build_version::OaBuildVersion;
mod oa_enable_per_user_t_lib_registration;
pub use oa_enable_per_user_t_lib_registration::OaEnablePerUserTLibRegistration;
mod ole_create_font_indirect;
pub use ole_create_font_indirect::OleCreateFontIndirect;
mod ole_create_picture_indirect;
pub use ole_create_picture_indirect::OleCreatePictureIndirect;
mod ole_create_property_frame;
pub use ole_create_property_frame::OleCreatePropertyFrame;
mod ole_create_property_frame_indirect;
pub use ole_create_property_frame_indirect::OleCreatePropertyFrameIndirect;
mod ole_icon_to_cursor;
pub use ole_icon_to_cursor::OleIconToCursor;
mod ole_load_picture;
pub use ole_load_picture::OleLoadPicture;
mod ole_load_picture_ex;
pub use ole_load_picture_ex::OleLoadPictureEx;
mod ole_load_picture_file;
pub use ole_load_picture_file::OleLoadPictureFile;
mod ole_load_picture_file_ex;
pub use ole_load_picture_file_ex::OleLoadPictureFileEx;
mod ole_load_picture_path;
pub use ole_load_picture_path::OleLoadPicturePath;
mod ole_save_picture_file;
pub use ole_save_picture_file::OleSavePictureFile;
mod ole_translate_color;
pub use ole_translate_color::OleTranslateColor;
mod query_path_of_reg_type_lib;
pub use query_path_of_reg_type_lib::QueryPathOfRegTypeLib;
mod register_active_object;
pub use register_active_object::RegisterActiveObject;
mod register_type_lib;
pub use register_type_lib::RegisterTypeLib;
mod register_type_lib_for_user;
pub use register_type_lib_for_user::RegisterTypeLibForUser;
mod revoke_active_object;
pub use revoke_active_object::RevokeActiveObject;
mod safe_array_access_data;
pub use safe_array_access_data::SafeArrayAccessData;
mod safe_array_add_ref;
pub use safe_array_add_ref::SafeArrayAddRef;
mod safe_array_alloc_data;
pub use safe_array_alloc_data::SafeArrayAllocData;
mod safe_array_alloc_descriptor;
pub use safe_array_alloc_descriptor::SafeArrayAllocDescriptor;
mod safe_array_alloc_descriptor_ex;
pub use safe_array_alloc_descriptor_ex::SafeArrayAllocDescriptorEx;
mod safe_array_copy;
pub use safe_array_copy::SafeArrayCopy;
mod safe_array_copy_data;
pub use safe_array_copy_data::SafeArrayCopyData;
mod safe_array_create;
pub use safe_array_create::SafeArrayCreate;
mod safe_array_create_ex;
pub use safe_array_create_ex::SafeArrayCreateEx;
mod safe_array_create_vector;
pub use safe_array_create_vector::SafeArrayCreateVector;
mod safe_array_create_vector_ex;
pub use safe_array_create_vector_ex::SafeArrayCreateVectorEx;
mod safe_array_destroy;
pub use safe_array_destroy::SafeArrayDestroy;
mod safe_array_destroy_data;
pub use safe_array_destroy_data::SafeArrayDestroyData;
mod safe_array_destroy_descriptor;
pub use safe_array_destroy_descriptor::SafeArrayDestroyDescriptor;
mod safe_array_get_dim;
pub use safe_array_get_dim::SafeArrayGetDim;
mod safe_array_get_element;
pub use safe_array_get_element::SafeArrayGetElement;
mod safe_array_get_elemsize;
pub use safe_array_get_elemsize::SafeArrayGetElemsize;
mod safe_array_get_iid;
pub use safe_array_get_iid::SafeArrayGetIID;
mod safe_array_get_l_bound;
pub use safe_array_get_l_bound::SafeArrayGetLBound;
mod safe_array_get_record_info;
pub use safe_array_get_record_info::SafeArrayGetRecordInfo;
mod safe_array_get_u_bound;
pub use safe_array_get_u_bound::SafeArrayGetUBound;
mod safe_array_get_vartype;
pub use safe_array_get_vartype::SafeArrayGetVartype;
mod safe_array_lock;
pub use safe_array_lock::SafeArrayLock;
mod safe_array_ptr_of_index;
pub use safe_array_ptr_of_index::SafeArrayPtrOfIndex;
mod safe_array_put_element;
pub use safe_array_put_element::SafeArrayPutElement;
mod safe_array_redim;
pub use safe_array_redim::SafeArrayRedim;
mod safe_array_release_data;
pub use safe_array_release_data::SafeArrayReleaseData;
mod safe_array_release_descriptor;
pub use safe_array_release_descriptor::SafeArrayReleaseDescriptor;
mod safe_array_set_iid;
pub use safe_array_set_iid::SafeArraySetIID;
mod safe_array_set_record_info;
pub use safe_array_set_record_info::SafeArraySetRecordInfo;
mod safe_array_unaccess_data;
pub use safe_array_unaccess_data::SafeArrayUnaccessData;
mod safe_array_unlock;
pub use safe_array_unlock::SafeArrayUnlock;
mod set_error_info;
pub use set_error_info::SetErrorInfo;
mod set_oa_no_cache;
pub use set_oa_no_cache::SetOaNoCache;
mod set_var_conversion_locale_setting;
pub use set_var_conversion_locale_setting::SetVarConversionLocaleSetting;
mod sys_add_ref_string;
pub use sys_add_ref_string::SysAddRefString;
mod sys_alloc_string_byte_len;
pub use sys_alloc_string_byte_len::SysAllocStringByteLen;
mod sys_re_alloc_string;
pub use sys_re_alloc_string::SysReAllocString;
mod sys_release_string;
pub use sys_release_string::SysReleaseString;
mod system_time_to_variant_time;
pub use system_time_to_variant_time::SystemTimeToVariantTime;
mod un_register_type_lib;
pub use un_register_type_lib::UnRegisterTypeLib;
mod un_register_type_lib_for_user;
pub use un_register_type_lib_for_user::UnRegisterTypeLibForUser;
mod variant_user_free;
pub use variant_user_free::VARIANT_UserFree;
mod variant_user_free64;
pub use variant_user_free64::VARIANT_UserFree64;
mod variant_user_marshal;
pub use variant_user_marshal::VARIANT_UserMarshal;
mod variant_user_marshal64;
pub use variant_user_marshal64::VARIANT_UserMarshal64;
mod variant_user_size;
pub use variant_user_size::VARIANT_UserSize;
mod variant_user_size64;
pub use variant_user_size64::VARIANT_UserSize64;
mod variant_user_unmarshal;
pub use variant_user_unmarshal::VARIANT_UserUnmarshal;
mod variant_user_unmarshal64;
pub use variant_user_unmarshal64::VARIANT_UserUnmarshal64;
mod var_abs;
pub use var_abs::VarAbs;
mod var_add;
pub use var_add::VarAdd;
mod var_and;
pub use var_and::VarAnd;
mod var_bool_from_cy;
pub use var_bool_from_cy::VarBoolFromCy;
mod var_bool_from_date;
pub use var_bool_from_date::VarBoolFromDate;
mod var_bool_from_dec;
pub use var_bool_from_dec::VarBoolFromDec;
mod var_bool_from_disp;
pub use var_bool_from_disp::VarBoolFromDisp;
mod var_bool_from_i1;
pub use var_bool_from_i1::VarBoolFromI1;
mod var_bool_from_i2;
pub use var_bool_from_i2::VarBoolFromI2;
mod var_bool_from_i4;
pub use var_bool_from_i4::VarBoolFromI4;
mod var_bool_from_i8;
pub use var_bool_from_i8::VarBoolFromI8;
mod var_bool_from_r4;
pub use var_bool_from_r4::VarBoolFromR4;
mod var_bool_from_r8;
pub use var_bool_from_r8::VarBoolFromR8;
mod var_bool_from_str;
pub use var_bool_from_str::VarBoolFromStr;
mod var_bool_from_ui1;
pub use var_bool_from_ui1::VarBoolFromUI1;
mod var_bool_from_ui2;
pub use var_bool_from_ui2::VarBoolFromUI2;
mod var_bool_from_ui4;
pub use var_bool_from_ui4::VarBoolFromUI4;
mod var_bool_from_ui8;
pub use var_bool_from_ui8::VarBoolFromUI8;
mod var_bstr_cat;
pub use var_bstr_cat::VarBstrCat;
mod var_bstr_cmp;
pub use var_bstr_cmp::VarBstrCmp;
mod var_bstr_from_bool;
pub use var_bstr_from_bool::VarBstrFromBool;
mod var_bstr_from_cy;
pub use var_bstr_from_cy::VarBstrFromCy;
mod var_bstr_from_date;
pub use var_bstr_from_date::VarBstrFromDate;
mod var_bstr_from_dec;
pub use var_bstr_from_dec::VarBstrFromDec;
mod var_bstr_from_disp;
pub use var_bstr_from_disp::VarBstrFromDisp;
mod var_bstr_from_i1;
pub use var_bstr_from_i1::VarBstrFromI1;
mod var_bstr_from_i2;
pub use var_bstr_from_i2::VarBstrFromI2;
mod var_bstr_from_i4;
pub use var_bstr_from_i4::VarBstrFromI4;
mod var_bstr_from_i8;
pub use var_bstr_from_i8::VarBstrFromI8;
mod var_bstr_from_r4;
pub use var_bstr_from_r4::VarBstrFromR4;
mod var_bstr_from_r8;
pub use var_bstr_from_r8::VarBstrFromR8;
mod var_bstr_from_ui1;
pub use var_bstr_from_ui1::VarBstrFromUI1;
mod var_bstr_from_ui2;
pub use var_bstr_from_ui2::VarBstrFromUI2;
mod var_bstr_from_ui4;
pub use var_bstr_from_ui4::VarBstrFromUI4;
mod var_bstr_from_ui8;
pub use var_bstr_from_ui8::VarBstrFromUI8;
mod var_cat;
pub use var_cat::VarCat;
mod var_cmp;
pub use var_cmp::VarCmp;
mod var_cy_abs;
pub use var_cy_abs::VarCyAbs;
mod var_cy_add;
pub use var_cy_add::VarCyAdd;
mod var_cy_cmp;
pub use var_cy_cmp::VarCyCmp;
mod var_cy_cmp_r8;
pub use var_cy_cmp_r8::VarCyCmpR8;
mod var_cy_fix;
pub use var_cy_fix::VarCyFix;
mod var_cy_from_bool;
pub use var_cy_from_bool::VarCyFromBool;
mod var_cy_from_date;
pub use var_cy_from_date::VarCyFromDate;
mod var_cy_from_dec;
pub use var_cy_from_dec::VarCyFromDec;
mod var_cy_from_disp;
pub use var_cy_from_disp::VarCyFromDisp;
mod var_cy_from_i1;
pub use var_cy_from_i1::VarCyFromI1;
mod var_cy_from_i2;
pub use var_cy_from_i2::VarCyFromI2;
mod var_cy_from_i4;
pub use var_cy_from_i4::VarCyFromI4;
mod var_cy_from_i8;
pub use var_cy_from_i8::VarCyFromI8;
mod var_cy_from_r4;
pub use var_cy_from_r4::VarCyFromR4;
mod var_cy_from_r8;
pub use var_cy_from_r8::VarCyFromR8;
mod var_cy_from_str;
pub use var_cy_from_str::VarCyFromStr;
mod var_cy_from_ui1;
pub use var_cy_from_ui1::VarCyFromUI1;
mod var_cy_from_ui2;
pub use var_cy_from_ui2::VarCyFromUI2;
mod var_cy_from_ui4;
pub use var_cy_from_ui4::VarCyFromUI4;
mod var_cy_from_ui8;
pub use var_cy_from_ui8::VarCyFromUI8;
mod var_cy_int;
pub use var_cy_int::VarCyInt;
mod var_cy_mul;
pub use var_cy_mul::VarCyMul;
mod var_cy_mul_i4;
pub use var_cy_mul_i4::VarCyMulI4;
mod var_cy_mul_i8;
pub use var_cy_mul_i8::VarCyMulI8;
mod var_cy_neg;
pub use var_cy_neg::VarCyNeg;
mod var_cy_round;
pub use var_cy_round::VarCyRound;
mod var_cy_sub;
pub use var_cy_sub::VarCySub;
mod var_date_from_bool;
pub use var_date_from_bool::VarDateFromBool;
mod var_date_from_cy;
pub use var_date_from_cy::VarDateFromCy;
mod var_date_from_dec;
pub use var_date_from_dec::VarDateFromDec;
mod var_date_from_disp;
pub use var_date_from_disp::VarDateFromDisp;
mod var_date_from_i1;
pub use var_date_from_i1::VarDateFromI1;
mod var_date_from_i2;
pub use var_date_from_i2::VarDateFromI2;
mod var_date_from_i4;
pub use var_date_from_i4::VarDateFromI4;
mod var_date_from_i8;
pub use var_date_from_i8::VarDateFromI8;
mod var_date_from_r4;
pub use var_date_from_r4::VarDateFromR4;
mod var_date_from_r8;
pub use var_date_from_r8::VarDateFromR8;
mod var_date_from_str;
pub use var_date_from_str::VarDateFromStr;
mod var_date_from_ui1;
pub use var_date_from_ui1::VarDateFromUI1;
mod var_date_from_ui2;
pub use var_date_from_ui2::VarDateFromUI2;
mod var_date_from_ui4;
pub use var_date_from_ui4::VarDateFromUI4;
mod var_date_from_ui8;
pub use var_date_from_ui8::VarDateFromUI8;
mod var_date_from_udate;
pub use var_date_from_udate::VarDateFromUdate;
mod var_date_from_udate_ex;
pub use var_date_from_udate_ex::VarDateFromUdateEx;
mod var_dec_abs;
pub use var_dec_abs::VarDecAbs;
mod var_dec_add;
pub use var_dec_add::VarDecAdd;
mod var_dec_cmp;
pub use var_dec_cmp::VarDecCmp;
mod var_dec_cmp_r8;
pub use var_dec_cmp_r8::VarDecCmpR8;
mod var_dec_div;
pub use var_dec_div::VarDecDiv;
mod var_dec_fix;
pub use var_dec_fix::VarDecFix;
mod var_dec_from_bool;
pub use var_dec_from_bool::VarDecFromBool;
mod var_dec_from_cy;
pub use var_dec_from_cy::VarDecFromCy;
mod var_dec_from_date;
pub use var_dec_from_date::VarDecFromDate;
mod var_dec_from_disp;
pub use var_dec_from_disp::VarDecFromDisp;
mod var_dec_from_i1;
pub use var_dec_from_i1::VarDecFromI1;
mod var_dec_from_i2;
pub use var_dec_from_i2::VarDecFromI2;
mod var_dec_from_i4;
pub use var_dec_from_i4::VarDecFromI4;
mod var_dec_from_i8;
pub use var_dec_from_i8::VarDecFromI8;
mod var_dec_from_r4;
pub use var_dec_from_r4::VarDecFromR4;
mod var_dec_from_r8;
pub use var_dec_from_r8::VarDecFromR8;
mod var_dec_from_str;
pub use var_dec_from_str::VarDecFromStr;
mod var_dec_from_ui1;
pub use var_dec_from_ui1::VarDecFromUI1;
mod var_dec_from_ui2;
pub use var_dec_from_ui2::VarDecFromUI2;
mod var_dec_from_ui4;
pub use var_dec_from_ui4::VarDecFromUI4;
mod var_dec_from_ui8;
pub use var_dec_from_ui8::VarDecFromUI8;
mod var_dec_int;
pub use var_dec_int::VarDecInt;
mod var_dec_mul;
pub use var_dec_mul::VarDecMul;
mod var_dec_neg;
pub use var_dec_neg::VarDecNeg;
mod var_dec_round;
pub use var_dec_round::VarDecRound;
mod var_dec_sub;
pub use var_dec_sub::VarDecSub;
mod var_div;
pub use var_div::VarDiv;
mod var_eqv;
pub use var_eqv::VarEqv;
mod var_fix;
pub use var_fix::VarFix;
mod var_format;
pub use var_format::VarFormat;
mod var_format_currency;
pub use var_format_currency::VarFormatCurrency;
mod var_format_date_time;
pub use var_format_date_time::VarFormatDateTime;
mod var_format_from_tokens;
pub use var_format_from_tokens::VarFormatFromTokens;
mod var_format_number;
pub use var_format_number::VarFormatNumber;
mod var_format_percent;
pub use var_format_percent::VarFormatPercent;
mod var_i1_from_bool;
pub use var_i1_from_bool::VarI1FromBool;
mod var_i1_from_cy;
pub use var_i1_from_cy::VarI1FromCy;
mod var_i1_from_date;
pub use var_i1_from_date::VarI1FromDate;
mod var_i1_from_dec;
pub use var_i1_from_dec::VarI1FromDec;
mod var_i1_from_disp;
pub use var_i1_from_disp::VarI1FromDisp;
mod var_i1_from_i2;
pub use var_i1_from_i2::VarI1FromI2;
mod var_i1_from_i4;
pub use var_i1_from_i4::VarI1FromI4;
mod var_i1_from_i8;
pub use var_i1_from_i8::VarI1FromI8;
mod var_i1_from_r4;
pub use var_i1_from_r4::VarI1FromR4;
mod var_i1_from_r8;
pub use var_i1_from_r8::VarI1FromR8;
mod var_i1_from_str;
pub use var_i1_from_str::VarI1FromStr;
mod var_i1_from_ui1;
pub use var_i1_from_ui1::VarI1FromUI1;
mod var_i1_from_ui2;
pub use var_i1_from_ui2::VarI1FromUI2;
mod var_i1_from_ui4;
pub use var_i1_from_ui4::VarI1FromUI4;
mod var_i1_from_ui8;
pub use var_i1_from_ui8::VarI1FromUI8;
mod var_i2_from_bool;
pub use var_i2_from_bool::VarI2FromBool;
mod var_i2_from_cy;
pub use var_i2_from_cy::VarI2FromCy;
mod var_i2_from_date;
pub use var_i2_from_date::VarI2FromDate;
mod var_i2_from_dec;
pub use var_i2_from_dec::VarI2FromDec;
mod var_i2_from_disp;
pub use var_i2_from_disp::VarI2FromDisp;
mod var_i2_from_i1;
pub use var_i2_from_i1::VarI2FromI1;
mod var_i2_from_i4;
pub use var_i2_from_i4::VarI2FromI4;
mod var_i2_from_i8;
pub use var_i2_from_i8::VarI2FromI8;
mod var_i2_from_r4;
pub use var_i2_from_r4::VarI2FromR4;
mod var_i2_from_r8;
pub use var_i2_from_r8::VarI2FromR8;
mod var_i2_from_str;
pub use var_i2_from_str::VarI2FromStr;
mod var_i2_from_ui1;
pub use var_i2_from_ui1::VarI2FromUI1;
mod var_i2_from_ui2;
pub use var_i2_from_ui2::VarI2FromUI2;
mod var_i2_from_ui4;
pub use var_i2_from_ui4::VarI2FromUI4;
mod var_i2_from_ui8;
pub use var_i2_from_ui8::VarI2FromUI8;
mod var_i4_from_bool;
pub use var_i4_from_bool::VarI4FromBool;
mod var_i4_from_cy;
pub use var_i4_from_cy::VarI4FromCy;
mod var_i4_from_date;
pub use var_i4_from_date::VarI4FromDate;
mod var_i4_from_dec;
pub use var_i4_from_dec::VarI4FromDec;
mod var_i4_from_disp;
pub use var_i4_from_disp::VarI4FromDisp;
mod var_i4_from_i1;
pub use var_i4_from_i1::VarI4FromI1;
mod var_i4_from_i2;
pub use var_i4_from_i2::VarI4FromI2;
mod var_i4_from_i8;
pub use var_i4_from_i8::VarI4FromI8;
mod var_i4_from_r4;
pub use var_i4_from_r4::VarI4FromR4;
mod var_i4_from_r8;
pub use var_i4_from_r8::VarI4FromR8;
mod var_i4_from_str;
pub use var_i4_from_str::VarI4FromStr;
mod var_i4_from_ui1;
pub use var_i4_from_ui1::VarI4FromUI1;
mod var_i4_from_ui2;
pub use var_i4_from_ui2::VarI4FromUI2;
mod var_i4_from_ui4;
pub use var_i4_from_ui4::VarI4FromUI4;
mod var_i4_from_ui8;
pub use var_i4_from_ui8::VarI4FromUI8;
mod var_i8_from_bool;
pub use var_i8_from_bool::VarI8FromBool;
mod var_i8_from_cy;
pub use var_i8_from_cy::VarI8FromCy;
mod var_i8_from_date;
pub use var_i8_from_date::VarI8FromDate;
mod var_i8_from_dec;
pub use var_i8_from_dec::VarI8FromDec;
mod var_i8_from_disp;
pub use var_i8_from_disp::VarI8FromDisp;
mod var_i8_from_i1;
pub use var_i8_from_i1::VarI8FromI1;
mod var_i8_from_i2;
pub use var_i8_from_i2::VarI8FromI2;
mod var_i8_from_r4;
pub use var_i8_from_r4::VarI8FromR4;
mod var_i8_from_r8;
pub use var_i8_from_r8::VarI8FromR8;
mod var_i8_from_str;
pub use var_i8_from_str::VarI8FromStr;
mod var_i8_from_ui1;
pub use var_i8_from_ui1::VarI8FromUI1;
mod var_i8_from_ui2;
pub use var_i8_from_ui2::VarI8FromUI2;
mod var_i8_from_ui4;
pub use var_i8_from_ui4::VarI8FromUI4;
mod var_i8_from_ui8;
pub use var_i8_from_ui8::VarI8FromUI8;
mod var_idiv;
pub use var_idiv::VarIdiv;
mod var_imp;
pub use var_imp::VarImp;
mod var_int;
pub use var_int::VarInt;
mod var_mod;
pub use var_mod::VarMod;
mod var_month_name;
pub use var_month_name::VarMonthName;
mod var_mul;
pub use var_mul::VarMul;
mod var_neg;
pub use var_neg::VarNeg;
mod var_not;
pub use var_not::VarNot;
mod var_num_from_parse_num;
pub use var_num_from_parse_num::VarNumFromParseNum;
mod var_or;
pub use var_or::VarOr;
mod var_parse_num_from_str;
pub use var_parse_num_from_str::VarParseNumFromStr;
mod var_pow;
pub use var_pow::VarPow;
mod var_r4_cmp_r8;
pub use var_r4_cmp_r8::VarR4CmpR8;
mod var_r4_from_bool;
pub use var_r4_from_bool::VarR4FromBool;
mod var_r4_from_cy;
pub use var_r4_from_cy::VarR4FromCy;
mod var_r4_from_date;
pub use var_r4_from_date::VarR4FromDate;
mod var_r4_from_dec;
pub use var_r4_from_dec::VarR4FromDec;
mod var_r4_from_disp;
pub use var_r4_from_disp::VarR4FromDisp;
mod var_r4_from_i1;
pub use var_r4_from_i1::VarR4FromI1;
mod var_r4_from_i2;
pub use var_r4_from_i2::VarR4FromI2;
mod var_r4_from_i4;
pub use var_r4_from_i4::VarR4FromI4;
mod var_r4_from_i8;
pub use var_r4_from_i8::VarR4FromI8;
mod var_r4_from_r8;
pub use var_r4_from_r8::VarR4FromR8;
mod var_r4_from_str;
pub use var_r4_from_str::VarR4FromStr;
mod var_r4_from_ui1;
pub use var_r4_from_ui1::VarR4FromUI1;
mod var_r4_from_ui2;
pub use var_r4_from_ui2::VarR4FromUI2;
mod var_r4_from_ui4;
pub use var_r4_from_ui4::VarR4FromUI4;
mod var_r4_from_ui8;
pub use var_r4_from_ui8::VarR4FromUI8;
mod var_r8_from_bool;
pub use var_r8_from_bool::VarR8FromBool;
mod var_r8_from_cy;
pub use var_r8_from_cy::VarR8FromCy;
mod var_r8_from_date;
pub use var_r8_from_date::VarR8FromDate;
mod var_r8_from_dec;
pub use var_r8_from_dec::VarR8FromDec;
mod var_r8_from_disp;
pub use var_r8_from_disp::VarR8FromDisp;
mod var_r8_from_i1;
pub use var_r8_from_i1::VarR8FromI1;
mod var_r8_from_i2;
pub use var_r8_from_i2::VarR8FromI2;
mod var_r8_from_i4;
pub use var_r8_from_i4::VarR8FromI4;
mod var_r8_from_i8;
pub use var_r8_from_i8::VarR8FromI8;
mod var_r8_from_r4;
pub use var_r8_from_r4::VarR8FromR4;
mod var_r8_from_str;
pub use var_r8_from_str::VarR8FromStr;
mod var_r8_from_ui1;
pub use var_r8_from_ui1::VarR8FromUI1;
mod var_r8_from_ui2;
pub use var_r8_from_ui2::VarR8FromUI2;
mod var_r8_from_ui4;
pub use var_r8_from_ui4::VarR8FromUI4;
mod var_r8_from_ui8;
pub use var_r8_from_ui8::VarR8FromUI8;
mod var_r8_pow;
pub use var_r8_pow::VarR8Pow;
mod var_r8_round;
pub use var_r8_round::VarR8Round;
mod var_round;
pub use var_round::VarRound;
mod var_sub;
pub use var_sub::VarSub;
mod var_tokenize_format_string;
pub use var_tokenize_format_string::VarTokenizeFormatString;
mod var_ui1_from_bool;
pub use var_ui1_from_bool::VarUI1FromBool;
mod var_ui1_from_cy;
pub use var_ui1_from_cy::VarUI1FromCy;
mod var_ui1_from_date;
pub use var_ui1_from_date::VarUI1FromDate;
mod var_ui1_from_dec;
pub use var_ui1_from_dec::VarUI1FromDec;
mod var_ui1_from_disp;
pub use var_ui1_from_disp::VarUI1FromDisp;
mod var_ui1_from_i1;
pub use var_ui1_from_i1::VarUI1FromI1;
mod var_ui1_from_i2;
pub use var_ui1_from_i2::VarUI1FromI2;
mod var_ui1_from_i4;
pub use var_ui1_from_i4::VarUI1FromI4;
mod var_ui1_from_i8;
pub use var_ui1_from_i8::VarUI1FromI8;
mod var_ui1_from_r4;
pub use var_ui1_from_r4::VarUI1FromR4;
mod var_ui1_from_r8;
pub use var_ui1_from_r8::VarUI1FromR8;
mod var_ui1_from_str;
pub use var_ui1_from_str::VarUI1FromStr;
mod var_ui1_from_ui2;
pub use var_ui1_from_ui2::VarUI1FromUI2;
mod var_ui1_from_ui4;
pub use var_ui1_from_ui4::VarUI1FromUI4;
mod var_ui1_from_ui8;
pub use var_ui1_from_ui8::VarUI1FromUI8;
mod var_ui2_from_bool;
pub use var_ui2_from_bool::VarUI2FromBool;
mod var_ui2_from_cy;
pub use var_ui2_from_cy::VarUI2FromCy;
mod var_ui2_from_date;
pub use var_ui2_from_date::VarUI2FromDate;
mod var_ui2_from_dec;
pub use var_ui2_from_dec::VarUI2FromDec;
mod var_ui2_from_disp;
pub use var_ui2_from_disp::VarUI2FromDisp;
mod var_ui2_from_i1;
pub use var_ui2_from_i1::VarUI2FromI1;
mod var_ui2_from_i2;
pub use var_ui2_from_i2::VarUI2FromI2;
mod var_ui2_from_i4;
pub use var_ui2_from_i4::VarUI2FromI4;
mod var_ui2_from_i8;
pub use var_ui2_from_i8::VarUI2FromI8;
mod var_ui2_from_r4;
pub use var_ui2_from_r4::VarUI2FromR4;
mod var_ui2_from_r8;
pub use var_ui2_from_r8::VarUI2FromR8;
mod var_ui2_from_str;
pub use var_ui2_from_str::VarUI2FromStr;
mod var_ui2_from_ui1;
pub use var_ui2_from_ui1::VarUI2FromUI1;
mod var_ui2_from_ui4;
pub use var_ui2_from_ui4::VarUI2FromUI4;
mod var_ui2_from_ui8;
pub use var_ui2_from_ui8::VarUI2FromUI8;
mod var_ui4_from_bool;
pub use var_ui4_from_bool::VarUI4FromBool;
mod var_ui4_from_cy;
pub use var_ui4_from_cy::VarUI4FromCy;
mod var_ui4_from_date;
pub use var_ui4_from_date::VarUI4FromDate;
mod var_ui4_from_dec;
pub use var_ui4_from_dec::VarUI4FromDec;
mod var_ui4_from_disp;
pub use var_ui4_from_disp::VarUI4FromDisp;
mod var_ui4_from_i1;
pub use var_ui4_from_i1::VarUI4FromI1;
mod var_ui4_from_i2;
pub use var_ui4_from_i2::VarUI4FromI2;
mod var_ui4_from_i4;
pub use var_ui4_from_i4::VarUI4FromI4;
mod var_ui4_from_i8;
pub use var_ui4_from_i8::VarUI4FromI8;
mod var_ui4_from_r4;
pub use var_ui4_from_r4::VarUI4FromR4;
mod var_ui4_from_r8;
pub use var_ui4_from_r8::VarUI4FromR8;
mod var_ui4_from_str;
pub use var_ui4_from_str::VarUI4FromStr;
mod var_ui4_from_ui1;
pub use var_ui4_from_ui1::VarUI4FromUI1;
mod var_ui4_from_ui2;
pub use var_ui4_from_ui2::VarUI4FromUI2;
mod var_ui4_from_ui8;
pub use var_ui4_from_ui8::VarUI4FromUI8;
mod var_ui8_from_bool;
pub use var_ui8_from_bool::VarUI8FromBool;
mod var_ui8_from_cy;
pub use var_ui8_from_cy::VarUI8FromCy;
mod var_ui8_from_date;
pub use var_ui8_from_date::VarUI8FromDate;
mod var_ui8_from_dec;
pub use var_ui8_from_dec::VarUI8FromDec;
mod var_ui8_from_disp;
pub use var_ui8_from_disp::VarUI8FromDisp;
mod var_ui8_from_i1;
pub use var_ui8_from_i1::VarUI8FromI1;
mod var_ui8_from_i2;
pub use var_ui8_from_i2::VarUI8FromI2;
mod var_ui8_from_i8;
pub use var_ui8_from_i8::VarUI8FromI8;
mod var_ui8_from_r4;
pub use var_ui8_from_r4::VarUI8FromR4;
mod var_ui8_from_r8;
pub use var_ui8_from_r8::VarUI8FromR8;
mod var_ui8_from_str;
pub use var_ui8_from_str::VarUI8FromStr;
mod var_ui8_from_ui1;
pub use var_ui8_from_ui1::VarUI8FromUI1;
mod var_ui8_from_ui2;
pub use var_ui8_from_ui2::VarUI8FromUI2;
mod var_ui8_from_ui4;
pub use var_ui8_from_ui4::VarUI8FromUI4;
mod var_udate_from_date;
pub use var_udate_from_date::VarUdateFromDate;
mod var_weekday_name;
pub use var_weekday_name::VarWeekdayName;
mod var_xor;
pub use var_xor::VarXor;
mod variant_change_type;
pub use variant_change_type::VariantChangeType;
mod variant_change_type_ex;
pub use variant_change_type_ex::VariantChangeTypeEx;
mod variant_copy;
pub use variant_copy::VariantCopy;
mod variant_copy_ind;
pub use variant_copy_ind::VariantCopyInd;
mod variant_time_to_dos_date_time;
pub use variant_time_to_dos_date_time::VariantTimeToDosDateTime;
mod variant_time_to_system_time;
pub use variant_time_to_system_time::VariantTimeToSystemTime;
mod vector_from_bstr;
pub use vector_from_bstr::VectorFromBstr;
mod sys_alloc_string;
use sys_alloc_string::SysAllocString;
mod sys_alloc_string_len;
use sys_alloc_string_len::SysAllocStringLen;
mod sys_free_string;
use sys_free_string::SysFreeString;
mod sys_re_alloc_string_len;
use sys_re_alloc_string_len::SysReAllocStringLen;
mod sys_string_byte_len;
use sys_string_byte_len::SysStringByteLen;
mod sys_string_len;
use sys_string_len::SysStringLen;
mod variant_clear;
use variant_clear::VariantClear;
mod variant_init;
use variant_init::VariantInit;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if bstr::dispatch(api, emu) {
        return String::new();
    }
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysAllocString" => SysAllocString(emu),
        "SysFreeString" => SysFreeString(emu),
        "SysStringLen" => SysStringLen(emu),
        "SysStringByteLen" => SysStringByteLen(emu),
        "SysReAllocStringLen" => SysReAllocStringLen(emu),
        "VariantInit" => VariantInit(emu),
        "VariantClear" => VariantClear(emu),
        "BSTR_UserFree" => BSTR_UserFree(emu),
        "BSTR_UserFree64" => BSTR_UserFree64(emu),
        "BSTR_UserMarshal" => BSTR_UserMarshal(emu),
        "BSTR_UserMarshal64" => BSTR_UserMarshal64(emu),
        "BSTR_UserSize" => BSTR_UserSize(emu),
        "BSTR_UserSize64" => BSTR_UserSize64(emu),
        "BSTR_UserUnmarshal" => BSTR_UserUnmarshal(emu),
        "BSTR_UserUnmarshal64" => BSTR_UserUnmarshal64(emu),
        "BstrFromVector" => BstrFromVector(emu),
        "ClearCustData" => ClearCustData(emu),
        "CreateDispTypeInfo" => CreateDispTypeInfo(emu),
        "CreateErrorInfo" => CreateErrorInfo(emu),
        "CreateStdDispatch" => CreateStdDispatch(emu),
        "CreateTypeLib" => CreateTypeLib(emu),
        "CreateTypeLib2" => CreateTypeLib2(emu),
        "DispCallFunc" => DispCallFunc(emu),
        "DispGetIDsOfNames" => DispGetIDsOfNames(emu),
        "DispGetParam" => DispGetParam(emu),
        "DispInvoke" => DispInvoke(emu),
        "DllCanUnloadNow" => DllCanUnloadNow(emu),
        "DllGetClassObject" => DllGetClassObject(emu),
        "DllRegisterServer" => DllRegisterServer(emu),
        "DllUnregisterServer" => DllUnregisterServer(emu),
        "DosDateTimeToVariantTime" => DosDateTimeToVariantTime(emu),
        "GetActiveObject" => GetActiveObject(emu),
        "GetAltMonthNames" => GetAltMonthNames(emu),
        "GetErrorInfo" => GetErrorInfo(emu),
        "GetRecordInfoFromGuids" => GetRecordInfoFromGuids(emu),
        "GetRecordInfoFromTypeInfo" => GetRecordInfoFromTypeInfo(emu),
        "GetVarConversionLocaleSetting" => GetVarConversionLocaleSetting(emu),
        "HWND_UserFree" => HWND_UserFree(emu),
        "HWND_UserFree64" => HWND_UserFree64(emu),
        "HWND_UserMarshal" => HWND_UserMarshal(emu),
        "HWND_UserMarshal64" => HWND_UserMarshal64(emu),
        "HWND_UserSize" => HWND_UserSize(emu),
        "HWND_UserSize64" => HWND_UserSize64(emu),
        "HWND_UserUnmarshal" => HWND_UserUnmarshal(emu),
        "HWND_UserUnmarshal64" => HWND_UserUnmarshal64(emu),
        "LHashValOfNameSys" => LHashValOfNameSys(emu),
        "LHashValOfNameSysA" => LHashValOfNameSysA(emu),
        "LPSAFEARRAY_Marshal" => LPSAFEARRAY_Marshal(emu),
        "LPSAFEARRAY_Size" => LPSAFEARRAY_Size(emu),
        "LPSAFEARRAY_Unmarshal" => LPSAFEARRAY_Unmarshal(emu),
        "LPSAFEARRAY_UserFree" => LPSAFEARRAY_UserFree(emu),
        "LPSAFEARRAY_UserFree64" => LPSAFEARRAY_UserFree64(emu),
        "LPSAFEARRAY_UserMarshal" => LPSAFEARRAY_UserMarshal(emu),
        "LPSAFEARRAY_UserMarshal64" => LPSAFEARRAY_UserMarshal64(emu),
        "LPSAFEARRAY_UserSize" => LPSAFEARRAY_UserSize(emu),
        "LPSAFEARRAY_UserSize64" => LPSAFEARRAY_UserSize64(emu),
        "LPSAFEARRAY_UserUnmarshal" => LPSAFEARRAY_UserUnmarshal(emu),
        "LPSAFEARRAY_UserUnmarshal64" => LPSAFEARRAY_UserUnmarshal64(emu),
        "LoadRegTypeLib" => LoadRegTypeLib(emu),
        "LoadTypeLib" => LoadTypeLib(emu),
        "LoadTypeLibEx" => LoadTypeLibEx(emu),
        "OACleanup" => OACleanup(emu),
        "OACreateTypeLib2" => OACreateTypeLib2(emu),
        "OaBuildVersion" => OaBuildVersion(emu),
        "OaEnablePerUserTLibRegistration" => OaEnablePerUserTLibRegistration(emu),
        "OleCreateFontIndirect" => OleCreateFontIndirect(emu),
        "OleCreatePictureIndirect" => OleCreatePictureIndirect(emu),
        "OleCreatePropertyFrame" => OleCreatePropertyFrame(emu),
        "OleCreatePropertyFrameIndirect" => OleCreatePropertyFrameIndirect(emu),
        "OleIconToCursor" => OleIconToCursor(emu),
        "OleLoadPicture" => OleLoadPicture(emu),
        "OleLoadPictureEx" => OleLoadPictureEx(emu),
        "OleLoadPictureFile" => OleLoadPictureFile(emu),
        "OleLoadPictureFileEx" => OleLoadPictureFileEx(emu),
        "OleLoadPicturePath" => OleLoadPicturePath(emu),
        "OleSavePictureFile" => OleSavePictureFile(emu),
        "OleTranslateColor" => OleTranslateColor(emu),
        "QueryPathOfRegTypeLib" => QueryPathOfRegTypeLib(emu),
        "RegisterActiveObject" => RegisterActiveObject(emu),
        "RegisterTypeLib" => RegisterTypeLib(emu),
        "RegisterTypeLibForUser" => RegisterTypeLibForUser(emu),
        "RevokeActiveObject" => RevokeActiveObject(emu),
        "SafeArrayAccessData" => SafeArrayAccessData(emu),
        "SafeArrayAddRef" => SafeArrayAddRef(emu),
        "SafeArrayAllocData" => SafeArrayAllocData(emu),
        "SafeArrayAllocDescriptor" => SafeArrayAllocDescriptor(emu),
        "SafeArrayAllocDescriptorEx" => SafeArrayAllocDescriptorEx(emu),
        "SafeArrayCopy" => SafeArrayCopy(emu),
        "SafeArrayCopyData" => SafeArrayCopyData(emu),
        "SafeArrayCreate" => SafeArrayCreate(emu),
        "SafeArrayCreateEx" => SafeArrayCreateEx(emu),
        "SafeArrayCreateVector" => SafeArrayCreateVector(emu),
        "SafeArrayCreateVectorEx" => SafeArrayCreateVectorEx(emu),
        "SafeArrayDestroy" => SafeArrayDestroy(emu),
        "SafeArrayDestroyData" => SafeArrayDestroyData(emu),
        "SafeArrayDestroyDescriptor" => SafeArrayDestroyDescriptor(emu),
        "SafeArrayGetDim" => SafeArrayGetDim(emu),
        "SafeArrayGetElement" => SafeArrayGetElement(emu),
        "SafeArrayGetElemsize" => SafeArrayGetElemsize(emu),
        "SafeArrayGetIID" => SafeArrayGetIID(emu),
        "SafeArrayGetLBound" => SafeArrayGetLBound(emu),
        "SafeArrayGetRecordInfo" => SafeArrayGetRecordInfo(emu),
        "SafeArrayGetUBound" => SafeArrayGetUBound(emu),
        "SafeArrayGetVartype" => SafeArrayGetVartype(emu),
        "SafeArrayLock" => SafeArrayLock(emu),
        "SafeArrayPtrOfIndex" => SafeArrayPtrOfIndex(emu),
        "SafeArrayPutElement" => SafeArrayPutElement(emu),
        "SafeArrayRedim" => SafeArrayRedim(emu),
        "SafeArrayReleaseData" => SafeArrayReleaseData(emu),
        "SafeArrayReleaseDescriptor" => SafeArrayReleaseDescriptor(emu),
        "SafeArraySetIID" => SafeArraySetIID(emu),
        "SafeArraySetRecordInfo" => SafeArraySetRecordInfo(emu),
        "SafeArrayUnaccessData" => SafeArrayUnaccessData(emu),
        "SafeArrayUnlock" => SafeArrayUnlock(emu),
        "SetErrorInfo" => SetErrorInfo(emu),
        "SetOaNoCache" => SetOaNoCache(emu),
        "SetVarConversionLocaleSetting" => SetVarConversionLocaleSetting(emu),
        "SysAddRefString" => SysAddRefString(emu),
        "SysAllocStringByteLen" => SysAllocStringByteLen(emu),
        "SysReAllocString" => SysReAllocString(emu),
        "SysReleaseString" => SysReleaseString(emu),
        "SystemTimeToVariantTime" => SystemTimeToVariantTime(emu),
        "UnRegisterTypeLib" => UnRegisterTypeLib(emu),
        "UnRegisterTypeLibForUser" => UnRegisterTypeLibForUser(emu),
        "VARIANT_UserFree" => VARIANT_UserFree(emu),
        "VARIANT_UserFree64" => VARIANT_UserFree64(emu),
        "VARIANT_UserMarshal" => VARIANT_UserMarshal(emu),
        "VARIANT_UserMarshal64" => VARIANT_UserMarshal64(emu),
        "VARIANT_UserSize" => VARIANT_UserSize(emu),
        "VARIANT_UserSize64" => VARIANT_UserSize64(emu),
        "VARIANT_UserUnmarshal" => VARIANT_UserUnmarshal(emu),
        "VARIANT_UserUnmarshal64" => VARIANT_UserUnmarshal64(emu),
        "VarAbs" => VarAbs(emu),
        "VarAdd" => VarAdd(emu),
        "VarAnd" => VarAnd(emu),
        "VarBoolFromCy" => VarBoolFromCy(emu),
        "VarBoolFromDate" => VarBoolFromDate(emu),
        "VarBoolFromDec" => VarBoolFromDec(emu),
        "VarBoolFromDisp" => VarBoolFromDisp(emu),
        "VarBoolFromI1" => VarBoolFromI1(emu),
        "VarBoolFromI2" => VarBoolFromI2(emu),
        "VarBoolFromI4" => VarBoolFromI4(emu),
        "VarBoolFromI8" => VarBoolFromI8(emu),
        "VarBoolFromR4" => VarBoolFromR4(emu),
        "VarBoolFromR8" => VarBoolFromR8(emu),
        "VarBoolFromStr" => VarBoolFromStr(emu),
        "VarBoolFromUI1" => VarBoolFromUI1(emu),
        "VarBoolFromUI2" => VarBoolFromUI2(emu),
        "VarBoolFromUI4" => VarBoolFromUI4(emu),
        "VarBoolFromUI8" => VarBoolFromUI8(emu),
        "VarBstrCat" => VarBstrCat(emu),
        "VarBstrCmp" => VarBstrCmp(emu),
        "VarBstrFromBool" => VarBstrFromBool(emu),
        "VarBstrFromCy" => VarBstrFromCy(emu),
        "VarBstrFromDate" => VarBstrFromDate(emu),
        "VarBstrFromDec" => VarBstrFromDec(emu),
        "VarBstrFromDisp" => VarBstrFromDisp(emu),
        "VarBstrFromI1" => VarBstrFromI1(emu),
        "VarBstrFromI2" => VarBstrFromI2(emu),
        "VarBstrFromI4" => VarBstrFromI4(emu),
        "VarBstrFromI8" => VarBstrFromI8(emu),
        "VarBstrFromR4" => VarBstrFromR4(emu),
        "VarBstrFromR8" => VarBstrFromR8(emu),
        "VarBstrFromUI1" => VarBstrFromUI1(emu),
        "VarBstrFromUI2" => VarBstrFromUI2(emu),
        "VarBstrFromUI4" => VarBstrFromUI4(emu),
        "VarBstrFromUI8" => VarBstrFromUI8(emu),
        "VarCat" => VarCat(emu),
        "VarCmp" => VarCmp(emu),
        "VarCyAbs" => VarCyAbs(emu),
        "VarCyAdd" => VarCyAdd(emu),
        "VarCyCmp" => VarCyCmp(emu),
        "VarCyCmpR8" => VarCyCmpR8(emu),
        "VarCyFix" => VarCyFix(emu),
        "VarCyFromBool" => VarCyFromBool(emu),
        "VarCyFromDate" => VarCyFromDate(emu),
        "VarCyFromDec" => VarCyFromDec(emu),
        "VarCyFromDisp" => VarCyFromDisp(emu),
        "VarCyFromI1" => VarCyFromI1(emu),
        "VarCyFromI2" => VarCyFromI2(emu),
        "VarCyFromI4" => VarCyFromI4(emu),
        "VarCyFromI8" => VarCyFromI8(emu),
        "VarCyFromR4" => VarCyFromR4(emu),
        "VarCyFromR8" => VarCyFromR8(emu),
        "VarCyFromStr" => VarCyFromStr(emu),
        "VarCyFromUI1" => VarCyFromUI1(emu),
        "VarCyFromUI2" => VarCyFromUI2(emu),
        "VarCyFromUI4" => VarCyFromUI4(emu),
        "VarCyFromUI8" => VarCyFromUI8(emu),
        "VarCyInt" => VarCyInt(emu),
        "VarCyMul" => VarCyMul(emu),
        "VarCyMulI4" => VarCyMulI4(emu),
        "VarCyMulI8" => VarCyMulI8(emu),
        "VarCyNeg" => VarCyNeg(emu),
        "VarCyRound" => VarCyRound(emu),
        "VarCySub" => VarCySub(emu),
        "VarDateFromBool" => VarDateFromBool(emu),
        "VarDateFromCy" => VarDateFromCy(emu),
        "VarDateFromDec" => VarDateFromDec(emu),
        "VarDateFromDisp" => VarDateFromDisp(emu),
        "VarDateFromI1" => VarDateFromI1(emu),
        "VarDateFromI2" => VarDateFromI2(emu),
        "VarDateFromI4" => VarDateFromI4(emu),
        "VarDateFromI8" => VarDateFromI8(emu),
        "VarDateFromR4" => VarDateFromR4(emu),
        "VarDateFromR8" => VarDateFromR8(emu),
        "VarDateFromStr" => VarDateFromStr(emu),
        "VarDateFromUI1" => VarDateFromUI1(emu),
        "VarDateFromUI2" => VarDateFromUI2(emu),
        "VarDateFromUI4" => VarDateFromUI4(emu),
        "VarDateFromUI8" => VarDateFromUI8(emu),
        "VarDateFromUdate" => VarDateFromUdate(emu),
        "VarDateFromUdateEx" => VarDateFromUdateEx(emu),
        "VarDecAbs" => VarDecAbs(emu),
        "VarDecAdd" => VarDecAdd(emu),
        "VarDecCmp" => VarDecCmp(emu),
        "VarDecCmpR8" => VarDecCmpR8(emu),
        "VarDecDiv" => VarDecDiv(emu),
        "VarDecFix" => VarDecFix(emu),
        "VarDecFromBool" => VarDecFromBool(emu),
        "VarDecFromCy" => VarDecFromCy(emu),
        "VarDecFromDate" => VarDecFromDate(emu),
        "VarDecFromDisp" => VarDecFromDisp(emu),
        "VarDecFromI1" => VarDecFromI1(emu),
        "VarDecFromI2" => VarDecFromI2(emu),
        "VarDecFromI4" => VarDecFromI4(emu),
        "VarDecFromI8" => VarDecFromI8(emu),
        "VarDecFromR4" => VarDecFromR4(emu),
        "VarDecFromR8" => VarDecFromR8(emu),
        "VarDecFromStr" => VarDecFromStr(emu),
        "VarDecFromUI1" => VarDecFromUI1(emu),
        "VarDecFromUI2" => VarDecFromUI2(emu),
        "VarDecFromUI4" => VarDecFromUI4(emu),
        "VarDecFromUI8" => VarDecFromUI8(emu),
        "VarDecInt" => VarDecInt(emu),
        "VarDecMul" => VarDecMul(emu),
        "VarDecNeg" => VarDecNeg(emu),
        "VarDecRound" => VarDecRound(emu),
        "VarDecSub" => VarDecSub(emu),
        "VarDiv" => VarDiv(emu),
        "VarEqv" => VarEqv(emu),
        "VarFix" => VarFix(emu),
        "VarFormat" => VarFormat(emu),
        "VarFormatCurrency" => VarFormatCurrency(emu),
        "VarFormatDateTime" => VarFormatDateTime(emu),
        "VarFormatFromTokens" => VarFormatFromTokens(emu),
        "VarFormatNumber" => VarFormatNumber(emu),
        "VarFormatPercent" => VarFormatPercent(emu),
        "VarI1FromBool" => VarI1FromBool(emu),
        "VarI1FromCy" => VarI1FromCy(emu),
        "VarI1FromDate" => VarI1FromDate(emu),
        "VarI1FromDec" => VarI1FromDec(emu),
        "VarI1FromDisp" => VarI1FromDisp(emu),
        "VarI1FromI2" => VarI1FromI2(emu),
        "VarI1FromI4" => VarI1FromI4(emu),
        "VarI1FromI8" => VarI1FromI8(emu),
        "VarI1FromR4" => VarI1FromR4(emu),
        "VarI1FromR8" => VarI1FromR8(emu),
        "VarI1FromStr" => VarI1FromStr(emu),
        "VarI1FromUI1" => VarI1FromUI1(emu),
        "VarI1FromUI2" => VarI1FromUI2(emu),
        "VarI1FromUI4" => VarI1FromUI4(emu),
        "VarI1FromUI8" => VarI1FromUI8(emu),
        "VarI2FromBool" => VarI2FromBool(emu),
        "VarI2FromCy" => VarI2FromCy(emu),
        "VarI2FromDate" => VarI2FromDate(emu),
        "VarI2FromDec" => VarI2FromDec(emu),
        "VarI2FromDisp" => VarI2FromDisp(emu),
        "VarI2FromI1" => VarI2FromI1(emu),
        "VarI2FromI4" => VarI2FromI4(emu),
        "VarI2FromI8" => VarI2FromI8(emu),
        "VarI2FromR4" => VarI2FromR4(emu),
        "VarI2FromR8" => VarI2FromR8(emu),
        "VarI2FromStr" => VarI2FromStr(emu),
        "VarI2FromUI1" => VarI2FromUI1(emu),
        "VarI2FromUI2" => VarI2FromUI2(emu),
        "VarI2FromUI4" => VarI2FromUI4(emu),
        "VarI2FromUI8" => VarI2FromUI8(emu),
        "VarI4FromBool" => VarI4FromBool(emu),
        "VarI4FromCy" => VarI4FromCy(emu),
        "VarI4FromDate" => VarI4FromDate(emu),
        "VarI4FromDec" => VarI4FromDec(emu),
        "VarI4FromDisp" => VarI4FromDisp(emu),
        "VarI4FromI1" => VarI4FromI1(emu),
        "VarI4FromI2" => VarI4FromI2(emu),
        "VarI4FromI8" => VarI4FromI8(emu),
        "VarI4FromR4" => VarI4FromR4(emu),
        "VarI4FromR8" => VarI4FromR8(emu),
        "VarI4FromStr" => VarI4FromStr(emu),
        "VarI4FromUI1" => VarI4FromUI1(emu),
        "VarI4FromUI2" => VarI4FromUI2(emu),
        "VarI4FromUI4" => VarI4FromUI4(emu),
        "VarI4FromUI8" => VarI4FromUI8(emu),
        "VarI8FromBool" => VarI8FromBool(emu),
        "VarI8FromCy" => VarI8FromCy(emu),
        "VarI8FromDate" => VarI8FromDate(emu),
        "VarI8FromDec" => VarI8FromDec(emu),
        "VarI8FromDisp" => VarI8FromDisp(emu),
        "VarI8FromI1" => VarI8FromI1(emu),
        "VarI8FromI2" => VarI8FromI2(emu),
        "VarI8FromR4" => VarI8FromR4(emu),
        "VarI8FromR8" => VarI8FromR8(emu),
        "VarI8FromStr" => VarI8FromStr(emu),
        "VarI8FromUI1" => VarI8FromUI1(emu),
        "VarI8FromUI2" => VarI8FromUI2(emu),
        "VarI8FromUI4" => VarI8FromUI4(emu),
        "VarI8FromUI8" => VarI8FromUI8(emu),
        "VarIdiv" => VarIdiv(emu),
        "VarImp" => VarImp(emu),
        "VarInt" => VarInt(emu),
        "VarMod" => VarMod(emu),
        "VarMonthName" => VarMonthName(emu),
        "VarMul" => VarMul(emu),
        "VarNeg" => VarNeg(emu),
        "VarNot" => VarNot(emu),
        "VarNumFromParseNum" => VarNumFromParseNum(emu),
        "VarOr" => VarOr(emu),
        "VarParseNumFromStr" => VarParseNumFromStr(emu),
        "VarPow" => VarPow(emu),
        "VarR4CmpR8" => VarR4CmpR8(emu),
        "VarR4FromBool" => VarR4FromBool(emu),
        "VarR4FromCy" => VarR4FromCy(emu),
        "VarR4FromDate" => VarR4FromDate(emu),
        "VarR4FromDec" => VarR4FromDec(emu),
        "VarR4FromDisp" => VarR4FromDisp(emu),
        "VarR4FromI1" => VarR4FromI1(emu),
        "VarR4FromI2" => VarR4FromI2(emu),
        "VarR4FromI4" => VarR4FromI4(emu),
        "VarR4FromI8" => VarR4FromI8(emu),
        "VarR4FromR8" => VarR4FromR8(emu),
        "VarR4FromStr" => VarR4FromStr(emu),
        "VarR4FromUI1" => VarR4FromUI1(emu),
        "VarR4FromUI2" => VarR4FromUI2(emu),
        "VarR4FromUI4" => VarR4FromUI4(emu),
        "VarR4FromUI8" => VarR4FromUI8(emu),
        "VarR8FromBool" => VarR8FromBool(emu),
        "VarR8FromCy" => VarR8FromCy(emu),
        "VarR8FromDate" => VarR8FromDate(emu),
        "VarR8FromDec" => VarR8FromDec(emu),
        "VarR8FromDisp" => VarR8FromDisp(emu),
        "VarR8FromI1" => VarR8FromI1(emu),
        "VarR8FromI2" => VarR8FromI2(emu),
        "VarR8FromI4" => VarR8FromI4(emu),
        "VarR8FromI8" => VarR8FromI8(emu),
        "VarR8FromR4" => VarR8FromR4(emu),
        "VarR8FromStr" => VarR8FromStr(emu),
        "VarR8FromUI1" => VarR8FromUI1(emu),
        "VarR8FromUI2" => VarR8FromUI2(emu),
        "VarR8FromUI4" => VarR8FromUI4(emu),
        "VarR8FromUI8" => VarR8FromUI8(emu),
        "VarR8Pow" => VarR8Pow(emu),
        "VarR8Round" => VarR8Round(emu),
        "VarRound" => VarRound(emu),
        "VarSub" => VarSub(emu),
        "VarTokenizeFormatString" => VarTokenizeFormatString(emu),
        "VarUI1FromBool" => VarUI1FromBool(emu),
        "VarUI1FromCy" => VarUI1FromCy(emu),
        "VarUI1FromDate" => VarUI1FromDate(emu),
        "VarUI1FromDec" => VarUI1FromDec(emu),
        "VarUI1FromDisp" => VarUI1FromDisp(emu),
        "VarUI1FromI1" => VarUI1FromI1(emu),
        "VarUI1FromI2" => VarUI1FromI2(emu),
        "VarUI1FromI4" => VarUI1FromI4(emu),
        "VarUI1FromI8" => VarUI1FromI8(emu),
        "VarUI1FromR4" => VarUI1FromR4(emu),
        "VarUI1FromR8" => VarUI1FromR8(emu),
        "VarUI1FromStr" => VarUI1FromStr(emu),
        "VarUI1FromUI2" => VarUI1FromUI2(emu),
        "VarUI1FromUI4" => VarUI1FromUI4(emu),
        "VarUI1FromUI8" => VarUI1FromUI8(emu),
        "VarUI2FromBool" => VarUI2FromBool(emu),
        "VarUI2FromCy" => VarUI2FromCy(emu),
        "VarUI2FromDate" => VarUI2FromDate(emu),
        "VarUI2FromDec" => VarUI2FromDec(emu),
        "VarUI2FromDisp" => VarUI2FromDisp(emu),
        "VarUI2FromI1" => VarUI2FromI1(emu),
        "VarUI2FromI2" => VarUI2FromI2(emu),
        "VarUI2FromI4" => VarUI2FromI4(emu),
        "VarUI2FromI8" => VarUI2FromI8(emu),
        "VarUI2FromR4" => VarUI2FromR4(emu),
        "VarUI2FromR8" => VarUI2FromR8(emu),
        "VarUI2FromStr" => VarUI2FromStr(emu),
        "VarUI2FromUI1" => VarUI2FromUI1(emu),
        "VarUI2FromUI4" => VarUI2FromUI4(emu),
        "VarUI2FromUI8" => VarUI2FromUI8(emu),
        "VarUI4FromBool" => VarUI4FromBool(emu),
        "VarUI4FromCy" => VarUI4FromCy(emu),
        "VarUI4FromDate" => VarUI4FromDate(emu),
        "VarUI4FromDec" => VarUI4FromDec(emu),
        "VarUI4FromDisp" => VarUI4FromDisp(emu),
        "VarUI4FromI1" => VarUI4FromI1(emu),
        "VarUI4FromI2" => VarUI4FromI2(emu),
        "VarUI4FromI4" => VarUI4FromI4(emu),
        "VarUI4FromI8" => VarUI4FromI8(emu),
        "VarUI4FromR4" => VarUI4FromR4(emu),
        "VarUI4FromR8" => VarUI4FromR8(emu),
        "VarUI4FromStr" => VarUI4FromStr(emu),
        "VarUI4FromUI1" => VarUI4FromUI1(emu),
        "VarUI4FromUI2" => VarUI4FromUI2(emu),
        "VarUI4FromUI8" => VarUI4FromUI8(emu),
        "VarUI8FromBool" => VarUI8FromBool(emu),
        "VarUI8FromCy" => VarUI8FromCy(emu),
        "VarUI8FromDate" => VarUI8FromDate(emu),
        "VarUI8FromDec" => VarUI8FromDec(emu),
        "VarUI8FromDisp" => VarUI8FromDisp(emu),
        "VarUI8FromI1" => VarUI8FromI1(emu),
        "VarUI8FromI2" => VarUI8FromI2(emu),
        "VarUI8FromI8" => VarUI8FromI8(emu),
        "VarUI8FromR4" => VarUI8FromR4(emu),
        "VarUI8FromR8" => VarUI8FromR8(emu),
        "VarUI8FromStr" => VarUI8FromStr(emu),
        "VarUI8FromUI1" => VarUI8FromUI1(emu),
        "VarUI8FromUI2" => VarUI8FromUI2(emu),
        "VarUI8FromUI4" => VarUI8FromUI4(emu),
        "VarUdateFromDate" => VarUdateFromDate(emu),
        "VarWeekdayName" => VarWeekdayName(emu),
        "VarXor" => VarXor(emu),
        "VariantChangeType" => VariantChangeType(emu),
        "VariantChangeTypeEx" => VariantChangeTypeEx(emu),
        "VariantCopy" => VariantCopy(emu),
        "VariantCopyInd" => VariantCopyInd(emu),
        "VariantTimeToDosDateTime" => VariantTimeToDosDateTime(emu),
        "VariantTimeToSystemTime" => VariantTimeToSystemTime(emu),
        "VectorFromBstr" => VectorFromBstr(emu),
        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }
    String::new()
}
