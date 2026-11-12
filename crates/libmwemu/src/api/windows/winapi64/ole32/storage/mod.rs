use crate::emu::Emu;
use crate::winapi::winapi64::ole32;

mod co_get_instance_from_i_storage;
pub use co_get_instance_from_i_storage::CoGetInstanceFromIStorage;

mod co_get_interface_and_release_stream;
pub use co_get_interface_and_release_stream::CoGetInterfaceAndReleaseStream;

mod create_i_lock_bytes_on_h_global;
pub use create_i_lock_bytes_on_h_global::CreateILockBytesOnHGlobal;

mod create_stream_on_h_global;
pub use create_stream_on_h_global::CreateStreamOnHGlobal;

mod fmt_id_to_prop_stg_name;
pub use fmt_id_to_prop_stg_name::FmtIdToPropStgName;

mod free_prop_variant_array;
pub use free_prop_variant_array::FreePropVariantArray;

mod get_convert_stg;
pub use get_convert_stg::GetConvertStg;

mod get_document_bit_stg;
pub use get_document_bit_stg::GetDocumentBitStg;

mod get_h_global_from_i_lock_bytes;
pub use get_h_global_from_i_lock_bytes::GetHGlobalFromILockBytes;

mod get_h_global_from_stream;
pub use get_h_global_from_stream::GetHGlobalFromStream;

mod ole_convert_i_storage_to_olestream;
pub use ole_convert_i_storage_to_olestream::OleConvertIStorageToOLESTREAM;

mod ole_convert_i_storage_to_olestream_ex;
pub use ole_convert_i_storage_to_olestream_ex::OleConvertIStorageToOLESTREAMEx;

mod ole_convert_olestream_to_i_storage;
pub use ole_convert_olestream_to_i_storage::OleConvertOLESTREAMToIStorage;

mod ole_convert_olestream_to_i_storage2;
pub use ole_convert_olestream_to_i_storage2::OleConvertOLESTREAMToIStorage2;

mod ole_convert_olestream_to_i_storage_ex;
pub use ole_convert_olestream_to_i_storage_ex::OleConvertOLESTREAMToIStorageEx;

mod ole_convert_olestream_to_i_storage_ex2;
pub use ole_convert_olestream_to_i_storage_ex2::OleConvertOLESTREAMToIStorageEx2;

mod ole_load_from_stream;
pub use ole_load_from_stream::OleLoadFromStream;

mod ole_save_to_stream;
pub use ole_save_to_stream::OleSaveToStream;

mod open_or_create_stream;
pub use open_or_create_stream::OpenOrCreateStream;

mod prop_stg_name_to_fmt_id;
pub use prop_stg_name_to_fmt_id::PropStgNameToFmtId;

mod prop_sys_alloc_string;
pub use prop_sys_alloc_string::PropSysAllocString;

mod prop_sys_free_string;
pub use prop_sys_free_string::PropSysFreeString;

mod prop_variant_change_type;
pub use prop_variant_change_type::PropVariantChangeType;

mod prop_variant_clear;
pub use prop_variant_clear::PropVariantClear;

mod prop_variant_copy;
pub use prop_variant_copy::PropVariantCopy;

mod read_class_stg;
pub use read_class_stg::ReadClassStg;

mod read_class_stm;
pub use read_class_stm::ReadClassStm;

mod read_fmt_user_type_stg;
pub use read_fmt_user_type_stg::ReadFmtUserTypeStg;

mod read_ole_stg;
pub use read_ole_stg::ReadOleStg;

mod read_storage_properties;
pub use read_storage_properties::ReadStorageProperties;

mod read_string_stream;
pub use read_string_stream::ReadStringStream;

mod release_stg_medium;
pub use release_stg_medium::ReleaseStgMedium;

mod set_convert_stg;
pub use set_convert_stg::SetConvertStg;

mod set_document_bit_stg;
pub use set_document_bit_stg::SetDocumentBitStg;

mod stg_convert_property_to_variant;
pub use stg_convert_property_to_variant::StgConvertPropertyToVariant;

mod stg_convert_variant_to_property;
pub use stg_convert_variant_to_property::StgConvertVariantToProperty;

mod stg_create_docfile;
pub use stg_create_docfile::StgCreateDocfile;

mod stg_create_docfile_on_i_lock_bytes;
pub use stg_create_docfile_on_i_lock_bytes::StgCreateDocfileOnILockBytes;

mod stg_create_prop_set_stg;
pub use stg_create_prop_set_stg::StgCreatePropSetStg;

mod stg_create_prop_stg;
pub use stg_create_prop_stg::StgCreatePropStg;

mod stg_create_storage_ex;
pub use stg_create_storage_ex::StgCreateStorageEx;

mod stg_get_i_fill_lock_bytes_on_file;
pub use stg_get_i_fill_lock_bytes_on_file::StgGetIFillLockBytesOnFile;

mod stg_get_i_fill_lock_bytes_on_i_lock_bytes;
pub use stg_get_i_fill_lock_bytes_on_i_lock_bytes::StgGetIFillLockBytesOnILockBytes;

mod stg_is_storage_file;
pub use stg_is_storage_file::StgIsStorageFile;

mod stg_is_storage_i_lock_bytes;
pub use stg_is_storage_i_lock_bytes::StgIsStorageILockBytes;

mod stg_open_async_docfile_on_i_fill_lock_bytes;
pub use stg_open_async_docfile_on_i_fill_lock_bytes::StgOpenAsyncDocfileOnIFillLockBytes;

mod stg_open_prop_stg;
pub use stg_open_prop_stg::StgOpenPropStg;

mod stg_open_storage;
pub use stg_open_storage::StgOpenStorage;

mod stg_open_storage_ex;
pub use stg_open_storage_ex::StgOpenStorageEx;

mod stg_open_storage_on_handle;
pub use stg_open_storage_on_handle::StgOpenStorageOnHandle;

mod stg_open_storage_on_i_lock_bytes;
pub use stg_open_storage_on_i_lock_bytes::StgOpenStorageOnILockBytes;

mod stg_property_length_as_variant;
pub use stg_property_length_as_variant::StgPropertyLengthAsVariant;

mod stg_set_times;
pub use stg_set_times::StgSetTimes;

mod write_class_stg;
pub use write_class_stg::WriteClassStg;

mod write_class_stm;
pub use write_class_stm::WriteClassStm;

mod write_fmt_user_type_stg;
pub use write_fmt_user_type_stg::WriteFmtUserTypeStg;

mod write_ole_stg;
pub use write_ole_stg::WriteOleStg;

mod write_storage_properties;
pub use write_storage_properties::WriteStorageProperties;

mod write_string_stream;
pub use write_string_stream::WriteStringStream;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CoGetInstanceFromIStorage" => ole32::CoGetInstanceFromIStorage(emu),
        "CoGetInterfaceAndReleaseStream" => ole32::CoGetInterfaceAndReleaseStream(emu),
        "CreateILockBytesOnHGlobal" => ole32::CreateILockBytesOnHGlobal(emu),
        "CreateStreamOnHGlobal" => ole32::CreateStreamOnHGlobal(emu),
        "FmtIdToPropStgName" => ole32::FmtIdToPropStgName(emu),
        "FreePropVariantArray" => ole32::FreePropVariantArray(emu),
        "GetConvertStg" => ole32::GetConvertStg(emu),
        "GetDocumentBitStg" => ole32::GetDocumentBitStg(emu),
        "GetHGlobalFromILockBytes" => ole32::GetHGlobalFromILockBytes(emu),
        "GetHGlobalFromStream" => ole32::GetHGlobalFromStream(emu),
        "OleConvertIStorageToOLESTREAM" => ole32::OleConvertIStorageToOLESTREAM(emu),
        "OleConvertIStorageToOLESTREAMEx" => ole32::OleConvertIStorageToOLESTREAMEx(emu),
        "OleConvertOLESTREAMToIStorage" => ole32::OleConvertOLESTREAMToIStorage(emu),
        "OleConvertOLESTREAMToIStorage2" => ole32::OleConvertOLESTREAMToIStorage2(emu),
        "OleConvertOLESTREAMToIStorageEx" => ole32::OleConvertOLESTREAMToIStorageEx(emu),
        "OleConvertOLESTREAMToIStorageEx2" => ole32::OleConvertOLESTREAMToIStorageEx2(emu),
        "OleLoadFromStream" => ole32::OleLoadFromStream(emu),
        "OleSaveToStream" => ole32::OleSaveToStream(emu),
        "OpenOrCreateStream" => ole32::OpenOrCreateStream(emu),
        "PropStgNameToFmtId" => ole32::PropStgNameToFmtId(emu),
        "PropSysAllocString" => ole32::PropSysAllocString(emu),
        "PropSysFreeString" => ole32::PropSysFreeString(emu),
        "PropVariantChangeType" => ole32::PropVariantChangeType(emu),
        "PropVariantClear" => ole32::PropVariantClear(emu),
        "PropVariantCopy" => ole32::PropVariantCopy(emu),
        "ReadClassStg" => ole32::ReadClassStg(emu),
        "ReadClassStm" => ole32::ReadClassStm(emu),
        "ReadFmtUserTypeStg" => ole32::ReadFmtUserTypeStg(emu),
        "ReadOleStg" => ole32::ReadOleStg(emu),
        "ReadStorageProperties" => ole32::ReadStorageProperties(emu),
        "ReadStringStream" => ole32::ReadStringStream(emu),
        "ReleaseStgMedium" => ole32::ReleaseStgMedium(emu),
        "SetConvertStg" => ole32::SetConvertStg(emu),
        "SetDocumentBitStg" => ole32::SetDocumentBitStg(emu),
        "StgConvertPropertyToVariant" => ole32::StgConvertPropertyToVariant(emu),
        "StgConvertVariantToProperty" => ole32::StgConvertVariantToProperty(emu),
        "StgCreateDocfile" => ole32::StgCreateDocfile(emu),
        "StgCreateDocfileOnILockBytes" => ole32::StgCreateDocfileOnILockBytes(emu),
        "StgCreatePropSetStg" => ole32::StgCreatePropSetStg(emu),
        "StgCreatePropStg" => ole32::StgCreatePropStg(emu),
        "StgCreateStorageEx" => ole32::StgCreateStorageEx(emu),
        "StgGetIFillLockBytesOnFile" => ole32::StgGetIFillLockBytesOnFile(emu),
        "StgGetIFillLockBytesOnILockBytes" => ole32::StgGetIFillLockBytesOnILockBytes(emu),
        "StgIsStorageFile" => ole32::StgIsStorageFile(emu),
        "StgIsStorageILockBytes" => ole32::StgIsStorageILockBytes(emu),
        "StgOpenAsyncDocfileOnIFillLockBytes" => ole32::StgOpenAsyncDocfileOnIFillLockBytes(emu),
        "StgOpenPropStg" => ole32::StgOpenPropStg(emu),
        "StgOpenStorage" => ole32::StgOpenStorage(emu),
        "StgOpenStorageEx" => ole32::StgOpenStorageEx(emu),
        "StgOpenStorageOnHandle" => ole32::StgOpenStorageOnHandle(emu),
        "StgOpenStorageOnILockBytes" => ole32::StgOpenStorageOnILockBytes(emu),
        "StgPropertyLengthAsVariant" => ole32::StgPropertyLengthAsVariant(emu),
        "StgSetTimes" => ole32::StgSetTimes(emu),
        "WriteClassStg" => ole32::WriteClassStg(emu),
        "WriteClassStm" => ole32::WriteClassStm(emu),
        "WriteFmtUserTypeStg" => ole32::WriteFmtUserTypeStg(emu),
        "WriteOleStg" => ole32::WriteOleStg(emu),
        "WriteStorageProperties" => ole32::WriteStorageProperties(emu),
        "WriteStringStream" => ole32::WriteStringStream(emu),
        _ => return false,
    }
    true
}
