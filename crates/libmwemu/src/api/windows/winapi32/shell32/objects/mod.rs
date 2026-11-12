use crate::emu::Emu;
use crate::winapi::winapi32::shell32;

mod sh_get_data_from_id_list;
pub use sh_get_data_from_id_list::{SHGetDataFromIDListA, SHGetDataFromIDListW};

mod cidl_data_create_from_id_array;
pub use cidl_data_create_from_id_array::CIDLData_CreateFromIDArray;

mod c_storage_item_get_validated_storage_item_object;
pub use c_storage_item_get_validated_storage_item_object::CStorageItem_GetValidatedStorageItemObject;

mod create_storage_item_from_path_full_trust_caller;
pub use create_storage_item_from_path_full_trust_caller::CreateStorageItemFromPath_FullTrustCaller;

mod create_storage_item_from_path_full_trust_caller_for_package;
pub use create_storage_item_from_path_full_trust_caller_for_package::CreateStorageItemFromPath_FullTrustCaller_ForPackage;

mod create_storage_item_from_path_partial_trust_caller;
pub use create_storage_item_from_path_partial_trust_caller::CreateStorageItemFromPath_PartialTrustCaller;

mod create_storage_item_from_shell_item_full_trust_caller;
pub use create_storage_item_from_shell_item_full_trust_caller::CreateStorageItemFromShellItem_FullTrustCaller;

mod create_storage_item_from_shell_item_full_trust_caller_for_package;
pub use create_storage_item_from_shell_item_full_trust_caller_for_package::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage;

mod create_storage_item_from_shell_item_full_trust_caller_use_implicit_flags_and_package;
pub use create_storage_item_from_shell_item_full_trust_caller_use_implicit_flags_and_package::CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage;

mod get_system_persisted_storage_item_list;
pub use get_system_persisted_storage_item_list::GetSystemPersistedStorageItemList;

mod il_append_id;
pub use il_append_id::ILAppendID;

mod il_clone;
pub use il_clone::ILClone;

mod il_clone_first;
pub use il_clone_first::ILCloneFirst;

mod il_combine;
pub use il_combine::ILCombine;

mod il_create_from_path;
pub use il_create_from_path::{ILCreateFromPath, ILCreateFromPathA, ILCreateFromPathW};

mod il_find_child;
pub use il_find_child::ILFindChild;

mod il_find_last_id;
pub use il_find_last_id::ILFindLastID;

mod il_free;
pub use il_free::ILFree;

mod il_get_next;
pub use il_get_next::ILGetNext;

mod il_get_size;
pub use il_get_size::ILGetSize;

mod il_is_equal;
pub use il_is_equal::ILIsEqual;

mod il_is_parent;
pub use il_is_parent::ILIsParent;

mod il_load_from_stream_ex;
pub use il_load_from_stream_ex::ILLoadFromStreamEx;

mod il_remove_last_id;
pub use il_remove_last_id::ILRemoveLastID;

mod il_save_to_stream;
pub use il_save_to_stream::ILSaveToStream;

mod open_reg_stream;
pub use open_reg_stream::OpenRegStream;

mod pif_mgr_close_properties;
pub use pif_mgr_close_properties::PifMgr_CloseProperties;

mod pif_mgr_get_properties;
pub use pif_mgr_get_properties::PifMgr_GetProperties;

mod pif_mgr_open_properties;
pub use pif_mgr_open_properties::PifMgr_OpenProperties;

mod pif_mgr_set_properties;
pub use pif_mgr_set_properties::PifMgr_SetProperties;

mod sh_add_default_properties_by_ext;
pub use sh_add_default_properties_by_ext::SHAddDefaultPropertiesByExt;

mod sh_alloc;
pub use sh_alloc::SHAlloc;

mod sh_bind_to_folder_id_list_parent;
pub use sh_bind_to_folder_id_list_parent::SHBindToFolderIDListParent;

mod sh_bind_to_folder_id_list_parent_ex;
pub use sh_bind_to_folder_id_list_parent_ex::SHBindToFolderIDListParentEx;

mod sh_bind_to_object;
pub use sh_bind_to_object::SHBindToObject;

mod sh_bind_to_parent;
pub use sh_bind_to_parent::SHBindToParent;

mod shclsid_from_string;
pub use shclsid_from_string::SHCLSIDFromString;

mod sh_clone_special_id_list;
pub use sh_clone_special_id_list::SHCloneSpecialIDList;

mod sh_co_create_instance;
pub use sh_co_create_instance::SHCoCreateInstance;

mod sh_co_create_instance_worker;
pub use sh_co_create_instance_worker::SHCoCreateInstanceWorker;

mod sh_create_data_object;
pub use sh_create_data_object::SHCreateDataObject;

mod sh_create_default_properties_op;
pub use sh_create_default_properties_op::SHCreateDefaultPropertiesOp;

mod sh_create_item_from_id_list;
pub use sh_create_item_from_id_list::SHCreateItemFromIDList;

mod sh_create_item_from_parsing_name;
pub use sh_create_item_from_parsing_name::SHCreateItemFromParsingName;

mod sh_create_item_from_relative_name;
pub use sh_create_item_from_relative_name::SHCreateItemFromRelativeName;

mod sh_create_item_in_known_folder;
pub use sh_create_item_in_known_folder::SHCreateItemInKnownFolder;

mod sh_create_item_with_parent;
pub use sh_create_item_with_parent::SHCreateItemWithParent;

mod sh_create_shell_folder_view;
pub use sh_create_shell_folder_view::SHCreateShellFolderView;

mod sh_create_shell_folder_view_ex;
pub use sh_create_shell_folder_view_ex::SHCreateShellFolderViewEx;

mod sh_create_shell_item;
pub use sh_create_shell_item::SHCreateShellItem;

mod sh_create_shell_item_array;
pub use sh_create_shell_item_array::SHCreateShellItemArray;

mod sh_create_shell_item_array_from_data_object;
pub use sh_create_shell_item_array_from_data_object::SHCreateShellItemArrayFromDataObject;

mod sh_create_shell_item_array_from_id_lists;
pub use sh_create_shell_item_array_from_id_lists::SHCreateShellItemArrayFromIDLists;

mod sh_create_shell_item_array_from_shell_item;
pub use sh_create_shell_item_array_from_shell_item::SHCreateShellItemArrayFromShellItem;

mod shell32_c_fill_properties_task_create_instance;
pub use shell32_c_fill_properties_task_create_instance::SHELL32_CFillPropertiesTask_CreateInstance;

mod shell32_pif_mgr_close_properties;
pub use shell32_pif_mgr_close_properties::SHELL32_PifMgr_CloseProperties;

mod shell32_pif_mgr_get_properties;
pub use shell32_pif_mgr_get_properties::SHELL32_PifMgr_GetProperties;

mod shell32_pif_mgr_open_properties;
pub use shell32_pif_mgr_open_properties::SHELL32_PifMgr_OpenProperties;

mod shell32_pif_mgr_set_properties;
pub use shell32_pif_mgr_set_properties::SHELL32_PifMgr_SetProperties;

mod shell32_printjob_get_pidl;
pub use shell32_printjob_get_pidl::SHELL32_Printjob_GetPidl;

mod shell32_sh_create_shell_folder_view;
pub use shell32_sh_create_shell_folder_view::SHELL32_SHCreateShellFolderView;

mod sh_free;
pub use sh_free::SHFree;

mod sh_get_attributes_from_data_object;
pub use sh_get_attributes_from_data_object::SHGetAttributesFromDataObject;

mod sh_get_desktop_folder;
pub use sh_get_desktop_folder::SHGetDesktopFolder;

mod sh_get_id_list_from_object;
pub use sh_get_id_list_from_object::SHGetIDListFromObject;

mod sh_get_instance_explorer;
pub use sh_get_instance_explorer::SHGetInstanceExplorer;

mod sh_get_item_from_data_object;
pub use sh_get_item_from_data_object::SHGetItemFromDataObject;

mod sh_get_item_from_object;
pub use sh_get_item_from_object::SHGetItemFromObject;

mod sh_get_known_folder_id_list;
pub use sh_get_known_folder_id_list::SHGetKnownFolderIDList;

mod sh_get_known_folder_item;
pub use sh_get_known_folder_item::SHGetKnownFolderItem;

mod sh_get_malloc;
pub use sh_get_malloc::SHGetMalloc;

mod sh_get_name_from_id_list;
pub use sh_get_name_from_id_list::SHGetNameFromIDList;

mod sh_get_path_from_id_list;
pub use sh_get_path_from_id_list::{
    SHGetPathFromIDList, SHGetPathFromIDListA, SHGetPathFromIDListW,
};

mod sh_get_path_from_id_list_ex;
pub use sh_get_path_from_id_list_ex::SHGetPathFromIDListEx;

mod sh_get_property_store_from_id_list;
pub use sh_get_property_store_from_id_list::SHGetPropertyStoreFromIDList;

mod sh_get_property_store_from_parsing_name;
pub use sh_get_property_store_from_parsing_name::SHGetPropertyStoreFromParsingName;

mod sh_get_real_idl;
pub use sh_get_real_idl::SHGetRealIDL;

mod sh_get_temporary_property_for_item;
pub use sh_get_temporary_property_for_item::SHGetTemporaryPropertyForItem;

mod shil_create_from_path;
pub use shil_create_from_path::SHILCreateFromPath;

mod sh_multi_file_properties;
pub use sh_multi_file_properties::SHMultiFileProperties;

mod sh_object_properties;
pub use sh_object_properties::SHObjectProperties;

mod sh_open_or_get_folder_view;
pub use sh_open_or_get_folder_view::SHOpenOrGetFolderView;

mod sh_prop_stg_create;
pub use sh_prop_stg_create::SHPropStgCreate;

mod sh_prop_stg_read_multiple;
pub use sh_prop_stg_read_multiple::SHPropStgReadMultiple;

mod sh_prop_stg_write_multiple;
pub use sh_prop_stg_write_multiple::SHPropStgWriteMultiple;

mod sh_set_default_properties;
pub use sh_set_default_properties::SHSetDefaultProperties;

mod sh_set_instance_explorer;
pub use sh_set_instance_explorer::SHSetInstanceExplorer;

mod sh_set_temporary_property_for_item;
pub use sh_set_temporary_property_for_item::SHSetTemporaryPropertyForItem;

mod sh_shell_folder_view_message;
pub use sh_shell_folder_view_message::SHShellFolderView_Message;

mod sh_simple_id_list_from_path;
pub use sh_simple_id_list_from_path::SHSimpleIDListFromPath;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CIDLData_CreateFromIDArray" => shell32::CIDLData_CreateFromIDArray(emu),
        "CStorageItem_GetValidatedStorageItemObject" => {
            shell32::CStorageItem_GetValidatedStorageItemObject(emu)
        }
        "CreateStorageItemFromPath_FullTrustCaller" => {
            shell32::CreateStorageItemFromPath_FullTrustCaller(emu)
        }
        "CreateStorageItemFromPath_FullTrustCaller_ForPackage" => {
            shell32::CreateStorageItemFromPath_FullTrustCaller_ForPackage(emu)
        }
        "CreateStorageItemFromPath_PartialTrustCaller" => {
            shell32::CreateStorageItemFromPath_PartialTrustCaller(emu)
        }
        "CreateStorageItemFromShellItem_FullTrustCaller" => {
            shell32::CreateStorageItemFromShellItem_FullTrustCaller(emu)
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage" => {
            shell32::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage(emu)
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage" => {
            shell32::CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage(emu)
        }
        "GetSystemPersistedStorageItemList" => shell32::GetSystemPersistedStorageItemList(emu),
        "ILAppendID" => shell32::ILAppendID(emu),
        "ILClone" => shell32::ILClone(emu),
        "ILCloneFirst" => shell32::ILCloneFirst(emu),
        "ILCombine" => shell32::ILCombine(emu),
        "ILCreateFromPath" => shell32::ILCreateFromPath(emu),
        "ILCreateFromPathA" => shell32::ILCreateFromPathA(emu),
        "ILCreateFromPathW" => shell32::ILCreateFromPathW(emu),
        "ILFindChild" => shell32::ILFindChild(emu),
        "ILFindLastID" => shell32::ILFindLastID(emu),
        "ILFree" => shell32::ILFree(emu),
        "ILGetNext" => shell32::ILGetNext(emu),
        "ILGetSize" => shell32::ILGetSize(emu),
        "ILIsEqual" => shell32::ILIsEqual(emu),
        "ILIsParent" => shell32::ILIsParent(emu),
        "ILLoadFromStreamEx" => shell32::ILLoadFromStreamEx(emu),
        "ILRemoveLastID" => shell32::ILRemoveLastID(emu),
        "ILSaveToStream" => shell32::ILSaveToStream(emu),
        "OpenRegStream" => shell32::OpenRegStream(emu),
        "PifMgr_CloseProperties" => shell32::PifMgr_CloseProperties(emu),
        "PifMgr_GetProperties" => shell32::PifMgr_GetProperties(emu),
        "PifMgr_OpenProperties" => shell32::PifMgr_OpenProperties(emu),
        "PifMgr_SetProperties" => shell32::PifMgr_SetProperties(emu),
        "SHAddDefaultPropertiesByExt" => shell32::SHAddDefaultPropertiesByExt(emu),
        "SHAlloc" => shell32::SHAlloc(emu),
        "SHBindToFolderIDListParent" => shell32::SHBindToFolderIDListParent(emu),
        "SHBindToFolderIDListParentEx" => shell32::SHBindToFolderIDListParentEx(emu),
        "SHBindToObject" => shell32::SHBindToObject(emu),
        "SHBindToParent" => shell32::SHBindToParent(emu),
        "SHCLSIDFromString" => shell32::SHCLSIDFromString(emu),
        "SHCloneSpecialIDList" => shell32::SHCloneSpecialIDList(emu),
        "SHCoCreateInstance" => shell32::SHCoCreateInstance(emu),
        "SHCoCreateInstanceWorker" => shell32::SHCoCreateInstanceWorker(emu),
        "SHCreateDataObject" => shell32::SHCreateDataObject(emu),
        "SHCreateDefaultPropertiesOp" => shell32::SHCreateDefaultPropertiesOp(emu),
        "SHCreateItemFromIDList" => shell32::SHCreateItemFromIDList(emu),
        "SHCreateItemFromParsingName" => shell32::SHCreateItemFromParsingName(emu),
        "SHCreateItemFromRelativeName" => shell32::SHCreateItemFromRelativeName(emu),
        "SHCreateItemInKnownFolder" => shell32::SHCreateItemInKnownFolder(emu),
        "SHCreateItemWithParent" => shell32::SHCreateItemWithParent(emu),
        "SHCreateShellFolderView" => shell32::SHCreateShellFolderView(emu),
        "SHCreateShellFolderViewEx" => shell32::SHCreateShellFolderViewEx(emu),
        "SHCreateShellItem" => shell32::SHCreateShellItem(emu),
        "SHCreateShellItemArray" => shell32::SHCreateShellItemArray(emu),
        "SHCreateShellItemArrayFromDataObject" => {
            shell32::SHCreateShellItemArrayFromDataObject(emu)
        }
        "SHCreateShellItemArrayFromIDLists" => shell32::SHCreateShellItemArrayFromIDLists(emu),
        "SHCreateShellItemArrayFromShellItem" => shell32::SHCreateShellItemArrayFromShellItem(emu),
        "SHELL32_CFillPropertiesTask_CreateInstance" => {
            shell32::SHELL32_CFillPropertiesTask_CreateInstance(emu)
        }
        "SHELL32_PifMgr_CloseProperties" => shell32::SHELL32_PifMgr_CloseProperties(emu),
        "SHELL32_PifMgr_GetProperties" => shell32::SHELL32_PifMgr_GetProperties(emu),
        "SHELL32_PifMgr_OpenProperties" => shell32::SHELL32_PifMgr_OpenProperties(emu),
        "SHELL32_PifMgr_SetProperties" => shell32::SHELL32_PifMgr_SetProperties(emu),
        "SHELL32_Printjob_GetPidl" => shell32::SHELL32_Printjob_GetPidl(emu),
        "SHELL32_SHCreateShellFolderView" => shell32::SHELL32_SHCreateShellFolderView(emu),
        "SHFree" => shell32::SHFree(emu),
        "SHGetAttributesFromDataObject" => shell32::SHGetAttributesFromDataObject(emu),
        "SHGetDataFromIDListA" => shell32::SHGetDataFromIDListA(emu),
        "SHGetDataFromIDListW" => shell32::SHGetDataFromIDListW(emu),
        "SHGetDesktopFolder" => shell32::SHGetDesktopFolder(emu),
        "SHGetIDListFromObject" => shell32::SHGetIDListFromObject(emu),
        "SHGetInstanceExplorer" => shell32::SHGetInstanceExplorer(emu),
        "SHGetItemFromDataObject" => shell32::SHGetItemFromDataObject(emu),
        "SHGetItemFromObject" => shell32::SHGetItemFromObject(emu),
        "SHGetKnownFolderIDList" => shell32::SHGetKnownFolderIDList(emu),
        "SHGetKnownFolderItem" => shell32::SHGetKnownFolderItem(emu),
        "SHGetMalloc" => shell32::SHGetMalloc(emu),
        "SHGetNameFromIDList" => shell32::SHGetNameFromIDList(emu),
        "SHGetPathFromIDList" => shell32::SHGetPathFromIDList(emu),
        "SHGetPathFromIDListA" => shell32::SHGetPathFromIDListA(emu),
        "SHGetPathFromIDListEx" => shell32::SHGetPathFromIDListEx(emu),
        "SHGetPathFromIDListW" => shell32::SHGetPathFromIDListW(emu),
        "SHGetPropertyStoreFromIDList" => shell32::SHGetPropertyStoreFromIDList(emu),
        "SHGetPropertyStoreFromParsingName" => shell32::SHGetPropertyStoreFromParsingName(emu),
        "SHGetRealIDL" => shell32::SHGetRealIDL(emu),
        "SHGetTemporaryPropertyForItem" => shell32::SHGetTemporaryPropertyForItem(emu),
        "SHILCreateFromPath" => shell32::SHILCreateFromPath(emu),
        "SHMultiFileProperties" => shell32::SHMultiFileProperties(emu),
        "SHObjectProperties" => shell32::SHObjectProperties(emu),
        "SHOpenOrGetFolderView" => shell32::SHOpenOrGetFolderView(emu),
        "SHPropStgCreate" => shell32::SHPropStgCreate(emu),
        "SHPropStgReadMultiple" => shell32::SHPropStgReadMultiple(emu),
        "SHPropStgWriteMultiple" => shell32::SHPropStgWriteMultiple(emu),
        "SHSetDefaultProperties" => shell32::SHSetDefaultProperties(emu),
        "SHSetInstanceExplorer" => shell32::SHSetInstanceExplorer(emu),
        "SHSetTemporaryPropertyForItem" => shell32::SHSetTemporaryPropertyForItem(emu),
        "SHShellFolderView_Message" => shell32::SHShellFolderView_Message(emu),
        "SHSimpleIDListFromPath" => shell32::SHSimpleIDListFromPath(emu),
        _ => return false,
    }
    true
}
