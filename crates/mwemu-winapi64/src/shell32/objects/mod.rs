use crate::emu::Emu;
use crate::winapi::winapi64::shell32;

#[path = "../sh_get_data_from_id_list.rs"]
mod sh_get_data_from_id_list;
pub use sh_get_data_from_id_list::*;

#[path = "../cidl_data_create_from_id_array.rs"]
mod cidl_data_create_from_id_array;
pub use cidl_data_create_from_id_array::CIDLData_CreateFromIDArray;

#[path = "../c_storage_item_get_validated_storage_item_object.rs"]
mod c_storage_item_get_validated_storage_item_object;
pub use c_storage_item_get_validated_storage_item_object::CStorageItem_GetValidatedStorageItemObject;

#[path = "../create_storage_item_from_path_full_trust_caller.rs"]
mod create_storage_item_from_path_full_trust_caller;
pub use create_storage_item_from_path_full_trust_caller::CreateStorageItemFromPath_FullTrustCaller;

#[path = "../create_storage_item_from_path_full_trust_caller_for_package.rs"]
mod create_storage_item_from_path_full_trust_caller_for_package;
pub use create_storage_item_from_path_full_trust_caller_for_package::CreateStorageItemFromPath_FullTrustCaller_ForPackage;

#[path = "../create_storage_item_from_path_partial_trust_caller.rs"]
mod create_storage_item_from_path_partial_trust_caller;
pub use create_storage_item_from_path_partial_trust_caller::CreateStorageItemFromPath_PartialTrustCaller;

#[path = "../create_storage_item_from_shell_item_full_trust_caller.rs"]
mod create_storage_item_from_shell_item_full_trust_caller;
pub use create_storage_item_from_shell_item_full_trust_caller::CreateStorageItemFromShellItem_FullTrustCaller;

#[path = "../create_storage_item_from_shell_item_full_trust_caller_for_package.rs"]
mod create_storage_item_from_shell_item_full_trust_caller_for_package;
pub use create_storage_item_from_shell_item_full_trust_caller_for_package::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage;

#[path = "../create_storage_item_from_shell_item_full_trust_caller_use_implicit_flags_and_package.rs"]
mod create_storage_item_from_shell_item_full_trust_caller_use_implicit_flags_and_package;
pub use create_storage_item_from_shell_item_full_trust_caller_use_implicit_flags_and_package::CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage;

#[path = "../get_system_persisted_storage_item_list.rs"]
mod get_system_persisted_storage_item_list;
pub use get_system_persisted_storage_item_list::GetSystemPersistedStorageItemList;

#[path = "../il_append_id.rs"]
mod il_append_id;
pub use il_append_id::ILAppendID;

#[path = "../il_clone.rs"]
mod il_clone;
pub use il_clone::ILClone;

#[path = "../il_clone_first.rs"]
mod il_clone_first;
pub use il_clone_first::ILCloneFirst;

#[path = "../il_combine.rs"]
mod il_combine;
pub use il_combine::ILCombine;

#[path = "../il_create_from_path.rs"]
mod il_create_from_path;
pub use il_create_from_path::ILCreateFromPath;

#[path = "../il_create_from_path_a.rs"]
mod il_create_from_path_a;
pub use il_create_from_path_a::ILCreateFromPathA;

#[path = "../il_create_from_path_w.rs"]
mod il_create_from_path_w;
pub use il_create_from_path_w::ILCreateFromPathW;

#[path = "../il_find_child.rs"]
mod il_find_child;
pub use il_find_child::ILFindChild;

#[path = "../il_find_last_id.rs"]
mod il_find_last_id;
pub use il_find_last_id::ILFindLastID;

#[path = "../il_free.rs"]
mod il_free;
pub use il_free::ILFree;

#[path = "../il_get_next.rs"]
mod il_get_next;
pub use il_get_next::ILGetNext;

#[path = "../il_get_size.rs"]
mod il_get_size;
pub use il_get_size::ILGetSize;

#[path = "../il_is_equal.rs"]
mod il_is_equal;
pub use il_is_equal::ILIsEqual;

#[path = "../il_is_parent.rs"]
mod il_is_parent;
pub use il_is_parent::ILIsParent;

#[path = "../il_load_from_stream_ex.rs"]
mod il_load_from_stream_ex;
pub use il_load_from_stream_ex::ILLoadFromStreamEx;

#[path = "../il_remove_last_id.rs"]
mod il_remove_last_id;
pub use il_remove_last_id::ILRemoveLastID;

#[path = "../il_save_to_stream.rs"]
mod il_save_to_stream;
pub use il_save_to_stream::ILSaveToStream;

#[path = "../open_reg_stream.rs"]
mod open_reg_stream;
pub use open_reg_stream::OpenRegStream;

#[path = "../pif_mgr_close_properties.rs"]
mod pif_mgr_close_properties;
pub use pif_mgr_close_properties::PifMgr_CloseProperties;

#[path = "../pif_mgr_get_properties.rs"]
mod pif_mgr_get_properties;
pub use pif_mgr_get_properties::PifMgr_GetProperties;

#[path = "../pif_mgr_open_properties.rs"]
mod pif_mgr_open_properties;
pub use pif_mgr_open_properties::PifMgr_OpenProperties;

#[path = "../pif_mgr_set_properties.rs"]
mod pif_mgr_set_properties;
pub use pif_mgr_set_properties::PifMgr_SetProperties;

#[path = "../sh_add_default_properties_by_ext.rs"]
mod sh_add_default_properties_by_ext;
pub use sh_add_default_properties_by_ext::SHAddDefaultPropertiesByExt;

#[path = "../sh_alloc.rs"]
mod sh_alloc;
pub use sh_alloc::SHAlloc;

#[path = "../sh_bind_to_folder_id_list_parent.rs"]
mod sh_bind_to_folder_id_list_parent;
pub use sh_bind_to_folder_id_list_parent::SHBindToFolderIDListParent;

#[path = "../sh_bind_to_folder_id_list_parent_ex.rs"]
mod sh_bind_to_folder_id_list_parent_ex;
pub use sh_bind_to_folder_id_list_parent_ex::SHBindToFolderIDListParentEx;

#[path = "../sh_bind_to_object.rs"]
mod sh_bind_to_object;
pub use sh_bind_to_object::SHBindToObject;

#[path = "../sh_bind_to_parent.rs"]
mod sh_bind_to_parent;
pub use sh_bind_to_parent::SHBindToParent;

#[path = "../shclsid_from_string.rs"]
mod shclsid_from_string;
pub use shclsid_from_string::SHCLSIDFromString;

#[path = "../sh_clone_special_id_list.rs"]
mod sh_clone_special_id_list;
pub use sh_clone_special_id_list::SHCloneSpecialIDList;

#[path = "../sh_co_create_instance.rs"]
mod sh_co_create_instance;
pub use sh_co_create_instance::SHCoCreateInstance;

#[path = "../sh_co_create_instance_worker.rs"]
mod sh_co_create_instance_worker;
pub use sh_co_create_instance_worker::SHCoCreateInstanceWorker;

#[path = "../sh_create_data_object.rs"]
mod sh_create_data_object;
pub use sh_create_data_object::SHCreateDataObject;

#[path = "../sh_create_default_properties_op.rs"]
mod sh_create_default_properties_op;
pub use sh_create_default_properties_op::SHCreateDefaultPropertiesOp;

#[path = "../sh_create_item_from_id_list.rs"]
mod sh_create_item_from_id_list;
pub use sh_create_item_from_id_list::SHCreateItemFromIDList;

#[path = "../sh_create_item_from_parsing_name.rs"]
mod sh_create_item_from_parsing_name;
pub use sh_create_item_from_parsing_name::SHCreateItemFromParsingName;

#[path = "../sh_create_item_from_relative_name.rs"]
mod sh_create_item_from_relative_name;
pub use sh_create_item_from_relative_name::SHCreateItemFromRelativeName;

#[path = "../sh_create_item_in_known_folder.rs"]
mod sh_create_item_in_known_folder;
pub use sh_create_item_in_known_folder::SHCreateItemInKnownFolder;

#[path = "../sh_create_item_with_parent.rs"]
mod sh_create_item_with_parent;
pub use sh_create_item_with_parent::SHCreateItemWithParent;

#[path = "../sh_create_shell_folder_view.rs"]
mod sh_create_shell_folder_view;
pub use sh_create_shell_folder_view::SHCreateShellFolderView;

#[path = "../sh_create_shell_folder_view_ex.rs"]
mod sh_create_shell_folder_view_ex;
pub use sh_create_shell_folder_view_ex::SHCreateShellFolderViewEx;

#[path = "../sh_create_shell_item.rs"]
mod sh_create_shell_item;
pub use sh_create_shell_item::SHCreateShellItem;

#[path = "../sh_create_shell_item_array.rs"]
mod sh_create_shell_item_array;
pub use sh_create_shell_item_array::SHCreateShellItemArray;

#[path = "../sh_create_shell_item_array_from_data_object.rs"]
mod sh_create_shell_item_array_from_data_object;
pub use sh_create_shell_item_array_from_data_object::SHCreateShellItemArrayFromDataObject;

#[path = "../sh_create_shell_item_array_from_id_lists.rs"]
mod sh_create_shell_item_array_from_id_lists;
pub use sh_create_shell_item_array_from_id_lists::SHCreateShellItemArrayFromIDLists;

#[path = "../sh_create_shell_item_array_from_shell_item.rs"]
mod sh_create_shell_item_array_from_shell_item;
pub use sh_create_shell_item_array_from_shell_item::SHCreateShellItemArrayFromShellItem;

#[path = "../shell32_c_fill_properties_task_create_instance.rs"]
mod shell32_c_fill_properties_task_create_instance;
pub use shell32_c_fill_properties_task_create_instance::SHELL32_CFillPropertiesTask_CreateInstance;

#[path = "../shell32_pif_mgr_close_properties.rs"]
mod shell32_pif_mgr_close_properties;
pub use shell32_pif_mgr_close_properties::SHELL32_PifMgr_CloseProperties;

#[path = "../shell32_pif_mgr_get_properties.rs"]
mod shell32_pif_mgr_get_properties;
pub use shell32_pif_mgr_get_properties::SHELL32_PifMgr_GetProperties;

#[path = "../shell32_pif_mgr_open_properties.rs"]
mod shell32_pif_mgr_open_properties;
pub use shell32_pif_mgr_open_properties::SHELL32_PifMgr_OpenProperties;

#[path = "../shell32_pif_mgr_set_properties.rs"]
mod shell32_pif_mgr_set_properties;
pub use shell32_pif_mgr_set_properties::SHELL32_PifMgr_SetProperties;

#[path = "../shell32_printjob_get_pidl.rs"]
mod shell32_printjob_get_pidl;
pub use shell32_printjob_get_pidl::SHELL32_Printjob_GetPidl;

#[path = "../shell32_sh_create_shell_folder_view.rs"]
mod shell32_sh_create_shell_folder_view;
pub use shell32_sh_create_shell_folder_view::SHELL32_SHCreateShellFolderView;

#[path = "../sh_free.rs"]
mod sh_free;
pub use sh_free::SHFree;

#[path = "../sh_get_attributes_from_data_object.rs"]
mod sh_get_attributes_from_data_object;
pub use sh_get_attributes_from_data_object::SHGetAttributesFromDataObject;

#[path = "../sh_get_desktop_folder.rs"]
mod sh_get_desktop_folder;
pub use sh_get_desktop_folder::SHGetDesktopFolder;

#[path = "../sh_get_id_list_from_object.rs"]
mod sh_get_id_list_from_object;
pub use sh_get_id_list_from_object::SHGetIDListFromObject;

#[path = "../sh_get_instance_explorer.rs"]
mod sh_get_instance_explorer;
pub use sh_get_instance_explorer::SHGetInstanceExplorer;

#[path = "../sh_get_item_from_data_object.rs"]
mod sh_get_item_from_data_object;
pub use sh_get_item_from_data_object::SHGetItemFromDataObject;

#[path = "../sh_get_item_from_object.rs"]
mod sh_get_item_from_object;
pub use sh_get_item_from_object::SHGetItemFromObject;

#[path = "../sh_get_known_folder_id_list.rs"]
mod sh_get_known_folder_id_list;
pub use sh_get_known_folder_id_list::SHGetKnownFolderIDList;

#[path = "../sh_get_known_folder_item.rs"]
mod sh_get_known_folder_item;
pub use sh_get_known_folder_item::SHGetKnownFolderItem;

#[path = "../sh_get_malloc.rs"]
mod sh_get_malloc;
pub use sh_get_malloc::SHGetMalloc;

#[path = "../sh_get_name_from_id_list.rs"]
mod sh_get_name_from_id_list;
pub use sh_get_name_from_id_list::SHGetNameFromIDList;

#[path = "../sh_get_path_from_id_list.rs"]
mod sh_get_path_from_id_list;
pub use sh_get_path_from_id_list::SHGetPathFromIDList;

#[path = "../sh_get_path_from_id_list_a.rs"]
mod sh_get_path_from_id_list_a;
pub use sh_get_path_from_id_list_a::SHGetPathFromIDListA;

#[path = "../sh_get_path_from_id_list_ex.rs"]
mod sh_get_path_from_id_list_ex;
pub use sh_get_path_from_id_list_ex::SHGetPathFromIDListEx;

#[path = "../sh_get_path_from_id_list_w.rs"]
mod sh_get_path_from_id_list_w;
pub use sh_get_path_from_id_list_w::SHGetPathFromIDListW;

#[path = "../sh_get_property_store_from_id_list.rs"]
mod sh_get_property_store_from_id_list;
pub use sh_get_property_store_from_id_list::SHGetPropertyStoreFromIDList;

#[path = "../sh_get_property_store_from_parsing_name.rs"]
mod sh_get_property_store_from_parsing_name;
pub use sh_get_property_store_from_parsing_name::SHGetPropertyStoreFromParsingName;

#[path = "../sh_get_real_idl.rs"]
mod sh_get_real_idl;
pub use sh_get_real_idl::SHGetRealIDL;

#[path = "../sh_get_temporary_property_for_item.rs"]
mod sh_get_temporary_property_for_item;
pub use sh_get_temporary_property_for_item::SHGetTemporaryPropertyForItem;

#[path = "../shil_create_from_path.rs"]
mod shil_create_from_path;
pub use shil_create_from_path::SHILCreateFromPath;

#[path = "../sh_multi_file_properties.rs"]
mod sh_multi_file_properties;
pub use sh_multi_file_properties::SHMultiFileProperties;

#[path = "../sh_object_properties.rs"]
mod sh_object_properties;
pub use sh_object_properties::SHObjectProperties;

#[path = "../sh_open_or_get_folder_view.rs"]
mod sh_open_or_get_folder_view;
pub use sh_open_or_get_folder_view::SHOpenOrGetFolderView;

#[path = "../sh_prop_stg_create.rs"]
mod sh_prop_stg_create;
pub use sh_prop_stg_create::SHPropStgCreate;

#[path = "../sh_prop_stg_read_multiple.rs"]
mod sh_prop_stg_read_multiple;
pub use sh_prop_stg_read_multiple::SHPropStgReadMultiple;

#[path = "../sh_prop_stg_write_multiple.rs"]
mod sh_prop_stg_write_multiple;
pub use sh_prop_stg_write_multiple::SHPropStgWriteMultiple;

#[path = "../sh_set_default_properties.rs"]
mod sh_set_default_properties;
pub use sh_set_default_properties::SHSetDefaultProperties;

#[path = "../sh_set_instance_explorer.rs"]
mod sh_set_instance_explorer;
pub use sh_set_instance_explorer::SHSetInstanceExplorer;

#[path = "../sh_set_temporary_property_for_item.rs"]
mod sh_set_temporary_property_for_item;
pub use sh_set_temporary_property_for_item::SHSetTemporaryPropertyForItem;

#[path = "../sh_shell_folder_view_message.rs"]
mod sh_shell_folder_view_message;
pub use sh_shell_folder_view_message::SHShellFolderView_Message;

#[path = "../sh_simple_id_list_from_path.rs"]
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
