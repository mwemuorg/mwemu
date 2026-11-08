use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod activate_keyboard_layout;
pub use activate_keyboard_layout::ActivateKeyboardLayout;
mod add_clipboard_format_listener;
pub use add_clipboard_format_listener::AddClipboardFormatListener;
mod add_visual_identifier;
pub use add_visual_identifier::AddVisualIdentifier;
mod adjust_window_rect_ex_for_dpi;
pub use adjust_window_rect_ex_for_dpi::AdjustWindowRectExForDpi;
mod align_rects;
pub use align_rects::AlignRects;
mod allow_foreground_activation;
pub use allow_foreground_activation::AllowForegroundActivation;
mod allow_set_foreground_window;
pub use allow_set_foreground_window::AllowSetForegroundWindow;
mod animate_window;
pub use animate_window::AnimateWindow;
mod any_popup;
pub use any_popup::AnyPopup;
mod append_menu_a;
pub use append_menu_a::AppendMenuA;
mod append_menu_w;
pub use append_menu_w::AppendMenuW;
mod apply_window_action;
pub use apply_window_action::ApplyWindowAction;
mod are_dpi_awareness_contexts_equal;
pub use are_dpi_awareness_contexts_equal::AreDpiAwarenessContextsEqual;
mod arrange_iconic_windows;
pub use arrange_iconic_windows::ArrangeIconicWindows;
mod attach_thread_input;
pub use attach_thread_input::AttachThreadInput;
mod begin_defer_window_pos;
pub use begin_defer_window_pos::BeginDeferWindowPos;
mod block_input;
pub use block_input::BlockInput;
mod bring_window_to_top;
pub use bring_window_to_top::BringWindowToTop;
mod broadcast_system_message;
pub use broadcast_system_message::BroadcastSystemMessage;
mod broadcast_system_message_a;
pub use broadcast_system_message_a::BroadcastSystemMessageA;
mod broadcast_system_message_ex_a;
pub use broadcast_system_message_ex_a::BroadcastSystemMessageExA;
mod broadcast_system_message_ex_w;
pub use broadcast_system_message_ex_w::BroadcastSystemMessageExW;
mod broadcast_system_message_w;
pub use broadcast_system_message_w::BroadcastSystemMessageW;
mod build_reason_array;
pub use build_reason_array::BuildReasonArray;
mod calc_menu_bar;
pub use calc_menu_bar::CalcMenuBar;
mod calculate_popup_window_position;
pub use calculate_popup_window_position::CalculatePopupWindowPosition;
mod call_msg_filter;
pub use call_msg_filter::CallMsgFilter;
mod call_msg_filter_a;
pub use call_msg_filter_a::CallMsgFilterA;
mod call_msg_filter_w;
pub use call_msg_filter_w::CallMsgFilterW;
mod call_next_hook_ex;
pub use call_next_hook_ex::CallNextHookEx;
mod call_window_proc_a;
pub use call_window_proc_a::CallWindowProcA;
mod call_window_proc_w;
pub use call_window_proc_w::CallWindowProcW;
mod cancel_shutdown;
pub use cancel_shutdown::CancelShutdown;
mod cascade_child_windows;
pub use cascade_child_windows::CascadeChildWindows;
mod cascade_windows;
pub use cascade_windows::CascadeWindows;
mod change_clipboard_chain;
pub use change_clipboard_chain::ChangeClipboardChain;
mod change_display_settings_a;
pub use change_display_settings_a::ChangeDisplaySettingsA;
mod change_display_settings_ex_a;
pub use change_display_settings_ex_a::ChangeDisplaySettingsExA;
mod change_display_settings_ex_w;
pub use change_display_settings_ex_w::ChangeDisplaySettingsExW;
mod change_display_settings_w;
pub use change_display_settings_w::ChangeDisplaySettingsW;
mod change_menu_a;
pub use change_menu_a::ChangeMenuA;
mod change_menu_w;
pub use change_menu_w::ChangeMenuW;
mod change_window_message_filter;
pub use change_window_message_filter::ChangeWindowMessageFilter;
mod change_window_message_filter_ex;
pub use change_window_message_filter_ex::ChangeWindowMessageFilterEx;
mod char_lower_a;
pub use char_lower_a::CharLowerA;
mod char_next_a;
pub use char_next_a::CharNextA;
mod char_next_ex_a;
pub use char_next_ex_a::CharNextExA;
mod char_next_w;
pub use char_next_w::CharNextW;
mod char_prev_a;
pub use char_prev_a::CharPrevA;
mod char_prev_ex_a;
pub use char_prev_ex_a::CharPrevExA;
mod char_prev_w;
pub use char_prev_w::CharPrevW;
mod char_to_oem_a;
pub use char_to_oem_a::CharToOemA;
mod char_to_oem_buff_a;
pub use char_to_oem_buff_a::CharToOemBuffA;
mod char_to_oem_buff_w;
pub use char_to_oem_buff_w::CharToOemBuffW;
mod char_to_oem_w;
pub use char_to_oem_w::CharToOemW;
mod char_upper_a;
pub use char_upper_a::CharUpperA;
mod char_upper_w;
pub use char_upper_w::CharUpperW;
mod check_banned_one_core_transform_api;
pub use check_banned_one_core_transform_api::CheckBannedOneCoreTransformApi;
mod check_dbcs_enabled_ext;
pub use check_dbcs_enabled_ext::CheckDBCSEnabledExt;
mod check_dlg_button;
pub use check_dlg_button::CheckDlgButton;
mod check_menu_item;
pub use check_menu_item::CheckMenuItem;
mod check_menu_radio_item;
pub use check_menu_radio_item::CheckMenuRadioItem;
mod check_process_for_clipboard_access;
pub use check_process_for_clipboard_access::CheckProcessForClipboardAccess;
mod check_process_session;
pub use check_process_session::CheckProcessSession;
mod check_radio_button;
pub use check_radio_button::CheckRadioButton;
mod check_window_thread_desktop;
pub use check_window_thread_desktop::CheckWindowThreadDesktop;
mod child_window_from_point;
pub use child_window_from_point::ChildWindowFromPoint;
mod child_window_from_point_ex;
pub use child_window_from_point_ex::ChildWindowFromPointEx;
mod cli_imm_set_hot_key;
pub use cli_imm_set_hot_key::CliImmSetHotKey;
mod client_thread_setup;
pub use client_thread_setup::ClientThreadSetup;
mod client_to_screen;
pub use client_to_screen::ClientToScreen;
mod clip_cursor;
pub use clip_cursor::ClipCursor;
mod close_clipboard;
pub use close_clipboard::CloseClipboard;
mod close_desktop;
pub use close_desktop::CloseDesktop;
mod close_gesture_info_handle;
pub use close_gesture_info_handle::CloseGestureInfoHandle;
mod close_touch_input_handle;
pub use close_touch_input_handle::CloseTouchInputHandle;
mod close_window;
pub use close_window::CloseWindow;
mod close_window_station;
pub use close_window_station::CloseWindowStation;
mod console_control;
pub use console_control::ConsoleControl;
mod control_magnification;
pub use control_magnification::ControlMagnification;
mod convert_primary_pointer_to_mouse_drag;
pub use convert_primary_pointer_to_mouse_drag::ConvertPrimaryPointerToMouseDrag;
mod convert_to_intercept_window;
pub use convert_to_intercept_window::ConvertToInterceptWindow;
mod copy_accelerator_table_a;
pub use copy_accelerator_table_a::CopyAcceleratorTableA;
mod copy_accelerator_table_w;
pub use copy_accelerator_table_w::CopyAcceleratorTableW;
mod copy_icon;
pub use copy_icon::CopyIcon;
mod copy_image;
pub use copy_image::CopyImage;
mod copy_rect;
pub use copy_rect::CopyRect;
mod count_clipboard_formats;
pub use count_clipboard_formats::CountClipboardFormats;
mod create_accelerator_table_a;
pub use create_accelerator_table_a::CreateAcceleratorTableA;
mod create_accelerator_table_w;
pub use create_accelerator_table_w::CreateAcceleratorTableW;
mod create_caret;
pub use create_caret::CreateCaret;
mod create_cursor;
pub use create_cursor::CreateCursor;
mod create_d_composition_hwnd_target;
pub use create_d_composition_hwnd_target::CreateDCompositionHwndTarget;
mod create_desktop_a;
pub use create_desktop_a::CreateDesktopA;
mod create_desktop_ex_a;
pub use create_desktop_ex_a::CreateDesktopExA;
mod create_desktop_ex_w;
pub use create_desktop_ex_w::CreateDesktopExW;
mod create_desktop_w;
pub use create_desktop_w::CreateDesktopW;
mod create_dialog_indirect_param_a;
pub use create_dialog_indirect_param_a::CreateDialogIndirectParamA;
mod create_dialog_indirect_param_aor_w;
pub use create_dialog_indirect_param_aor_w::CreateDialogIndirectParamAorW;
mod create_dialog_indirect_param_w;
pub use create_dialog_indirect_param_w::CreateDialogIndirectParamW;
mod create_dialog_param_a;
pub use create_dialog_param_a::CreateDialogParamA;
mod create_dialog_param_w;
pub use create_dialog_param_w::CreateDialogParamW;
mod create_icon;
pub use create_icon::CreateIcon;
mod create_icon_from_resource;
pub use create_icon_from_resource::CreateIconFromResource;
mod create_icon_from_resource_ex;
pub use create_icon_from_resource_ex::CreateIconFromResourceEx;
mod create_icon_indirect;
pub use create_icon_indirect::CreateIconIndirect;
mod create_mdi_window_a;
pub use create_mdi_window_a::CreateMDIWindowA;
mod create_mdi_window_w;
pub use create_mdi_window_w::CreateMDIWindowW;
mod create_menu;
pub use create_menu::CreateMenu;
mod create_popup_menu;
pub use create_popup_menu::CreatePopupMenu;
mod create_synthetic_pointer_device;
pub use create_synthetic_pointer_device::CreateSyntheticPointerDevice;
mod create_synthetic_pointer_device2;
pub use create_synthetic_pointer_device2::CreateSyntheticPointerDevice2;
mod create_system_threads;
pub use create_system_threads::CreateSystemThreads;
mod create_window_in_band;
pub use create_window_in_band::CreateWindowInBand;
mod create_window_in_band_ex;
pub use create_window_in_band_ex::CreateWindowInBandEx;
mod create_window_indirect;
pub use create_window_indirect::CreateWindowIndirect;
mod create_window_station_a;
pub use create_window_station_a::CreateWindowStationA;
mod create_window_station_w;
pub use create_window_station_w::CreateWindowStationW;
mod csr_broadcast_system_message_ex_w;
pub use csr_broadcast_system_message_ex_w::CsrBroadcastSystemMessageExW;
mod ctx_init_user32;
pub use ctx_init_user32::CtxInitUser32;
mod dde_abandon_transaction;
pub use dde_abandon_transaction::DdeAbandonTransaction;
mod dde_access_data;
pub use dde_access_data::DdeAccessData;
mod dde_add_data;
pub use dde_add_data::DdeAddData;
mod dde_client_transaction;
pub use dde_client_transaction::DdeClientTransaction;
mod dde_cmp_string_handles;
pub use dde_cmp_string_handles::DdeCmpStringHandles;
mod dde_connect;
pub use dde_connect::DdeConnect;
mod dde_connect_list;
pub use dde_connect_list::DdeConnectList;
mod dde_create_data_handle;
pub use dde_create_data_handle::DdeCreateDataHandle;
mod dde_create_string_handle_a;
pub use dde_create_string_handle_a::DdeCreateStringHandleA;
mod dde_create_string_handle_w;
pub use dde_create_string_handle_w::DdeCreateStringHandleW;
mod dde_disconnect;
pub use dde_disconnect::DdeDisconnect;
mod dde_disconnect_list;
pub use dde_disconnect_list::DdeDisconnectList;
mod dde_enable_callback;
pub use dde_enable_callback::DdeEnableCallback;
mod dde_free_data_handle;
pub use dde_free_data_handle::DdeFreeDataHandle;
mod dde_free_string_handle;
pub use dde_free_string_handle::DdeFreeStringHandle;
mod dde_get_data;
pub use dde_get_data::DdeGetData;
mod dde_get_last_error;
pub use dde_get_last_error::DdeGetLastError;
mod dde_get_quality_of_service;
pub use dde_get_quality_of_service::DdeGetQualityOfService;
mod dde_impersonate_client;
pub use dde_impersonate_client::DdeImpersonateClient;
mod dde_initialize_a;
pub use dde_initialize_a::DdeInitializeA;
mod dde_initialize_w;
pub use dde_initialize_w::DdeInitializeW;
mod dde_keep_string_handle;
pub use dde_keep_string_handle::DdeKeepStringHandle;
mod dde_name_service;
pub use dde_name_service::DdeNameService;
mod dde_post_advise;
pub use dde_post_advise::DdePostAdvise;
mod dde_query_conv_info;
pub use dde_query_conv_info::DdeQueryConvInfo;
mod dde_query_next_server;
pub use dde_query_next_server::DdeQueryNextServer;
mod dde_query_string_a;
pub use dde_query_string_a::DdeQueryStringA;
mod dde_query_string_w;
pub use dde_query_string_w::DdeQueryStringW;
mod dde_reconnect;
pub use dde_reconnect::DdeReconnect;
mod dde_set_quality_of_service;
pub use dde_set_quality_of_service::DdeSetQualityOfService;
mod dde_set_user_handle;
pub use dde_set_user_handle::DdeSetUserHandle;
mod dde_unaccess_data;
pub use dde_unaccess_data::DdeUnaccessData;
mod dde_uninitialize;
pub use dde_uninitialize::DdeUninitialize;
mod def_dlg_proc_a;
pub use def_dlg_proc_a::DefDlgProcA;
mod def_dlg_proc_w;
pub use def_dlg_proc_w::DefDlgProcW;
mod def_frame_proc_a;
pub use def_frame_proc_a::DefFrameProcA;
mod def_frame_proc_w;
pub use def_frame_proc_w::DefFrameProcW;
mod def_mdi_child_proc_a;
pub use def_mdi_child_proc_a::DefMDIChildProcA;
mod def_mdi_child_proc_w;
pub use def_mdi_child_proc_w::DefMDIChildProcW;
mod def_raw_input_proc;
pub use def_raw_input_proc::DefRawInputProc;
mod defer_window_pos;
pub use defer_window_pos::DeferWindowPos;
mod defer_window_pos_and_band;
pub use defer_window_pos_and_band::DeferWindowPosAndBand;
mod delegate_input;
pub use delegate_input::DelegateInput;
mod delete_menu;
pub use delete_menu::DeleteMenu;
mod deregister_shell_hook_window;
pub use deregister_shell_hook_window::DeregisterShellHookWindow;
mod destroy_accelerator_table;
pub use destroy_accelerator_table::DestroyAcceleratorTable;
mod destroy_caret;
pub use destroy_caret::DestroyCaret;
mod destroy_cursor;
pub use destroy_cursor::DestroyCursor;
mod destroy_d_composition_hwnd_target;
pub use destroy_d_composition_hwnd_target::DestroyDCompositionHwndTarget;
mod destroy_icon;
pub use destroy_icon::DestroyIcon;
mod destroy_menu;
pub use destroy_menu::DestroyMenu;
mod destroy_reasons;
pub use destroy_reasons::DestroyReasons;
mod destroy_synthetic_pointer_device;
pub use destroy_synthetic_pointer_device::DestroySyntheticPointerDevice;
mod dialog_box_indirect_param_a;
pub use dialog_box_indirect_param_a::DialogBoxIndirectParamA;
mod dialog_box_indirect_param_aor_w;
pub use dialog_box_indirect_param_aor_w::DialogBoxIndirectParamAorW;
mod dialog_box_indirect_param_w;
pub use dialog_box_indirect_param_w::DialogBoxIndirectParamW;
mod dialog_box_param_a;
pub use dialog_box_param_a::DialogBoxParamA;
mod dialog_box_param_w;
pub use dialog_box_param_w::DialogBoxParamW;
mod disable_process_windows_ghosting;
pub use disable_process_windows_ghosting::DisableProcessWindowsGhosting;
mod display_config_get_device_info;
pub use display_config_get_device_info::DisplayConfigGetDeviceInfo;
mod display_config_set_device_info;
pub use display_config_set_device_info::DisplayConfigSetDeviceInfo;
mod display_exit_windows_warnings;
pub use display_exit_windows_warnings::DisplayExitWindowsWarnings;
mod dlg_dir_list_a;
pub use dlg_dir_list_a::DlgDirListA;
mod dlg_dir_list_combo_box_a;
pub use dlg_dir_list_combo_box_a::DlgDirListComboBoxA;
mod dlg_dir_list_combo_box_w;
pub use dlg_dir_list_combo_box_w::DlgDirListComboBoxW;
mod dlg_dir_list_w;
pub use dlg_dir_list_w::DlgDirListW;
mod dlg_dir_select_combo_box_ex_a;
pub use dlg_dir_select_combo_box_ex_a::DlgDirSelectComboBoxExA;
mod dlg_dir_select_combo_box_ex_w;
pub use dlg_dir_select_combo_box_ex_w::DlgDirSelectComboBoxExW;
mod dlg_dir_select_ex_a;
pub use dlg_dir_select_ex_a::DlgDirSelectExA;
mod dlg_dir_select_ex_w;
pub use dlg_dir_select_ex_w::DlgDirSelectExW;
mod do_sound_connect;
pub use do_sound_connect::DoSoundConnect;
mod do_sound_disconnect;
pub use do_sound_disconnect::DoSoundDisconnect;
mod drag_detect;
pub use drag_detect::DragDetect;
mod drag_object;
pub use drag_object::DragObject;
mod draw_animated_rects;
pub use draw_animated_rects::DrawAnimatedRects;
mod draw_caption;
pub use draw_caption::DrawCaption;
mod draw_caption_temp_a;
pub use draw_caption_temp_a::DrawCaptionTempA;
mod draw_caption_temp_w;
pub use draw_caption_temp_w::DrawCaptionTempW;
mod draw_edge;
pub use draw_edge::DrawEdge;
mod draw_focus_rect;
pub use draw_focus_rect::DrawFocusRect;
mod draw_frame;
pub use draw_frame::DrawFrame;
mod draw_frame_control;
pub use draw_frame_control::DrawFrameControl;
mod draw_icon;
pub use draw_icon::DrawIcon;
mod draw_icon_ex;
pub use draw_icon_ex::DrawIconEx;
mod draw_menu_bar;
pub use draw_menu_bar::DrawMenuBar;
mod draw_menu_bar_temp;
pub use draw_menu_bar_temp::DrawMenuBarTemp;
mod draw_state_a;
pub use draw_state_a::DrawStateA;
mod draw_state_w;
pub use draw_state_w::DrawStateW;
mod draw_text_ex_a;
pub use draw_text_ex_a::DrawTextExA;
mod draw_text_ex_w;
pub use draw_text_ex_w::DrawTextExW;
mod dwm_get_dx_rgn;
pub use dwm_get_dx_rgn::DwmGetDxRgn;
mod dwm_get_dx_shared_surface;
pub use dwm_get_dx_shared_surface::DwmGetDxSharedSurface;
mod dwm_get_remote_session_occlusion_event;
pub use dwm_get_remote_session_occlusion_event::DwmGetRemoteSessionOcclusionEvent;
mod dwm_get_remote_session_occlusion_state;
pub use dwm_get_remote_session_occlusion_state::DwmGetRemoteSessionOcclusionState;
mod dwm_kernel_shutdown;
pub use dwm_kernel_shutdown::DwmKernelShutdown;
mod dwm_kernel_startup;
pub use dwm_kernel_startup::DwmKernelStartup;
mod dwm_lock_screen_updates;
pub use dwm_lock_screen_updates::DwmLockScreenUpdates;
mod dwm_validate_window;
pub use dwm_validate_window::DwmValidateWindow;
mod dwm_window_notifications_enabled;
pub use dwm_window_notifications_enabled::DwmWindowNotificationsEnabled;
mod edit_wnd_proc;
pub use edit_wnd_proc::EditWndProc;
mod empty_clipboard;
pub use empty_clipboard::EmptyClipboard;
mod enable_menu_item;
pub use enable_menu_item::EnableMenuItem;
mod enable_mouse_in_pointer;
pub use enable_mouse_in_pointer::EnableMouseInPointer;
mod enable_non_client_dpi_scaling;
pub use enable_non_client_dpi_scaling::EnableNonClientDpiScaling;
mod enable_one_core_transform_mode;
pub use enable_one_core_transform_mode::EnableOneCoreTransformMode;
mod enable_scroll_bar;
pub use enable_scroll_bar::EnableScrollBar;
mod enable_session_for_mmcss;
pub use enable_session_for_mmcss::EnableSessionForMMCSS;
mod enable_window_shell_window_management_behavior;
pub use enable_window_shell_window_management_behavior::EnableWindowShellWindowManagementBehavior;
mod end_defer_window_pos;
pub use end_defer_window_pos::EndDeferWindowPos;
mod end_defer_window_pos_ex;
pub use end_defer_window_pos_ex::EndDeferWindowPosEx;
mod end_dialog;
pub use end_dialog::EndDialog;
mod end_menu;
pub use end_menu::EndMenu;
mod end_task;
pub use end_task::EndTask;
mod enter_move_size_loop;
pub use enter_move_size_loop::EnterMoveSizeLoop;
mod enter_reader_mode_helper;
pub use enter_reader_mode_helper::EnterReaderModeHelper;
mod enum_child_windows;
pub use enum_child_windows::EnumChildWindows;
mod enum_clipboard_formats;
pub use enum_clipboard_formats::EnumClipboardFormats;
mod enum_desktop_windows;
pub use enum_desktop_windows::EnumDesktopWindows;
mod enum_desktops_a;
pub use enum_desktops_a::EnumDesktopsA;
mod enum_desktops_w;
pub use enum_desktops_w::EnumDesktopsW;
mod enum_display_devices_a;
pub use enum_display_devices_a::EnumDisplayDevicesA;
mod enum_display_devices_w;
pub use enum_display_devices_w::EnumDisplayDevicesW;
mod enum_display_monitors;
pub use enum_display_monitors::EnumDisplayMonitors;
mod enum_display_settings_a;
pub use enum_display_settings_a::EnumDisplaySettingsA;
mod enum_display_settings_ex_a;
pub use enum_display_settings_ex_a::EnumDisplaySettingsExA;
mod enum_display_settings_ex_w;
pub use enum_display_settings_ex_w::EnumDisplaySettingsExW;
mod enum_display_settings_w;
pub use enum_display_settings_w::EnumDisplaySettingsW;
mod enum_props_a;
pub use enum_props_a::EnumPropsA;
mod enum_props_ex_a;
pub use enum_props_ex_a::EnumPropsExA;
mod enum_props_ex_w;
pub use enum_props_ex_w::EnumPropsExW;
mod enum_props_w;
pub use enum_props_w::EnumPropsW;
mod enum_thread_windows;
pub use enum_thread_windows::EnumThreadWindows;
mod enum_window_stations_a;
pub use enum_window_stations_a::EnumWindowStationsA;
mod enum_window_stations_w;
pub use enum_window_stations_w::EnumWindowStationsW;
mod equal_rect;
pub use equal_rect::EqualRect;
mod evaluate_proximity_to_polygon;
pub use evaluate_proximity_to_polygon::EvaluateProximityToPolygon;
mod evaluate_proximity_to_rect;
pub use evaluate_proximity_to_rect::EvaluateProximityToRect;
mod exclude_update_rgn;
pub use exclude_update_rgn::ExcludeUpdateRgn;
mod exit_windows_ex;
pub use exit_windows_ex::ExitWindowsEx;
mod flash_window;
pub use flash_window::FlashWindow;
mod flash_window_ex;
pub use flash_window_ex::FlashWindowEx;
mod frame_rect;
pub use frame_rect::FrameRect;
mod free_dd_el_param;
pub use free_dd_el_param::FreeDDElParam;
mod frost_crashed_window;
pub use frost_crashed_window::FrostCrashedWindow;
mod get_active_window;
pub use get_active_window::GetActiveWindow;
mod get_alt_tab_info;
pub use get_alt_tab_info::GetAltTabInfo;
mod get_alt_tab_info_a;
pub use get_alt_tab_info_a::GetAltTabInfoA;
mod get_alt_tab_info_w;
pub use get_alt_tab_info_w::GetAltTabInfoW;
mod get_ancestor;
pub use get_ancestor::GetAncestor;
mod get_app_compat_flags;
pub use get_app_compat_flags::GetAppCompatFlags;
mod get_app_compat_flags2;
pub use get_app_compat_flags2::GetAppCompatFlags2;
mod get_auto_rotation_state;
pub use get_auto_rotation_state::GetAutoRotationState;
mod get_awareness_from_dpi_awareness_context;
pub use get_awareness_from_dpi_awareness_context::GetAwarenessFromDpiAwarenessContext;
mod get_cimssm;
pub use get_cimssm::GetCIMSSM;
mod get_capture;
pub use get_capture::GetCapture;
mod get_caret_blink_time;
pub use get_caret_blink_time::GetCaretBlinkTime;
mod get_caret_pos;
pub use get_caret_pos::GetCaretPos;
mod get_class_info_a;
pub use get_class_info_a::GetClassInfoA;
mod get_class_info_w;
pub use get_class_info_w::GetClassInfoW;
mod get_class_long_a;
pub use get_class_long_a::GetClassLongA;
mod get_class_long_ptr_a;
pub use get_class_long_ptr_a::GetClassLongPtrA;
mod get_class_long_ptr_w;
pub use get_class_long_ptr_w::GetClassLongPtrW;
mod get_class_long_w;
pub use get_class_long_w::GetClassLongW;
mod get_class_name_a;
pub use get_class_name_a::GetClassNameA;
mod get_class_name_w;
pub use get_class_name_w::GetClassNameW;
mod get_class_word;
pub use get_class_word::GetClassWord;
mod get_clip_cursor;
pub use get_clip_cursor::GetClipCursor;
mod get_clipboard_access_token;
pub use get_clipboard_access_token::GetClipboardAccessToken;
mod get_clipboard_data;
pub use get_clipboard_data::GetClipboardData;
mod get_clipboard_format_name_a;
pub use get_clipboard_format_name_a::GetClipboardFormatNameA;
mod get_clipboard_format_name_w;
pub use get_clipboard_format_name_w::GetClipboardFormatNameW;
mod get_clipboard_metadata;
pub use get_clipboard_metadata::GetClipboardMetadata;
mod get_clipboard_owner;
pub use get_clipboard_owner::GetClipboardOwner;
mod get_clipboard_sequence_number;
pub use get_clipboard_sequence_number::GetClipboardSequenceNumber;
mod get_clipboard_viewer;
pub use get_clipboard_viewer::GetClipboardViewer;
mod get_combo_box_info;
pub use get_combo_box_info::GetComboBoxInfo;
mod get_current_input_message_source;
pub use get_current_input_message_source::GetCurrentInputMessageSource;
mod get_current_monitor_topology_id;
pub use get_current_monitor_topology_id::GetCurrentMonitorTopologyId;
mod get_cursor;
pub use get_cursor::GetCursor;
mod get_cursor_frame_info;
pub use get_cursor_frame_info::GetCursorFrameInfo;
mod get_cursor_info;
pub use get_cursor_info::GetCursorInfo;
mod get_dc_ex;
pub use get_dc_ex::GetDCEx;
mod get_d_composition_hwnd_bitmap;
pub use get_d_composition_hwnd_bitmap::GetDCompositionHwndBitmap;
mod get_desktop_id;
pub use get_desktop_id::GetDesktopID;
mod get_dialog_base_units;
pub use get_dialog_base_units::GetDialogBaseUnits;
mod get_dialog_control_dpi_change_behavior;
pub use get_dialog_control_dpi_change_behavior::GetDialogControlDpiChangeBehavior;
mod get_dialog_dpi_change_behavior;
pub use get_dialog_dpi_change_behavior::GetDialogDpiChangeBehavior;
mod get_display_auto_rotation_preferences;
pub use get_display_auto_rotation_preferences::GetDisplayAutoRotationPreferences;
mod get_display_config_buffer_sizes;
pub use get_display_config_buffer_sizes::GetDisplayConfigBufferSizes;
mod get_dlg_ctrl_id;
pub use get_dlg_ctrl_id::GetDlgCtrlID;
mod get_dlg_item;
pub use get_dlg_item::GetDlgItem;
mod get_dlg_item_int;
pub use get_dlg_item_int::GetDlgItemInt;
mod get_dlg_item_text_a;
pub use get_dlg_item_text_a::GetDlgItemTextA;
mod get_dlg_item_text_w;
pub use get_dlg_item_text_w::GetDlgItemTextW;
mod get_double_click_time;
pub use get_double_click_time::GetDoubleClickTime;
mod get_dpi_awareness_context_for_process;
pub use get_dpi_awareness_context_for_process::GetDpiAwarenessContextForProcess;
mod get_dpi_for_monitor_internal;
pub use get_dpi_for_monitor_internal::GetDpiForMonitorInternal;
mod get_dpi_for_system;
pub use get_dpi_for_system::GetDpiForSystem;
mod get_dpi_for_window;
pub use get_dpi_for_window::GetDpiForWindow;
mod get_dpi_from_dpi_awareness_context;
pub use get_dpi_from_dpi_awareness_context::GetDpiFromDpiAwarenessContext;
mod get_extended_pointer_device_property;
pub use get_extended_pointer_device_property::GetExtendedPointerDeviceProperty;
mod get_gui_thread_info;
pub use get_gui_thread_info::GetGUIThreadInfo;
mod get_gesture_config;
pub use get_gesture_config::GetGestureConfig;
mod get_gesture_extra_args;
pub use get_gesture_extra_args::GetGestureExtraArgs;
mod get_gesture_info;
pub use get_gesture_info::GetGestureInfo;
mod get_gui_resources;
pub use get_gui_resources::GetGuiResources;
mod get_icon_info;
pub use get_icon_info::GetIconInfo;
mod get_icon_info_ex_a;
pub use get_icon_info_ex_a::GetIconInfoExA;
mod get_icon_info_ex_w;
pub use get_icon_info_ex_w::GetIconInfoExW;
mod get_input_desktop;
pub use get_input_desktop::GetInputDesktop;
mod get_input_locale_info;
pub use get_input_locale_info::GetInputLocaleInfo;
mod get_input_state;
pub use get_input_state::GetInputState;
mod get_internal_window_pos;
pub use get_internal_window_pos::GetInternalWindowPos;
mod get_kb_code_page;
pub use get_kb_code_page::GetKBCodePage;
mod get_key_name_text_a;
pub use get_key_name_text_a::GetKeyNameTextA;
mod get_key_name_text_w;
pub use get_key_name_text_w::GetKeyNameTextW;
mod get_keyboard_layout_list;
pub use get_keyboard_layout_list::GetKeyboardLayoutList;
mod get_keyboard_layout_name_a;
pub use get_keyboard_layout_name_a::GetKeyboardLayoutNameA;
mod get_keyboard_layout_name_w;
pub use get_keyboard_layout_name_w::GetKeyboardLayoutNameW;
mod get_keyboard_state;
pub use get_keyboard_state::GetKeyboardState;
mod get_last_active_popup;
pub use get_last_active_popup::GetLastActivePopup;
mod get_last_input_info;
pub use get_last_input_info::GetLastInputInfo;
mod get_layered_window_attributes;
pub use get_layered_window_attributes::GetLayeredWindowAttributes;
mod get_list_box_info;
pub use get_list_box_info::GetListBoxInfo;
mod get_magnification_desktop_color_effect;
pub use get_magnification_desktop_color_effect::GetMagnificationDesktopColorEffect;
mod get_magnification_desktop_magnification;
pub use get_magnification_desktop_magnification::GetMagnificationDesktopMagnification;
mod get_magnification_desktop_sampling_mode;
pub use get_magnification_desktop_sampling_mode::GetMagnificationDesktopSamplingMode;
mod get_magnification_lens_ctx_information;
pub use get_magnification_lens_ctx_information::GetMagnificationLensCtxInformation;
mod get_menu;
pub use get_menu::GetMenu;
mod get_menu_bar_info;
pub use get_menu_bar_info::GetMenuBarInfo;
mod get_menu_check_mark_dimensions;
pub use get_menu_check_mark_dimensions::GetMenuCheckMarkDimensions;
mod get_menu_context_help_id;
pub use get_menu_context_help_id::GetMenuContextHelpId;
mod get_menu_default_item;
pub use get_menu_default_item::GetMenuDefaultItem;
mod get_menu_info;
pub use get_menu_info::GetMenuInfo;
mod get_menu_item_count;
pub use get_menu_item_count::GetMenuItemCount;
mod get_menu_item_id;
pub use get_menu_item_id::GetMenuItemID;
mod get_menu_item_info_a;
pub use get_menu_item_info_a::GetMenuItemInfoA;
mod get_menu_item_info_w;
pub use get_menu_item_info_w::GetMenuItemInfoW;
mod get_menu_item_rect;
pub use get_menu_item_rect::GetMenuItemRect;
mod get_menu_state;
pub use get_menu_state::GetMenuState;
mod get_menu_string_a;
pub use get_menu_string_a::GetMenuStringA;
mod get_menu_string_w;
pub use get_menu_string_w::GetMenuStringW;
mod get_message_extra_info;
pub use get_message_extra_info::GetMessageExtraInfo;
mod get_message_pos;
pub use get_message_pos::GetMessagePos;
mod get_message_time;
pub use get_message_time::GetMessageTime;
mod get_monitor_info_a;
pub use get_monitor_info_a::GetMonitorInfoA;
mod get_monitor_info_w;
pub use get_monitor_info_w::GetMonitorInfoW;
mod get_mouse_move_points_ex;
pub use get_mouse_move_points_ex::GetMouseMovePointsEx;
mod get_next_dlg_group_item;
pub use get_next_dlg_group_item::GetNextDlgGroupItem;
mod get_next_dlg_tab_item;
pub use get_next_dlg_tab_item::GetNextDlgTabItem;
mod get_open_clipboard_window;
pub use get_open_clipboard_window::GetOpenClipboardWindow;
mod get_parent;
pub use get_parent::GetParent;
mod get_pointer_cursor_id;
pub use get_pointer_cursor_id::GetPointerCursorId;
mod get_pointer_device;
pub use get_pointer_device::GetPointerDevice;
mod get_pointer_device_cursors;
pub use get_pointer_device_cursors::GetPointerDeviceCursors;
mod get_pointer_device_input_space;
pub use get_pointer_device_input_space::GetPointerDeviceInputSpace;
mod get_pointer_device_orientation;
pub use get_pointer_device_orientation::GetPointerDeviceOrientation;
mod get_pointer_device_properties;
pub use get_pointer_device_properties::GetPointerDeviceProperties;
mod get_pointer_device_rects;
pub use get_pointer_device_rects::GetPointerDeviceRects;
mod get_pointer_devices;
pub use get_pointer_devices::GetPointerDevices;
mod get_pointer_frame_arrival_times;
pub use get_pointer_frame_arrival_times::GetPointerFrameArrivalTimes;
mod get_pointer_frame_info;
pub use get_pointer_frame_info::GetPointerFrameInfo;
mod get_pointer_frame_info_history;
pub use get_pointer_frame_info_history::GetPointerFrameInfoHistory;
mod get_pointer_frame_pen_info;
pub use get_pointer_frame_pen_info::GetPointerFramePenInfo;
mod get_pointer_frame_pen_info_history;
pub use get_pointer_frame_pen_info_history::GetPointerFramePenInfoHistory;
mod get_pointer_frame_times;
pub use get_pointer_frame_times::GetPointerFrameTimes;
mod get_pointer_frame_touch_info;
pub use get_pointer_frame_touch_info::GetPointerFrameTouchInfo;
mod get_pointer_frame_touch_info_history;
pub use get_pointer_frame_touch_info_history::GetPointerFrameTouchInfoHistory;
mod get_pointer_frame_touchpad_info;
pub use get_pointer_frame_touchpad_info::GetPointerFrameTouchpadInfo;
mod get_pointer_frame_touchpad_info_history;
pub use get_pointer_frame_touchpad_info_history::GetPointerFrameTouchpadInfoHistory;
mod get_pointer_info;
pub use get_pointer_info::GetPointerInfo;
mod get_pointer_info_history;
pub use get_pointer_info_history::GetPointerInfoHistory;
mod get_pointer_input_transform;
pub use get_pointer_input_transform::GetPointerInputTransform;
mod get_pointer_pen_info;
pub use get_pointer_pen_info::GetPointerPenInfo;
mod get_pointer_pen_info_history;
pub use get_pointer_pen_info_history::GetPointerPenInfoHistory;
mod get_pointer_touch_info;
pub use get_pointer_touch_info::GetPointerTouchInfo;
mod get_pointer_touch_info_history;
pub use get_pointer_touch_info_history::GetPointerTouchInfoHistory;
mod get_pointer_touchpad_info;
pub use get_pointer_touchpad_info::GetPointerTouchpadInfo;
mod get_pointer_touchpad_info_history;
pub use get_pointer_touchpad_info_history::GetPointerTouchpadInfoHistory;
mod get_pointer_type;
pub use get_pointer_type::GetPointerType;
mod get_priority_clipboard_format;
pub use get_priority_clipboard_format::GetPriorityClipboardFormat;
mod get_process_default_layout;
pub use get_process_default_layout::GetProcessDefaultLayout;
mod get_process_dpi_awareness_internal;
pub use get_process_dpi_awareness_internal::GetProcessDpiAwarenessInternal;
mod get_process_ui_context_information;
pub use get_process_ui_context_information::GetProcessUIContextInformation;
mod get_progman_window;
pub use get_progman_window::GetProgmanWindow;
mod get_prop_a;
pub use get_prop_a::GetPropA;
mod get_prop_w;
pub use get_prop_w::GetPropW;
mod get_queue_status;
pub use get_queue_status::GetQueueStatus;
mod get_raw_input_buffer;
pub use get_raw_input_buffer::GetRawInputBuffer;
mod get_raw_input_data;
pub use get_raw_input_data::GetRawInputData;
mod get_raw_input_device_info_a;
pub use get_raw_input_device_info_a::GetRawInputDeviceInfoA;
mod get_raw_input_device_info_w;
pub use get_raw_input_device_info_w::GetRawInputDeviceInfoW;
mod get_raw_input_device_list;
pub use get_raw_input_device_list::GetRawInputDeviceList;
mod get_raw_pointer_device_data;
pub use get_raw_pointer_device_data::GetRawPointerDeviceData;
mod get_reason_title_from_reason_code;
pub use get_reason_title_from_reason_code::GetReasonTitleFromReasonCode;
mod get_registered_raw_input_devices;
pub use get_registered_raw_input_devices::GetRegisteredRawInputDevices;
mod get_scroll_bar_info;
pub use get_scroll_bar_info::GetScrollBarInfo;
mod get_scroll_info;
pub use get_scroll_info::GetScrollInfo;
mod get_scroll_pos;
pub use get_scroll_pos::GetScrollPos;
mod get_scroll_range;
pub use get_scroll_range::GetScrollRange;
mod get_send_message_receiver;
pub use get_send_message_receiver::GetSendMessageReceiver;
mod get_shell_change_notify_window;
pub use get_shell_change_notify_window::GetShellChangeNotifyWindow;
mod get_shell_window;
pub use get_shell_window::GetShellWindow;
mod get_sub_menu;
pub use get_sub_menu::GetSubMenu;
mod get_sys_color;
pub use get_sys_color::GetSysColor;
mod get_sys_color_brush;
pub use get_sys_color_brush::GetSysColorBrush;
mod get_system_dpi_for_process;
pub use get_system_dpi_for_process::GetSystemDpiForProcess;
mod get_system_menu;
pub use get_system_menu::GetSystemMenu;
mod get_system_metrics_for_dpi;
pub use get_system_metrics_for_dpi::GetSystemMetricsForDpi;
mod get_tabbed_text_extent_a;
pub use get_tabbed_text_extent_a::GetTabbedTextExtentA;
mod get_tabbed_text_extent_w;
pub use get_tabbed_text_extent_w::GetTabbedTextExtentW;
mod get_taskman_window;
pub use get_taskman_window::GetTaskmanWindow;
mod get_thread_desktop;
pub use get_thread_desktop::GetThreadDesktop;
mod get_thread_dpi_awareness_context;
pub use get_thread_dpi_awareness_context::GetThreadDpiAwarenessContext;
mod get_thread_dpi_hosting_behavior;
pub use get_thread_dpi_hosting_behavior::GetThreadDpiHostingBehavior;
mod get_title_bar_info;
pub use get_title_bar_info::GetTitleBarInfo;
mod get_top_level_window;
pub use get_top_level_window::GetTopLevelWindow;
mod get_top_window;
pub use get_top_window::GetTopWindow;
mod get_touch_input_info;
pub use get_touch_input_info::GetTouchInputInfo;
mod get_unpredicted_message_pos;
pub use get_unpredicted_message_pos::GetUnpredictedMessagePos;
mod get_update_rect;
pub use get_update_rect::GetUpdateRect;
mod get_update_rgn;
pub use get_update_rgn::GetUpdateRgn;
mod get_updated_clipboard_formats;
pub use get_updated_clipboard_formats::GetUpdatedClipboardFormats;
mod get_user_object_information_a;
pub use get_user_object_information_a::GetUserObjectInformationA;
mod get_user_object_security;
pub use get_user_object_security::GetUserObjectSecurity;
mod get_win_station_info;
pub use get_win_station_info::GetWinStationInfo;
mod get_window;
pub use get_window::GetWindow;
mod get_window_band;
pub use get_window_band::GetWindowBand;
mod get_window_composition_attribute;
pub use get_window_composition_attribute::GetWindowCompositionAttribute;
mod get_window_composition_info;
pub use get_window_composition_info::GetWindowCompositionInfo;
mod get_window_context_help_id;
pub use get_window_context_help_id::GetWindowContextHelpId;
mod get_window_dc;
pub use get_window_dc::GetWindowDC;
mod get_window_display_affinity;
pub use get_window_display_affinity::GetWindowDisplayAffinity;
mod get_window_dpi_awareness_context;
pub use get_window_dpi_awareness_context::GetWindowDpiAwarenessContext;
mod get_window_dpi_hosting_behavior;
pub use get_window_dpi_hosting_behavior::GetWindowDpiHostingBehavior;
mod get_window_feedback_setting;
pub use get_window_feedback_setting::GetWindowFeedbackSetting;
mod get_window_info;
pub use get_window_info::GetWindowInfo;
mod get_window_minimize_rect;
pub use get_window_minimize_rect::GetWindowMinimizeRect;
mod get_window_module_file_name;
pub use get_window_module_file_name::GetWindowModuleFileName;
mod get_window_module_file_name_a;
pub use get_window_module_file_name_a::GetWindowModuleFileNameA;
mod get_window_module_file_name_w;
pub use get_window_module_file_name_w::GetWindowModuleFileNameW;
mod get_window_placement;
pub use get_window_placement::GetWindowPlacement;
mod get_window_process_handle;
pub use get_window_process_handle::GetWindowProcessHandle;
mod get_window_rgn;
pub use get_window_rgn::GetWindowRgn;
mod get_window_rgn_box;
pub use get_window_rgn_box::GetWindowRgnBox;
mod get_window_rgn_ex;
pub use get_window_rgn_ex::GetWindowRgnEx;
mod get_window_text_length_a;
pub use get_window_text_length_a::GetWindowTextLengthA;
mod get_window_text_length_w;
pub use get_window_text_length_w::GetWindowTextLengthW;
mod get_window_thread_process_id;
pub use get_window_thread_process_id::GetWindowThreadProcessId;
mod get_window_word;
pub use get_window_word::GetWindowWord;
mod ghost_window_from_hung_window;
pub use ghost_window_from_hung_window::GhostWindowFromHungWindow;
mod gray_string_a;
pub use gray_string_a::GrayStringA;
mod gray_string_w;
pub use gray_string_w::GrayStringW;
mod handle_delegated_input;
pub use handle_delegated_input::HandleDelegatedInput;
mod hide_caret;
pub use hide_caret::HideCaret;
mod hilite_menu_item;
pub use hilite_menu_item::HiliteMenuItem;
mod hung_window_from_ghost_window;
pub use hung_window_from_ghost_window::HungWindowFromGhostWindow;
mod imp_get_imea;
pub use imp_get_imea::IMPGetIMEA;
mod imp_get_imew;
pub use imp_get_imew::IMPGetIMEW;
mod imp_query_imea;
pub use imp_query_imea::IMPQueryIMEA;
mod imp_query_imew;
pub use imp_query_imew::IMPQueryIMEW;
mod imp_set_imea;
pub use imp_set_imea::IMPSetIMEA;
mod imp_set_imew;
pub use imp_set_imew::IMPSetIMEW;
mod impersonate_dde_client_window;
pub use impersonate_dde_client_window::ImpersonateDdeClientWindow;
mod in_send_message;
pub use in_send_message::InSendMessage;
mod in_send_message_ex;
pub use in_send_message_ex::InSendMessageEx;
mod inflate_rect;
pub use inflate_rect::InflateRect;
mod inherit_window_monitor;
pub use inherit_window_monitor::InheritWindowMonitor;
mod init_d_manip_hook;
pub use init_d_manip_hook::InitDManipHook;
mod initialize_generic_hid_injection;
pub use initialize_generic_hid_injection::InitializeGenericHidInjection;
mod initialize_input_device_injection;
pub use initialize_input_device_injection::InitializeInputDeviceInjection;
mod initialize_lpk_hooks;
pub use initialize_lpk_hooks::InitializeLpkHooks;
mod initialize_pointer_device_injection;
pub use initialize_pointer_device_injection::InitializePointerDeviceInjection;
mod initialize_pointer_device_injection_ex;
pub use initialize_pointer_device_injection_ex::InitializePointerDeviceInjectionEx;
mod initialize_touch_injection;
pub use initialize_touch_injection::InitializeTouchInjection;
mod inject_device_input;
pub use inject_device_input::InjectDeviceInput;
mod inject_generic_hid_input;
pub use inject_generic_hid_input::InjectGenericHidInput;
mod inject_keyboard_input;
pub use inject_keyboard_input::InjectKeyboardInput;
mod inject_mouse_input;
pub use inject_mouse_input::InjectMouseInput;
mod inject_pointer_input;
pub use inject_pointer_input::InjectPointerInput;
mod inject_synthetic_pointer_input;
pub use inject_synthetic_pointer_input::InjectSyntheticPointerInput;
mod inject_touch_input;
pub use inject_touch_input::InjectTouchInput;
mod input_space_region_from_point;
pub use input_space_region_from_point::InputSpaceRegionFromPoint;
mod insert_menu_a;
pub use insert_menu_a::InsertMenuA;
mod insert_menu_item_a;
pub use insert_menu_item_a::InsertMenuItemA;
mod insert_menu_item_w;
pub use insert_menu_item_w::InsertMenuItemW;
mod insert_menu_w;
pub use insert_menu_w::InsertMenuW;
mod internal_get_window_icon;
pub use internal_get_window_icon::InternalGetWindowIcon;
mod internal_get_window_text;
pub use internal_get_window_text::InternalGetWindowText;
mod intersect_rect;
pub use intersect_rect::IntersectRect;
mod invalidate_rgn;
pub use invalidate_rgn::InvalidateRgn;
mod invert_rect;
pub use invert_rect::InvertRect;
mod is_char_alpha_a;
pub use is_char_alpha_a::IsCharAlphaA;
mod is_char_alpha_numeric_a;
pub use is_char_alpha_numeric_a::IsCharAlphaNumericA;
mod is_char_alpha_numeric_w;
pub use is_char_alpha_numeric_w::IsCharAlphaNumericW;
mod is_char_alpha_w;
pub use is_char_alpha_w::IsCharAlphaW;
mod is_char_lower_a;
pub use is_char_lower_a::IsCharLowerA;
mod is_char_lower_w;
pub use is_char_lower_w::IsCharLowerW;
mod is_char_upper_a;
pub use is_char_upper_a::IsCharUpperA;
mod is_char_upper_w;
pub use is_char_upper_w::IsCharUpperW;
mod is_child;
pub use is_child::IsChild;
mod is_clipboard_format_available;
pub use is_clipboard_format_available::IsClipboardFormatAvailable;
mod is_dialog_message;
pub use is_dialog_message::IsDialogMessage;
mod is_dialog_message_a;
pub use is_dialog_message_a::IsDialogMessageA;
mod is_dialog_message_w;
pub use is_dialog_message_w::IsDialogMessageW;
mod is_dlg_button_checked;
pub use is_dlg_button_checked::IsDlgButtonChecked;
mod is_gui_thread;
pub use is_gui_thread::IsGUIThread;
mod is_hung_app_window;
pub use is_hung_app_window::IsHungAppWindow;
mod is_iconic;
pub use is_iconic::IsIconic;
mod is_immersive_process;
pub use is_immersive_process::IsImmersiveProcess;
mod is_in_desktop_window_band;
pub use is_in_desktop_window_band::IsInDesktopWindowBand;
mod is_intercept_window;
pub use is_intercept_window::IsInterceptWindow;
mod is_menu;
pub use is_menu::IsMenu;
mod is_mouse_in_pointer_enabled;
pub use is_mouse_in_pointer_enabled::IsMouseInPointerEnabled;
mod is_one_core_transform_mode;
pub use is_one_core_transform_mode::IsOneCoreTransformMode;
mod is_process_dpi_aware;
pub use is_process_dpi_aware::IsProcessDPIAware;
mod is_rect_empty;
pub use is_rect_empty::IsRectEmpty;
mod is_set_enabled;
pub use is_set_enabled::IsSETEnabled;
mod is_server_side_window;
pub use is_server_side_window::IsServerSideWindow;
mod is_thread_desktop_composited;
pub use is_thread_desktop_composited::IsThreadDesktopComposited;
mod is_thread_message_queue_attached;
pub use is_thread_message_queue_attached::IsThreadMessageQueueAttached;
mod is_thread_tsf_event_aware;
pub use is_thread_tsf_event_aware::IsThreadTSFEventAware;
mod is_top_level_window;
pub use is_top_level_window::IsTopLevelWindow;
mod is_touch_window;
pub use is_touch_window::IsTouchWindow;
mod is_valid_dpi_awareness_context;
pub use is_valid_dpi_awareness_context::IsValidDpiAwarenessContext;
mod is_win_event_hook_installed;
pub use is_win_event_hook_installed::IsWinEventHookInstalled;
mod is_window_arranged;
pub use is_window_arranged::IsWindowArranged;
mod is_window_display_change_suppressed;
pub use is_window_display_change_suppressed::IsWindowDisplayChangeSuppressed;
mod is_window_enabled;
pub use is_window_enabled::IsWindowEnabled;
mod is_window_in_destroy;
pub use is_window_in_destroy::IsWindowInDestroy;
mod is_window_redirected_for_print;
pub use is_window_redirected_for_print::IsWindowRedirectedForPrint;
mod is_window_unicode;
pub use is_window_unicode::IsWindowUnicode;
mod is_wow64_message;
pub use is_wow64_message::IsWow64Message;
mod is_zoomed;
pub use is_zoomed::IsZoomed;
mod kill_timer;
pub use kill_timer::KillTimer;
mod load_accelerators_a;
pub use load_accelerators_a::LoadAcceleratorsA;
mod load_accelerators_w;
pub use load_accelerators_w::LoadAcceleratorsW;
mod load_bitmap_a;
pub use load_bitmap_a::LoadBitmapA;
mod load_bitmap_w;
pub use load_bitmap_w::LoadBitmapW;
mod load_cursor_from_file_a;
pub use load_cursor_from_file_a::LoadCursorFromFileA;
mod load_cursor_from_file_w;
pub use load_cursor_from_file_w::LoadCursorFromFileW;
mod load_cursor_w;
pub use load_cursor_w::LoadCursorW;
mod load_icon_w;
pub use load_icon_w::LoadIconW;
mod load_image_a;
pub use load_image_a::LoadImageA;
mod load_image_w;
pub use load_image_w::LoadImageW;
mod load_keyboard_layout_a;
pub use load_keyboard_layout_a::LoadKeyboardLayoutA;
mod load_keyboard_layout_ex;
pub use load_keyboard_layout_ex::LoadKeyboardLayoutEx;
mod load_keyboard_layout_w;
pub use load_keyboard_layout_w::LoadKeyboardLayoutW;
mod load_local_fonts;
pub use load_local_fonts::LoadLocalFonts;
mod load_menu_a;
pub use load_menu_a::LoadMenuA;
mod load_menu_indirect_a;
pub use load_menu_indirect_a::LoadMenuIndirectA;
mod load_menu_indirect_w;
pub use load_menu_indirect_w::LoadMenuIndirectW;
mod load_menu_w;
pub use load_menu_w::LoadMenuW;
mod load_remote_fonts;
pub use load_remote_fonts::LoadRemoteFonts;
mod lock_set_foreground_window;
pub use lock_set_foreground_window::LockSetForegroundWindow;
mod lock_window_station;
pub use lock_window_station::LockWindowStation;
mod lock_window_update;
pub use lock_window_update::LockWindowUpdate;
mod lock_work_station;
pub use lock_work_station::LockWorkStation;
mod logical_to_physical_point;
pub use logical_to_physical_point::LogicalToPhysicalPoint;
mod logical_to_physical_point_for_per_monitor_dpi;
pub use logical_to_physical_point_for_per_monitor_dpi::LogicalToPhysicalPointForPerMonitorDPI;
mod lookup_icon_id_from_directory;
pub use lookup_icon_id_from_directory::LookupIconIdFromDirectory;
mod lookup_icon_id_from_directory_ex;
pub use lookup_icon_id_from_directory_ex::LookupIconIdFromDirectoryEx;
mod mb_to_wcs_ex;
pub use mb_to_wcs_ex::MBToWCSEx;
mod mb_to_wcs_ext;
pub use mb_to_wcs_ext::MBToWCSExt;
mod mb_get_string;
pub use mb_get_string::MB_GetString;
mod mit_get_cursor_update_handle;
pub use mit_get_cursor_update_handle::MITGetCursorUpdateHandle;
mod mit_set_last_input_recipient;
pub use mit_set_last_input_recipient::MITSetLastInputRecipient;
mod mit_synthesize_touch_input;
pub use mit_synthesize_touch_input::MITSynthesizeTouchInput;
mod make_thread_tsf_event_aware;
pub use make_thread_tsf_event_aware::MakeThreadTSFEventAware;
mod map_dialog_rect;
pub use map_dialog_rect::MapDialogRect;
mod map_points_by_visual_identifier;
pub use map_points_by_visual_identifier::MapPointsByVisualIdentifier;
mod map_virtual_key_ex_a;
pub use map_virtual_key_ex_a::MapVirtualKeyExA;
mod map_visual_relative_points;
pub use map_visual_relative_points::MapVisualRelativePoints;
mod map_window_points;
pub use map_window_points::MapWindowPoints;
mod menu_item_from_point;
pub use menu_item_from_point::MenuItemFromPoint;
mod menu_window_proc_a;
pub use menu_window_proc_a::MenuWindowProcA;
mod menu_window_proc_w;
pub use menu_window_proc_w::MenuWindowProcW;
mod message_beep;
pub use message_beep::MessageBeep;
mod message_box_ex_a;
pub use message_box_ex_a::MessageBoxExA;
mod message_box_ex_w;
pub use message_box_ex_w::MessageBoxExW;
mod message_box_indirect_a;
pub use message_box_indirect_a::MessageBoxIndirectA;
mod message_box_indirect_w;
pub use message_box_indirect_w::MessageBoxIndirectW;
mod message_box_timeout_a;
pub use message_box_timeout_a::MessageBoxTimeoutA;
mod message_box_timeout_w;
pub use message_box_timeout_w::MessageBoxTimeoutW;
mod modify_menu_a;
pub use modify_menu_a::ModifyMenuA;
mod modify_menu_w;
pub use modify_menu_w::ModifyMenuW;
mod monitor_from_point;
pub use monitor_from_point::MonitorFromPoint;
mod monitor_from_rect;
pub use monitor_from_rect::MonitorFromRect;
mod monitor_from_window;
pub use monitor_from_window::MonitorFromWindow;
mod move_window;
pub use move_window::MoveWindow;
mod msg_wait_for_multiple_objects;
pub use msg_wait_for_multiple_objects::MsgWaitForMultipleObjects;
mod msg_wait_for_multiple_objects_ex;
pub use msg_wait_for_multiple_objects_ex::MsgWaitForMultipleObjectsEx;
mod notify_overlay_window;
pub use notify_overlay_window::NotifyOverlayWindow;
mod notify_win_event;
pub use notify_win_event::NotifyWinEvent;
mod oem_key_scan;
pub use oem_key_scan::OemKeyScan;
mod oem_to_char_a;
pub use oem_to_char_a::OemToCharA;
mod oem_to_char_buff_a;
pub use oem_to_char_buff_a::OemToCharBuffA;
mod oem_to_char_buff_w;
pub use oem_to_char_buff_w::OemToCharBuffW;
mod oem_to_char_w;
pub use oem_to_char_w::OemToCharW;
mod offset_rect;
pub use offset_rect::OffsetRect;
mod open_clipboard;
pub use open_clipboard::OpenClipboard;
mod open_desktop_a;
pub use open_desktop_a::OpenDesktopA;
mod open_desktop_w;
pub use open_desktop_w::OpenDesktopW;
mod open_icon;
pub use open_icon::OpenIcon;
mod open_input_desktop;
pub use open_input_desktop::OpenInputDesktop;
mod open_thread_desktop;
pub use open_thread_desktop::OpenThreadDesktop;
mod open_window_station_a;
pub use open_window_station_a::OpenWindowStationA;
mod open_window_station_w;
pub use open_window_station_w::OpenWindowStationW;
mod pack_dd_el_param;
pub use pack_dd_el_param::PackDDElParam;
mod pack_touch_hit_testing_proximity_evaluation;
pub use pack_touch_hit_testing_proximity_evaluation::PackTouchHitTestingProximityEvaluation;
mod paint_desktop;
pub use paint_desktop::PaintDesktop;
mod paint_menu_bar;
pub use paint_menu_bar::PaintMenuBar;
mod paint_monitor;
pub use paint_monitor::PaintMonitor;
mod physical_to_logical_point;
pub use physical_to_logical_point::PhysicalToLogicalPoint;
mod physical_to_logical_point_for_per_monitor_dpi;
pub use physical_to_logical_point_for_per_monitor_dpi::PhysicalToLogicalPointForPerMonitorDPI;
mod post_thread_message_a;
pub use post_thread_message_a::PostThreadMessageA;
mod post_thread_message_w;
pub use post_thread_message_w::PostThreadMessageW;
mod print_window;
pub use print_window::PrintWindow;
mod private_extract_icon_ex_a;
pub use private_extract_icon_ex_a::PrivateExtractIconExA;
mod private_extract_icon_ex_w;
pub use private_extract_icon_ex_w::PrivateExtractIconExW;
mod private_extract_icons_a;
pub use private_extract_icons_a::PrivateExtractIconsA;
mod private_extract_icons_w;
pub use private_extract_icons_w::PrivateExtractIconsW;
mod private_register_ics_proc;
pub use private_register_ics_proc::PrivateRegisterICSProc;
mod pt_in_rect;
pub use pt_in_rect::PtInRect;
mod query_bsdr_window;
pub use query_bsdr_window::QueryBSDRWindow;
mod query_display_config;
pub use query_display_config::QueryDisplayConfig;
mod query_send_message;
pub use query_send_message::QuerySendMessage;
mod rim_add_input_observer;
pub use rim_add_input_observer::RIMAddInputObserver;
mod rim_are_sibling_devices;
pub use rim_are_sibling_devices::RIMAreSiblingDevices;
mod rim_device_io_control;
pub use rim_device_io_control::RIMDeviceIoControl;
mod rim_enable_monitor_mapping_for_device;
pub use rim_enable_monitor_mapping_for_device::RIMEnableMonitorMappingForDevice;
mod rim_free_input_buffer;
pub use rim_free_input_buffer::RIMFreeInputBuffer;
mod rim_get_device_preparsed_data;
pub use rim_get_device_preparsed_data::RIMGetDevicePreparsedData;
mod rim_get_device_preparsed_data_lockfree;
pub use rim_get_device_preparsed_data_lockfree::RIMGetDevicePreparsedDataLockfree;
mod rim_get_device_properties;
pub use rim_get_device_properties::RIMGetDeviceProperties;
mod rim_get_device_properties_lockfree;
pub use rim_get_device_properties_lockfree::RIMGetDevicePropertiesLockfree;
mod rim_get_physical_device_rect;
pub use rim_get_physical_device_rect::RIMGetPhysicalDeviceRect;
mod rim_get_source_process_id;
pub use rim_get_source_process_id::RIMGetSourceProcessId;
mod rim_observe_next_input;
pub use rim_observe_next_input::RIMObserveNextInput;
mod rim_on_async_pnp_work_notification;
pub use rim_on_async_pnp_work_notification::RIMOnAsyncPnpWorkNotification;
mod rim_on_pnp_notification;
pub use rim_on_pnp_notification::RIMOnPnpNotification;
mod rim_on_timer_notification;
pub use rim_on_timer_notification::RIMOnTimerNotification;
mod rim_query_device_path;
pub use rim_query_device_path::RIMQueryDevicePath;
mod rim_read_input;
pub use rim_read_input::RIMReadInput;
mod rim_register_for_input;
pub use rim_register_for_input::RIMRegisterForInput;
mod rim_register_for_input_ex;
pub use rim_register_for_input_ex::RIMRegisterForInputEx;
mod rim_remove_input_observer;
pub use rim_remove_input_observer::RIMRemoveInputObserver;
mod rim_set_extended_device_property;
pub use rim_set_extended_device_property::RIMSetExtendedDeviceProperty;
mod rim_set_test_mode_status;
pub use rim_set_test_mode_status::RIMSetTestModeStatus;
mod rim_unregister_for_input;
pub use rim_unregister_for_input::RIMUnregisterForInput;
mod rim_update_input_observer_registration;
pub use rim_update_input_observer_registration::RIMUpdateInputObserverRegistration;
mod raise_lower_shell_window;
pub use raise_lower_shell_window::RaiseLowerShellWindow;
mod real_child_window_from_point;
pub use real_child_window_from_point::RealChildWindowFromPoint;
mod real_get_window_class;
pub use real_get_window_class::RealGetWindowClass;
mod real_get_window_class_a;
pub use real_get_window_class_a::RealGetWindowClassA;
mod real_get_window_class_w;
pub use real_get_window_class_w::RealGetWindowClassW;
mod reason_code_needs_bug_id;
pub use reason_code_needs_bug_id::ReasonCodeNeedsBugID;
mod reason_code_needs_comment;
pub use reason_code_needs_comment::ReasonCodeNeedsComment;
mod record_shutdown_reason;
pub use record_shutdown_reason::RecordShutdownReason;
mod redraw_window;
pub use redraw_window::RedrawWindow;
mod register_bsdr_window;
pub use register_bsdr_window::RegisterBSDRWindow;
mod register_clipboard_format_a;
pub use register_clipboard_format_a::RegisterClipboardFormatA;
mod register_clipboard_format_w;
pub use register_clipboard_format_w::RegisterClipboardFormatW;
mod register_cloaked_notification;
pub use register_cloaked_notification::RegisterCloakedNotification;
mod register_d_manip_hook;
pub use register_d_manip_hook::RegisterDManipHook;
mod register_device_notification_a;
pub use register_device_notification_a::RegisterDeviceNotificationA;
mod register_device_notification_w;
pub use register_device_notification_w::RegisterDeviceNotificationW;
mod register_error_reporting_dialog;
pub use register_error_reporting_dialog::RegisterErrorReportingDialog;
mod register_for_custom_dock_targets;
pub use register_for_custom_dock_targets::RegisterForCustomDockTargets;
mod register_for_tooltip_dismiss_notification;
pub use register_for_tooltip_dismiss_notification::RegisterForTooltipDismissNotification;
mod register_frost_window;
pub use register_frost_window::RegisterFrostWindow;
mod register_ghost_window;
pub use register_ghost_window::RegisterGhostWindow;
mod register_hot_key;
pub use register_hot_key::RegisterHotKey;
mod register_logon_process;
pub use register_logon_process::RegisterLogonProcess;
mod register_message_pump_hook;
pub use register_message_pump_hook::RegisterMessagePumpHook;
mod register_pointer_device_notifications;
pub use register_pointer_device_notifications::RegisterPointerDeviceNotifications;
mod register_pointer_input_target;
pub use register_pointer_input_target::RegisterPointerInputTarget;
mod register_pointer_input_target_ex;
pub use register_pointer_input_target_ex::RegisterPointerInputTargetEx;
mod register_power_setting_notification;
pub use register_power_setting_notification::RegisterPowerSettingNotification;
mod register_raw_input_devices;
pub use register_raw_input_devices::RegisterRawInputDevices;
mod register_services_process;
pub use register_services_process::RegisterServicesProcess;
mod register_session_port;
pub use register_session_port::RegisterSessionPort;
mod register_shell_hook_window;
pub use register_shell_hook_window::RegisterShellHookWindow;
mod register_suspend_resume_notification;
pub use register_suspend_resume_notification::RegisterSuspendResumeNotification;
mod register_system_thread;
pub use register_system_thread::RegisterSystemThread;
mod register_tasklist;
pub use register_tasklist::RegisterTasklist;
mod register_touch_hit_testing_window;
pub use register_touch_hit_testing_window::RegisterTouchHitTestingWindow;
mod register_touch_window;
pub use register_touch_window::RegisterTouchWindow;
mod register_touchpad_capable_thread;
pub use register_touchpad_capable_thread::RegisterTouchpadCapableThread;
mod register_touchpad_capable_window;
pub use register_touchpad_capable_window::RegisterTouchpadCapableWindow;
mod register_user_api_hook;
pub use register_user_api_hook::RegisterUserApiHook;
mod register_window_message_a;
pub use register_window_message_a::RegisterWindowMessageA;
mod register_window_message_w;
pub use register_window_message_w::RegisterWindowMessageW;
mod release_capture;
pub use release_capture::ReleaseCapture;
mod release_dwm_hit_test_waiters;
pub use release_dwm_hit_test_waiters::ReleaseDwmHitTestWaiters;
mod remove_clipboard_format_listener;
pub use remove_clipboard_format_listener::RemoveClipboardFormatListener;
mod remove_injection_device;
pub use remove_injection_device::RemoveInjectionDevice;
mod remove_menu;
pub use remove_menu::RemoveMenu;
mod remove_prop_a;
pub use remove_prop_a::RemovePropA;
mod remove_prop_w;
pub use remove_prop_w::RemovePropW;
mod remove_thread_tsf_event_awareness;
pub use remove_thread_tsf_event_awareness::RemoveThreadTSFEventAwareness;
mod remove_visual_identifier;
pub use remove_visual_identifier::RemoveVisualIdentifier;
mod reply_message;
pub use reply_message::ReplyMessage;
mod report_inertia;
pub use report_inertia::ReportInertia;
mod resolve_desktop_for_wow;
pub use resolve_desktop_for_wow::ResolveDesktopForWOW;
mod reuse_dd_el_param;
pub use reuse_dd_el_param::ReuseDDElParam;
mod screen_to_client;
pub use screen_to_client::ScreenToClient;
mod scroll_children;
pub use scroll_children::ScrollChildren;
mod scroll_dc;
pub use scroll_dc::ScrollDC;
mod scroll_window;
pub use scroll_window::ScrollWindow;
mod scroll_window_ex;
pub use scroll_window_ex::ScrollWindowEx;
mod send_dlg_item_message_a;
pub use send_dlg_item_message_a::SendDlgItemMessageA;
mod send_dlg_item_message_w;
pub use send_dlg_item_message_w::SendDlgItemMessageW;
mod send_ime_message_ex_a;
pub use send_ime_message_ex_a::SendIMEMessageExA;
mod send_ime_message_ex_w;
pub use send_ime_message_ex_w::SendIMEMessageExW;
mod send_input;
pub use send_input::SendInput;
mod send_message_callback_a;
pub use send_message_callback_a::SendMessageCallbackA;
mod send_message_callback_w;
pub use send_message_callback_w::SendMessageCallbackW;
mod send_message_timeout_a;
pub use send_message_timeout_a::SendMessageTimeoutA;
mod send_message_timeout_w;
pub use send_message_timeout_w::SendMessageTimeoutW;
mod send_notify_message_a;
pub use send_notify_message_a::SendNotifyMessageA;
mod send_notify_message_w;
pub use send_notify_message_w::SendNotifyMessageW;
mod set_active_window;
pub use set_active_window::SetActiveWindow;
mod set_additional_foreground_boost_processes;
pub use set_additional_foreground_boost_processes::SetAdditionalForegroundBoostProcesses;
mod set_capture;
pub use set_capture::SetCapture;
mod set_caret_blink_time;
pub use set_caret_blink_time::SetCaretBlinkTime;
mod set_caret_pos;
pub use set_caret_pos::SetCaretPos;
mod set_class_long_a;
pub use set_class_long_a::SetClassLongA;
mod set_class_long_ptr_a;
pub use set_class_long_ptr_a::SetClassLongPtrA;
mod set_class_long_ptr_w;
pub use set_class_long_ptr_w::SetClassLongPtrW;
mod set_class_long_w;
pub use set_class_long_w::SetClassLongW;
mod set_class_word;
pub use set_class_word::SetClassWord;
mod set_clipboard_data;
pub use set_clipboard_data::SetClipboardData;
mod set_clipboard_viewer;
pub use set_clipboard_viewer::SetClipboardViewer;
mod set_coalescable_timer;
pub use set_coalescable_timer::SetCoalescableTimer;
mod set_core_window;
pub use set_core_window::SetCoreWindow;
mod set_covered_window_states;
pub use set_covered_window_states::SetCoveredWindowStates;
mod set_cursor_contents;
pub use set_cursor_contents::SetCursorContents;
mod set_debug_error_level;
pub use set_debug_error_level::SetDebugErrorLevel;
mod set_desk_wallpaper;
pub use set_desk_wallpaper::SetDeskWallpaper;
mod set_desktop_color_transform;
pub use set_desktop_color_transform::SetDesktopColorTransform;
mod set_dialog_control_dpi_change_behavior;
pub use set_dialog_control_dpi_change_behavior::SetDialogControlDpiChangeBehavior;
mod set_dialog_dpi_change_behavior;
pub use set_dialog_dpi_change_behavior::SetDialogDpiChangeBehavior;
mod set_display_auto_rotation_preferences;
pub use set_display_auto_rotation_preferences::SetDisplayAutoRotationPreferences;
mod set_display_config;
pub use set_display_config::SetDisplayConfig;
mod set_dlg_item_int;
pub use set_dlg_item_int::SetDlgItemInt;
mod set_dlg_item_text_a;
pub use set_dlg_item_text_a::SetDlgItemTextA;
mod set_dlg_item_text_w;
pub use set_dlg_item_text_w::SetDlgItemTextW;
mod set_double_click_time;
pub use set_double_click_time::SetDoubleClickTime;
mod set_feature_report_response;
pub use set_feature_report_response::SetFeatureReportResponse;
mod set_foreground_redirection_for_activation_object;
pub use set_foreground_redirection_for_activation_object::SetForegroundRedirectionForActivationObject;
mod set_fullscreen_magnifier_offsets_dwm_updated;
pub use set_fullscreen_magnifier_offsets_dwm_updated::SetFullscreenMagnifierOffsetsDWMUpdated;
mod set_gesture_config;
pub use set_gesture_config::SetGestureConfig;
mod set_internal_window_pos;
pub use set_internal_window_pos::SetInternalWindowPos;
mod set_keyboard_state;
pub use set_keyboard_state::SetKeyboardState;
mod set_last_error_ex;
pub use set_last_error_ex::SetLastErrorEx;
mod set_layered_window_attributes;
pub use set_layered_window_attributes::SetLayeredWindowAttributes;
mod set_magnification_desktop_color_effect;
pub use set_magnification_desktop_color_effect::SetMagnificationDesktopColorEffect;
mod set_magnification_desktop_magnification;
pub use set_magnification_desktop_magnification::SetMagnificationDesktopMagnification;
mod set_magnification_desktop_magnifier_offsets_dwm_updated;
pub use set_magnification_desktop_magnifier_offsets_dwm_updated::SetMagnificationDesktopMagnifierOffsetsDWMUpdated;
mod set_magnification_desktop_sampling_mode;
pub use set_magnification_desktop_sampling_mode::SetMagnificationDesktopSamplingMode;
mod set_magnification_lens_ctx_information;
pub use set_magnification_lens_ctx_information::SetMagnificationLensCtxInformation;
mod set_max_touchpad_sensitivity;
pub use set_max_touchpad_sensitivity::SetMaxTouchpadSensitivity;
mod set_menu;
pub use set_menu::SetMenu;
mod set_menu_context_help_id;
pub use set_menu_context_help_id::SetMenuContextHelpId;
mod set_menu_default_item;
pub use set_menu_default_item::SetMenuDefaultItem;
mod set_menu_info;
pub use set_menu_info::SetMenuInfo;
mod set_menu_item_bitmaps;
pub use set_menu_item_bitmaps::SetMenuItemBitmaps;
mod set_menu_item_info_a;
pub use set_menu_item_info_a::SetMenuItemInfoA;
mod set_menu_item_info_w;
pub use set_menu_item_info_w::SetMenuItemInfoW;
mod set_message_extra_info;
pub use set_message_extra_info::SetMessageExtraInfo;
mod set_message_queue;
pub use set_message_queue::SetMessageQueue;
mod set_mirror_rendering;
pub use set_mirror_rendering::SetMirrorRendering;
mod set_parent;
pub use set_parent::SetParent;
mod set_physical_cursor_pos;
pub use set_physical_cursor_pos::SetPhysicalCursorPos;
mod set_pointer_device_input_space;
pub use set_pointer_device_input_space::SetPointerDeviceInputSpace;
mod set_process_dpi_aware;
pub use set_process_dpi_aware::SetProcessDPIAware;
mod set_process_default_layout;
pub use set_process_default_layout::SetProcessDefaultLayout;
mod set_process_dpi_awareness_context;
pub use set_process_dpi_awareness_context::SetProcessDpiAwarenessContext;
mod set_process_dpi_awareness_internal;
pub use set_process_dpi_awareness_internal::SetProcessDpiAwarenessInternal;
mod set_process_launch_foreground_policy;
pub use set_process_launch_foreground_policy::SetProcessLaunchForegroundPolicy;
mod set_process_restriction_exemption;
pub use set_process_restriction_exemption::SetProcessRestrictionExemption;
mod set_process_window_station;
pub use set_process_window_station::SetProcessWindowStation;
mod set_progman_window;
pub use set_progman_window::SetProgmanWindow;
mod set_prop_a;
pub use set_prop_a::SetPropA;
mod set_prop_w;
pub use set_prop_w::SetPropW;
mod set_rect;
pub use set_rect::SetRect;
mod set_rect_empty;
pub use set_rect_empty::SetRectEmpty;
mod set_scroll_info;
pub use set_scroll_info::SetScrollInfo;
mod set_scroll_pos;
pub use set_scroll_pos::SetScrollPos;
mod set_scroll_range;
pub use set_scroll_range::SetScrollRange;
mod set_shell_change_notify_window;
pub use set_shell_change_notify_window::SetShellChangeNotifyWindow;
mod set_shell_window;
pub use set_shell_window::SetShellWindow;
mod set_shell_window_ex;
pub use set_shell_window_ex::SetShellWindowEx;
mod set_sys_colors;
pub use set_sys_colors::SetSysColors;
mod set_sys_colors_temp;
pub use set_sys_colors_temp::SetSysColorsTemp;
mod set_system_cursor;
pub use set_system_cursor::SetSystemCursor;
mod set_system_menu;
pub use set_system_menu::SetSystemMenu;
mod set_taskman_window;
pub use set_taskman_window::SetTaskmanWindow;
mod set_thread_cursor_creation_scaling;
pub use set_thread_cursor_creation_scaling::SetThreadCursorCreationScaling;
mod set_thread_desktop;
pub use set_thread_desktop::SetThreadDesktop;
mod set_thread_dpi_awareness_context;
pub use set_thread_dpi_awareness_context::SetThreadDpiAwarenessContext;
mod set_thread_dpi_hosting_behavior;
pub use set_thread_dpi_hosting_behavior::SetThreadDpiHostingBehavior;
mod set_thread_input_blocked;
pub use set_thread_input_blocked::SetThreadInputBlocked;
mod set_timer;
pub use set_timer::SetTimer;
mod set_user_object_capability;
pub use set_user_object_capability::SetUserObjectCapability;
mod set_user_object_information_a;
pub use set_user_object_information_a::SetUserObjectInformationA;
mod set_user_object_information_w;
pub use set_user_object_information_w::SetUserObjectInformationW;
mod set_user_object_security;
pub use set_user_object_security::SetUserObjectSecurity;
mod set_win_event_hook;
pub use set_win_event_hook::SetWinEventHook;
mod set_window_band;
pub use set_window_band::SetWindowBand;
mod set_window_composition_attribute;
pub use set_window_composition_attribute::SetWindowCompositionAttribute;
mod set_window_composition_transition;
pub use set_window_composition_transition::SetWindowCompositionTransition;
mod set_window_context_help_id;
pub use set_window_context_help_id::SetWindowContextHelpId;
mod set_window_display_affinity;
pub use set_window_display_affinity::SetWindowDisplayAffinity;
mod set_window_feedback_setting;
pub use set_window_feedback_setting::SetWindowFeedbackSetting;
mod set_window_message_capability;
pub use set_window_message_capability::SetWindowMessageCapability;
mod set_window_placement;
pub use set_window_placement::SetWindowPlacement;
mod set_window_rgn;
pub use set_window_rgn::SetWindowRgn;
mod set_window_rgn_ex;
pub use set_window_rgn_ex::SetWindowRgnEx;
mod set_window_station_user;
pub use set_window_station_user::SetWindowStationUser;
mod set_window_word;
pub use set_window_word::SetWindowWord;
mod set_windows_hook_a;
pub use set_windows_hook_a::SetWindowsHookA;
mod set_windows_hook_ex_aw;
pub use set_windows_hook_ex_aw::SetWindowsHookExAW;
mod set_windows_hook_ex_w;
pub use set_windows_hook_ex_w::SetWindowsHookExW;
mod set_windows_hook_w;
pub use set_windows_hook_w::SetWindowsHookW;
mod shell_foreground_boost_process;
pub use shell_foreground_boost_process::ShellForegroundBoostProcess;
mod shell_handwriting_delegate_input;
pub use shell_handwriting_delegate_input::ShellHandwritingDelegateInput;
mod shell_handwriting_handle_delegated_input;
pub use shell_handwriting_handle_delegated_input::ShellHandwritingHandleDelegatedInput;
mod shell_handwriting_undelegate_input;
pub use shell_handwriting_undelegate_input::ShellHandwritingUndelegateInput;
mod shell_migrate_window;
pub use shell_migrate_window::ShellMigrateWindow;
mod shell_register_hot_key;
pub use shell_register_hot_key::ShellRegisterHotKey;
mod shell_set_window_pos;
pub use shell_set_window_pos::ShellSetWindowPos;
mod show_caret;
pub use show_caret::ShowCaret;
mod show_owned_popups;
pub use show_owned_popups::ShowOwnedPopups;
mod show_scroll_bar;
pub use show_scroll_bar::ShowScrollBar;
mod show_start_glass;
pub use show_start_glass::ShowStartGlass;
mod show_system_cursor;
pub use show_system_cursor::ShowSystemCursor;
mod show_window_async;
pub use show_window_async::ShowWindowAsync;
mod shutdown_block_reason_create;
pub use shutdown_block_reason_create::ShutdownBlockReasonCreate;
mod shutdown_block_reason_destroy;
pub use shutdown_block_reason_destroy::ShutdownBlockReasonDestroy;
mod shutdown_block_reason_query;
pub use shutdown_block_reason_query::ShutdownBlockReasonQuery;
mod signal_redirection_start_complete;
pub use signal_redirection_start_complete::SignalRedirectionStartComplete;
mod skip_pointer_frame_messages;
pub use skip_pointer_frame_messages::SkipPointerFrameMessages;
mod soft_modal_message_box;
pub use soft_modal_message_box::SoftModalMessageBox;
mod sound_sentry;
pub use sound_sentry::SoundSentry;
mod subtract_rect;
pub use subtract_rect::SubtractRect;
mod suppress_window_display_change;
pub use suppress_window_display_change::SuppressWindowDisplayChange;
mod swap_mouse_button;
pub use swap_mouse_button::SwapMouseButton;
mod switch_desktop;
pub use switch_desktop::SwitchDesktop;
mod switch_desktop_with_fade;
pub use switch_desktop_with_fade::SwitchDesktopWithFade;
mod switch_to_this_window;
pub use switch_to_this_window::SwitchToThisWindow;
mod system_parameters_info_for_dpi;
pub use system_parameters_info_for_dpi::SystemParametersInfoForDpi;
mod system_parameters_info_w;
pub use system_parameters_info_w::SystemParametersInfoW;
mod tabbed_text_out_a;
pub use tabbed_text_out_a::TabbedTextOutA;
mod tabbed_text_out_w;
pub use tabbed_text_out_w::TabbedTextOutW;
mod tile_child_windows;
pub use tile_child_windows::TileChildWindows;
mod tile_windows;
pub use tile_windows::TileWindows;
mod to_ascii;
pub use to_ascii::ToAscii;
mod to_ascii_ex;
pub use to_ascii_ex::ToAsciiEx;
mod to_unicode;
pub use to_unicode::ToUnicode;
mod to_unicode_ex;
pub use to_unicode_ex::ToUnicodeEx;
mod track_mouse_event;
pub use track_mouse_event::TrackMouseEvent;
mod track_popup_menu;
pub use track_popup_menu::TrackPopupMenu;
mod track_popup_menu_ex;
pub use track_popup_menu_ex::TrackPopupMenuEx;
mod translate_accelerator;
pub use translate_accelerator::TranslateAccelerator;
mod translate_accelerator_a;
pub use translate_accelerator_a::TranslateAcceleratorA;
mod translate_accelerator_w;
pub use translate_accelerator_w::TranslateAcceleratorW;
mod translate_mdi_sys_accel;
pub use translate_mdi_sys_accel::TranslateMDISysAccel;
mod translate_message_ex;
pub use translate_message_ex::TranslateMessageEx;
mod undelegate_input;
pub use undelegate_input::UndelegateInput;
mod unhook_win_event;
pub use unhook_win_event::UnhookWinEvent;
mod unhook_windows_hook;
pub use unhook_windows_hook::UnhookWindowsHook;
mod unhook_windows_hook_ex;
pub use unhook_windows_hook_ex::UnhookWindowsHookEx;
mod union_rect;
pub use union_rect::UnionRect;
mod unload_keyboard_layout;
pub use unload_keyboard_layout::UnloadKeyboardLayout;
mod unlock_window_station;
pub use unlock_window_station::UnlockWindowStation;
mod unpack_dd_el_param;
pub use unpack_dd_el_param::UnpackDDElParam;
mod unregister_device_notification;
pub use unregister_device_notification::UnregisterDeviceNotification;
mod unregister_hot_key;
pub use unregister_hot_key::UnregisterHotKey;
mod unregister_message_pump_hook;
pub use unregister_message_pump_hook::UnregisterMessagePumpHook;
mod unregister_pointer_input_target;
pub use unregister_pointer_input_target::UnregisterPointerInputTarget;
mod unregister_pointer_input_target_ex;
pub use unregister_pointer_input_target_ex::UnregisterPointerInputTargetEx;
mod unregister_power_setting_notification;
pub use unregister_power_setting_notification::UnregisterPowerSettingNotification;
mod unregister_session_port;
pub use unregister_session_port::UnregisterSessionPort;
mod unregister_suspend_resume_notification;
pub use unregister_suspend_resume_notification::UnregisterSuspendResumeNotification;
mod unregister_touch_window;
pub use unregister_touch_window::UnregisterTouchWindow;
mod unregister_user_api_hook;
pub use unregister_user_api_hook::UnregisterUserApiHook;
mod update_default_desktop_thumbnail;
pub use update_default_desktop_thumbnail::UpdateDefaultDesktopThumbnail;
mod update_layered_window;
pub use update_layered_window::UpdateLayeredWindow;
mod update_layered_window_indirect;
pub use update_layered_window_indirect::UpdateLayeredWindowIndirect;
mod update_per_user_system_parameters;
pub use update_per_user_system_parameters::UpdatePerUserSystemParameters;
mod update_window_input_sink_hints;
pub use update_window_input_sink_hints::UpdateWindowInputSinkHints;
mod user32_initialize_imm_entry_table;
pub use user32_initialize_imm_entry_table::User32InitializeImmEntryTable;
mod user_client_dll_initialize;
pub use user_client_dll_initialize::UserClientDllInitialize;
mod user_handle_grant_access;
pub use user_handle_grant_access::UserHandleGrantAccess;
mod user_lpk_psm_text_out;
pub use user_lpk_psm_text_out::UserLpkPSMTextOut;
mod user_lpk_tabbed_text_out;
pub use user_lpk_tabbed_text_out::UserLpkTabbedTextOut;
mod user_realize_palette;
pub use user_realize_palette::UserRealizePalette;
mod user_register_wow_handlers;
pub use user_register_wow_handlers::UserRegisterWowHandlers;
mod v_rip_output;
pub use v_rip_output::VRipOutput;
mod v_tag_output;
pub use v_tag_output::VTagOutput;
mod validate_rect;
pub use validate_rect::ValidateRect;
mod validate_rgn;
pub use validate_rgn::ValidateRgn;
mod vk_key_scan_a;
pub use vk_key_scan_a::VkKeyScanA;
mod vk_key_scan_ex_a;
pub use vk_key_scan_ex_a::VkKeyScanExA;
mod vk_key_scan_ex_w;
pub use vk_key_scan_ex_w::VkKeyScanExW;
mod vk_key_scan_w;
pub use vk_key_scan_w::VkKeyScanW;
mod wcs_to_mb_ex;
pub use wcs_to_mb_ex::WCSToMBEx;
mod winnls_enable_ime;
pub use winnls_enable_ime::WINNLSEnableIME;
mod winnls_get_enable_status;
pub use winnls_get_enable_status::WINNLSGetEnableStatus;
mod winnls_get_ime_hotkey;
pub use winnls_get_ime_hotkey::WINNLSGetIMEHotkey;
mod wait_for_input_idle;
pub use wait_for_input_idle::WaitForInputIdle;
mod wait_for_redirection_start_complete;
pub use wait_for_redirection_start_complete::WaitForRedirectionStartComplete;
mod wait_message;
pub use wait_message::WaitMessage;
mod win_help_a;
pub use win_help_a::WinHelpA;
mod win_help_w;
pub use win_help_w::WinHelpW;
mod window_from_dc;
pub use window_from_dc::WindowFromDC;
mod window_from_physical_point;
pub use window_from_physical_point::WindowFromPhysicalPoint;
mod window_from_point;
pub use window_from_point::WindowFromPoint;
mod _user_test_token_for_interactive;
pub use _user_test_token_for_interactive::_UserTestTokenForInteractive;
mod g_shared_info;
pub use g_shared_info::gSharedInfo;
mod gapfn_sc_send_message;
pub use gapfn_sc_send_message::gapfnScSendMessage;
mod keybd_event;
pub use keybd_event::keybd_event;
mod mouse_event;
pub use mouse_event::mouse_event;
mod wvsprintf_a;
pub use wvsprintf_a::wvsprintfA;
mod wvsprintf_w;
pub use wvsprintf_w::wvsprintfW;
mod wow64_transition;
pub use wow64_transition::Wow64Transition;
mod register_class_ex_a;
pub use register_class_ex_a::RegisterClassExA;
mod register_class_ex_w;
pub use register_class_ex_w::RegisterClassExW;
mod unregister_class_a;
pub use unregister_class_a::UnregisterClassA;
mod unregister_class_w;
pub use unregister_class_w::UnregisterClassW;
mod enum_windows;
pub use enum_windows::EnumWindows;
mod find_window_ex_a;
pub use find_window_ex_a::FindWindowExA;
mod find_window_ex_w;
pub use find_window_ex_w::FindWindowExW;
mod get_window_long_ptr_a;
pub use get_window_long_ptr_a::GetWindowLongPtrA;
mod get_window_long_ptr_w;
pub use get_window_long_ptr_w::GetWindowLongPtrW;
mod set_window_long_ptr_a;
pub use set_window_long_ptr_a::SetWindowLongPtrA;
mod set_window_long_ptr_w;
pub use set_window_long_ptr_w::SetWindowLongPtrW;
mod get_physical_cursor_pos;
pub use get_physical_cursor_pos::GetPhysicalCursorPos;
mod adjust_window_rect;
use adjust_window_rect::AdjustWindowRect;
mod adjust_window_rect_ex;
use adjust_window_rect_ex::AdjustWindowRectEx;
mod begin_paint;
use begin_paint::BeginPaint;
mod char_lower_buff_a;
use char_lower_buff_a::CharLowerBuffA;
mod char_lower_buff_w;
use char_lower_buff_w::CharLowerBuffW;
mod char_lower_w;
use char_lower_w::CharLowerW;
mod char_upper_buff_a;
use char_upper_buff_a::CharUpperBuffA;
mod char_upper_buff_w;
use char_upper_buff_w::CharUpperBuffW;
mod create_window_ex_a;
use create_window_ex_a::CreateWindowExA;
mod create_window_ex_w;
use create_window_ex_w::CreateWindowExW;
mod def_window_proc_a;
use def_window_proc_a::DefWindowProcA;
mod def_window_proc_w;
use def_window_proc_w::DefWindowProcW;
mod destroy_window;
use destroy_window::DestroyWindow;
mod dispatch_message_a;
use dispatch_message_a::DispatchMessageA;
mod dispatch_message_w;
use dispatch_message_w::DispatchMessageW;
mod draw_text_a;
use draw_text_a::DrawTextA;
mod draw_text_w;
use draw_text_w::DrawTextW;
mod enable_window;
use enable_window::EnableWindow;
mod end_paint;
use end_paint::EndPaint;
mod fill_rect;
use fill_rect::FillRect;
mod find_window_a;
use find_window_a::FindWindowA;
mod find_window_w;
use find_window_w::FindWindowW;
mod get_async_key_state;
use get_async_key_state::GetAsyncKeyState;
mod get_class_info_ex_a;
use get_class_info_ex_a::GetClassInfoExA;
mod get_class_info_ex_w;
use get_class_info_ex_w::GetClassInfoExW;
mod get_client_rect;
use get_client_rect::GetClientRect;
mod get_cursor_pos;
use get_cursor_pos::GetCursorPos;
mod get_dc;
use get_dc::GetDC;
mod get_desktop_window;
use get_desktop_window::GetDesktopWindow;
mod get_focus;
use get_focus::GetFocus;
mod get_foreground_window;
use get_foreground_window::GetForegroundWindow;
mod get_key_state;
use get_key_state::GetKeyState;
mod get_keyboard_layout;
use get_keyboard_layout::GetKeyboardLayout;
mod get_keyboard_type;
use get_keyboard_type::GetKeyboardType;
mod get_message_a;
use get_message_a::GetMessageA;
mod get_message_w;
use get_message_w::GetMessageW;
mod get_process_window_station;
use get_process_window_station::GetProcessWindowStation;
mod get_system_metrics;
use get_system_metrics::GetSystemMetrics;
mod get_user_object_information_w;
use get_user_object_information_w::GetUserObjectInformationW;
mod get_window_long_a;
use get_window_long_a::GetWindowLongA;
mod get_window_long_w;
use get_window_long_w::GetWindowLongW;
mod get_window_rect;
use get_window_rect::GetWindowRect;
mod get_window_text_a;
use get_window_text_a::GetWindowTextA;
mod get_window_text_w;
use get_window_text_w::GetWindowTextW;
mod invalidate_rect;
use invalidate_rect::InvalidateRect;
mod is_window;
use is_window::IsWindow;
mod is_window_visible;
use is_window_visible::IsWindowVisible;
mod load_cursor_a;
use load_cursor_a::LoadCursorA;
mod load_icon_a;
use load_icon_a::LoadIconA;
mod load_string_a;
use load_string_a::LoadStringA;
mod load_string_w;
use load_string_w::LoadStringW;
mod map_virtual_key_a;
use map_virtual_key_a::MapVirtualKeyA;
mod map_virtual_key_w;
use map_virtual_key_w::MapVirtualKeyW;
mod message_box_a;
use message_box_a::MessageBoxA;
mod message_box_w;
use message_box_w::MessageBoxW;
mod peek_message_a;
use peek_message_a::PeekMessageA;
mod peek_message_w;
use peek_message_w::PeekMessageW;
mod post_message_a;
use post_message_a::PostMessageA;
mod post_message_w;
use post_message_w::PostMessageW;
mod post_quit_message;
use post_quit_message::PostQuitMessage;
mod register_class_a;
use register_class_a::RegisterClassA;
mod register_class_w;
use register_class_w::RegisterClassW;
mod release_dc;
use release_dc::ReleaseDC;
mod send_message_a;
use send_message_a::SendMessageA;
mod send_message_w;
use send_message_w::SendMessageW;
mod set_cursor;
use set_cursor::SetCursor;
mod set_cursor_pos;
use set_cursor_pos::SetCursorPos;
mod set_focus;
use set_focus::SetFocus;
mod set_foreground_window;
use set_foreground_window::SetForegroundWindow;
mod set_window_long_a;
use set_window_long_a::SetWindowLongA;
mod set_window_long_w;
use set_window_long_w::SetWindowLongW;
mod set_window_pos;
use set_window_pos::SetWindowPos;
mod set_window_text_a;
use set_window_text_a::SetWindowTextA;
mod set_window_text_w;
use set_window_text_w::SetWindowTextW;
mod set_windows_hook_ex_a;
use set_windows_hook_ex_a::SetWindowsHookExA;
mod show_cursor;
use show_cursor::ShowCursor;
mod show_window;
use show_window::ShowWindow;
mod system_parameters_info_a;
use system_parameters_info_a::SystemParametersInfoA;
mod translate_message;
use translate_message::TranslateMessage;
mod update_window;
use update_window::UpdateWindow;
mod wsprintf_a;
use wsprintf_a::wsprintfA;
mod wsprintf_w;
use wsprintf_w::wsprintfW;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "MessageBoxA" => MessageBoxA(emu),
        "MessageBoxW" => MessageBoxW(emu),
        "GetDesktopWindow" => GetDesktopWindow(emu),
        "wsprintfW" => wsprintfW(emu),
        "GetProcessWindowStation" => GetProcessWindowStation(emu),
        "GetUserObjectInformationW" => GetUserObjectInformationW(emu),
        "CharLowerW" => CharLowerW(emu),
        "wsprintfA" => wsprintfA(emu),
        "GetSystemMetrics" => GetSystemMetrics(emu),
        "SystemParametersInfoA" => SystemParametersInfoA(emu),
        "LoadIconA" => LoadIconA(emu),
        "LoadCursorA" => LoadCursorA(emu),
        "RegisterClassA" => RegisterClassA(emu),
        "RegisterClassW" => RegisterClassW(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "CharLowerBuffA" => CharLowerBuffA(emu),
        "CharLowerBuffW" => CharLowerBuffW(emu),
        "CharUpperBuffA" => CharUpperBuffA(emu),
        "CharUpperBuffW" => CharUpperBuffW(emu),
        "FindWindowA" => FindWindowA(emu),
        "FindWindowW" => FindWindowW(emu),
        "GetKeyState" => GetKeyState(emu),
        "GetAsyncKeyState" => GetAsyncKeyState(emu),
        "SetWindowsHookExA" => SetWindowsHookExA(emu),
        "AdjustWindowRect" => AdjustWindowRect(emu),
        "AdjustWindowRectEx" => AdjustWindowRectEx(emu),
        "CreateWindowExA" => CreateWindowExA(emu),
        "CreateWindowExW" => CreateWindowExW(emu),
        "ShowWindow" => ShowWindow(emu),
        "UpdateWindow" => UpdateWindow(emu),
        "DestroyWindow" => DestroyWindow(emu),
        "PostMessageA" => PostMessageA(emu),
        "PostMessageW" => PostMessageW(emu),
        "SendMessageA" => SendMessageA(emu),
        "SendMessageW" => SendMessageW(emu),
        "DefWindowProcA" => DefWindowProcA(emu),
        "DefWindowProcW" => DefWindowProcW(emu),
        "GetWindowRect" => GetWindowRect(emu),
        "GetClientRect" => GetClientRect(emu),
        "SetWindowPos" => SetWindowPos(emu),
        "SetWindowTextA" => SetWindowTextA(emu),
        "SetWindowTextW" => SetWindowTextW(emu),
        "GetWindowTextA" => GetWindowTextA(emu),
        "GetWindowTextW" => GetWindowTextW(emu),
        "GetWindowLongA" => GetWindowLongA(emu),
        "GetWindowLongW" => GetWindowLongW(emu),
        "SetWindowLongA" => SetWindowLongA(emu),
        "SetWindowLongW" => SetWindowLongW(emu),
        "PeekMessageA" => PeekMessageA(emu),
        "PeekMessageW" => PeekMessageW(emu),
        "GetMessageA" => GetMessageA(emu),
        "GetMessageW" => GetMessageW(emu),
        "TranslateMessage" => TranslateMessage(emu),
        "DispatchMessageA" => DispatchMessageA(emu),
        "DispatchMessageW" => DispatchMessageW(emu),
        "PostQuitMessage" => PostQuitMessage(emu),
        "IsWindow" => IsWindow(emu),
        "IsWindowVisible" => IsWindowVisible(emu),
        "EnableWindow" => EnableWindow(emu),
        "SetFocus" => SetFocus(emu),
        "GetFocus" => GetFocus(emu),
        "GetForegroundWindow" => GetForegroundWindow(emu),
        "SetForegroundWindow" => SetForegroundWindow(emu),
        "GetCursorPos" => GetCursorPos(emu),
        "SetCursorPos" => SetCursorPos(emu),
        "SetCursor" => SetCursor(emu),
        "ShowCursor" => ShowCursor(emu),
        "LoadStringA" => LoadStringA(emu),
        "LoadStringW" => LoadStringW(emu),
        "GetClassInfoExA" => GetClassInfoExA(emu),
        "GetClassInfoExW" => GetClassInfoExW(emu),
        "MapVirtualKeyA" => MapVirtualKeyA(emu),
        "MapVirtualKeyW" => MapVirtualKeyW(emu),
        "GetKeyboardLayout" => GetKeyboardLayout(emu),
        "GetKeyboardType" => GetKeyboardType(emu),
        "BeginPaint" => BeginPaint(emu),
        "EndPaint" => EndPaint(emu),
        "InvalidateRect" => InvalidateRect(emu),
        "FillRect" => FillRect(emu),
        "DrawTextA" => DrawTextA(emu),
        "DrawTextW" => DrawTextW(emu),
        "ActivateKeyboardLayout" => ActivateKeyboardLayout(emu),
        "AddClipboardFormatListener" => AddClipboardFormatListener(emu),
        "AddVisualIdentifier" => AddVisualIdentifier(emu),
        "AdjustWindowRectExForDpi" => AdjustWindowRectExForDpi(emu),
        "AlignRects" => AlignRects(emu),
        "AllowForegroundActivation" => AllowForegroundActivation(emu),
        "AllowSetForegroundWindow" => AllowSetForegroundWindow(emu),
        "AnimateWindow" => AnimateWindow(emu),
        "AnyPopup" => AnyPopup(emu),
        "AppendMenuA" => AppendMenuA(emu),
        "AppendMenuW" => AppendMenuW(emu),
        "ApplyWindowAction" => ApplyWindowAction(emu),
        "AreDpiAwarenessContextsEqual" => AreDpiAwarenessContextsEqual(emu),
        "ArrangeIconicWindows" => ArrangeIconicWindows(emu),
        "AttachThreadInput" => AttachThreadInput(emu),
        "BeginDeferWindowPos" => BeginDeferWindowPos(emu),
        "BlockInput" => BlockInput(emu),
        "BringWindowToTop" => BringWindowToTop(emu),
        "BroadcastSystemMessage" => BroadcastSystemMessage(emu),
        "BroadcastSystemMessageA" => BroadcastSystemMessageA(emu),
        "BroadcastSystemMessageExA" => BroadcastSystemMessageExA(emu),
        "BroadcastSystemMessageExW" => BroadcastSystemMessageExW(emu),
        "BroadcastSystemMessageW" => BroadcastSystemMessageW(emu),
        "BuildReasonArray" => BuildReasonArray(emu),
        "CalcMenuBar" => CalcMenuBar(emu),
        "CalculatePopupWindowPosition" => CalculatePopupWindowPosition(emu),
        "CallMsgFilter" => CallMsgFilter(emu),
        "CallMsgFilterA" => CallMsgFilterA(emu),
        "CallMsgFilterW" => CallMsgFilterW(emu),
        "CallNextHookEx" => CallNextHookEx(emu),
        "CallWindowProcA" => CallWindowProcA(emu),
        "CallWindowProcW" => CallWindowProcW(emu),
        "CancelShutdown" => CancelShutdown(emu),
        "CascadeChildWindows" => CascadeChildWindows(emu),
        "CascadeWindows" => CascadeWindows(emu),
        "ChangeClipboardChain" => ChangeClipboardChain(emu),
        "ChangeDisplaySettingsA" => ChangeDisplaySettingsA(emu),
        "ChangeDisplaySettingsExA" => ChangeDisplaySettingsExA(emu),
        "ChangeDisplaySettingsExW" => ChangeDisplaySettingsExW(emu),
        "ChangeDisplaySettingsW" => ChangeDisplaySettingsW(emu),
        "ChangeMenuA" => ChangeMenuA(emu),
        "ChangeMenuW" => ChangeMenuW(emu),
        "ChangeWindowMessageFilter" => ChangeWindowMessageFilter(emu),
        "ChangeWindowMessageFilterEx" => ChangeWindowMessageFilterEx(emu),
        "CharLowerA" => CharLowerA(emu),
        "CharNextA" => CharNextA(emu),
        "CharNextExA" => CharNextExA(emu),
        "CharNextW" => CharNextW(emu),
        "CharPrevA" => CharPrevA(emu),
        "CharPrevExA" => CharPrevExA(emu),
        "CharPrevW" => CharPrevW(emu),
        "CharToOemA" => CharToOemA(emu),
        "CharToOemBuffA" => CharToOemBuffA(emu),
        "CharToOemBuffW" => CharToOemBuffW(emu),
        "CharToOemW" => CharToOemW(emu),
        "CharUpperA" => CharUpperA(emu),
        "CharUpperW" => CharUpperW(emu),
        "CheckBannedOneCoreTransformApi" => CheckBannedOneCoreTransformApi(emu),
        "CheckDBCSEnabledExt" => CheckDBCSEnabledExt(emu),
        "CheckDlgButton" => CheckDlgButton(emu),
        "CheckMenuItem" => CheckMenuItem(emu),
        "CheckMenuRadioItem" => CheckMenuRadioItem(emu),
        "CheckProcessForClipboardAccess" => CheckProcessForClipboardAccess(emu),
        "CheckProcessSession" => CheckProcessSession(emu),
        "CheckRadioButton" => CheckRadioButton(emu),
        "CheckWindowThreadDesktop" => CheckWindowThreadDesktop(emu),
        "ChildWindowFromPoint" => ChildWindowFromPoint(emu),
        "ChildWindowFromPointEx" => ChildWindowFromPointEx(emu),
        "CliImmSetHotKey" => CliImmSetHotKey(emu),
        "ClientThreadSetup" => ClientThreadSetup(emu),
        "ClientToScreen" => ClientToScreen(emu),
        "ClipCursor" => ClipCursor(emu),
        "CloseClipboard" => CloseClipboard(emu),
        "CloseDesktop" => CloseDesktop(emu),
        "CloseGestureInfoHandle" => CloseGestureInfoHandle(emu),
        "CloseTouchInputHandle" => CloseTouchInputHandle(emu),
        "CloseWindow" => CloseWindow(emu),
        "CloseWindowStation" => CloseWindowStation(emu),
        "ConsoleControl" => ConsoleControl(emu),
        "ControlMagnification" => ControlMagnification(emu),
        "ConvertPrimaryPointerToMouseDrag" => ConvertPrimaryPointerToMouseDrag(emu),
        "ConvertToInterceptWindow" => ConvertToInterceptWindow(emu),
        "CopyAcceleratorTableA" => CopyAcceleratorTableA(emu),
        "CopyAcceleratorTableW" => CopyAcceleratorTableW(emu),
        "CopyIcon" => CopyIcon(emu),
        "CopyImage" => CopyImage(emu),
        "CopyRect" => CopyRect(emu),
        "CountClipboardFormats" => CountClipboardFormats(emu),
        "CreateAcceleratorTableA" => CreateAcceleratorTableA(emu),
        "CreateAcceleratorTableW" => CreateAcceleratorTableW(emu),
        "CreateCaret" => CreateCaret(emu),
        "CreateCursor" => CreateCursor(emu),
        "CreateDCompositionHwndTarget" => CreateDCompositionHwndTarget(emu),
        "CreateDesktopA" => CreateDesktopA(emu),
        "CreateDesktopExA" => CreateDesktopExA(emu),
        "CreateDesktopExW" => CreateDesktopExW(emu),
        "CreateDesktopW" => CreateDesktopW(emu),
        "CreateDialogIndirectParamA" => CreateDialogIndirectParamA(emu),
        "CreateDialogIndirectParamAorW" => CreateDialogIndirectParamAorW(emu),
        "CreateDialogIndirectParamW" => CreateDialogIndirectParamW(emu),
        "CreateDialogParamA" => CreateDialogParamA(emu),
        "CreateDialogParamW" => CreateDialogParamW(emu),
        "CreateIcon" => CreateIcon(emu),
        "CreateIconFromResource" => CreateIconFromResource(emu),
        "CreateIconFromResourceEx" => CreateIconFromResourceEx(emu),
        "CreateIconIndirect" => CreateIconIndirect(emu),
        "CreateMDIWindowA" => CreateMDIWindowA(emu),
        "CreateMDIWindowW" => CreateMDIWindowW(emu),
        "CreateMenu" => CreateMenu(emu),
        "CreatePopupMenu" => CreatePopupMenu(emu),
        "CreateSyntheticPointerDevice" => CreateSyntheticPointerDevice(emu),
        "CreateSyntheticPointerDevice2" => CreateSyntheticPointerDevice2(emu),
        "CreateSystemThreads" => CreateSystemThreads(emu),
        "CreateWindowInBand" => CreateWindowInBand(emu),
        "CreateWindowInBandEx" => CreateWindowInBandEx(emu),
        "CreateWindowIndirect" => CreateWindowIndirect(emu),
        "CreateWindowStationA" => CreateWindowStationA(emu),
        "CreateWindowStationW" => CreateWindowStationW(emu),
        "CsrBroadcastSystemMessageExW" => CsrBroadcastSystemMessageExW(emu),
        "CtxInitUser32" => CtxInitUser32(emu),
        "DdeAbandonTransaction" => DdeAbandonTransaction(emu),
        "DdeAccessData" => DdeAccessData(emu),
        "DdeAddData" => DdeAddData(emu),
        "DdeClientTransaction" => DdeClientTransaction(emu),
        "DdeCmpStringHandles" => DdeCmpStringHandles(emu),
        "DdeConnect" => DdeConnect(emu),
        "DdeConnectList" => DdeConnectList(emu),
        "DdeCreateDataHandle" => DdeCreateDataHandle(emu),
        "DdeCreateStringHandleA" => DdeCreateStringHandleA(emu),
        "DdeCreateStringHandleW" => DdeCreateStringHandleW(emu),
        "DdeDisconnect" => DdeDisconnect(emu),
        "DdeDisconnectList" => DdeDisconnectList(emu),
        "DdeEnableCallback" => DdeEnableCallback(emu),
        "DdeFreeDataHandle" => DdeFreeDataHandle(emu),
        "DdeFreeStringHandle" => DdeFreeStringHandle(emu),
        "DdeGetData" => DdeGetData(emu),
        "DdeGetLastError" => DdeGetLastError(emu),
        "DdeGetQualityOfService" => DdeGetQualityOfService(emu),
        "DdeImpersonateClient" => DdeImpersonateClient(emu),
        "DdeInitializeA" => DdeInitializeA(emu),
        "DdeInitializeW" => DdeInitializeW(emu),
        "DdeKeepStringHandle" => DdeKeepStringHandle(emu),
        "DdeNameService" => DdeNameService(emu),
        "DdePostAdvise" => DdePostAdvise(emu),
        "DdeQueryConvInfo" => DdeQueryConvInfo(emu),
        "DdeQueryNextServer" => DdeQueryNextServer(emu),
        "DdeQueryStringA" => DdeQueryStringA(emu),
        "DdeQueryStringW" => DdeQueryStringW(emu),
        "DdeReconnect" => DdeReconnect(emu),
        "DdeSetQualityOfService" => DdeSetQualityOfService(emu),
        "DdeSetUserHandle" => DdeSetUserHandle(emu),
        "DdeUnaccessData" => DdeUnaccessData(emu),
        "DdeUninitialize" => DdeUninitialize(emu),
        "DefDlgProcA" => DefDlgProcA(emu),
        "DefDlgProcW" => DefDlgProcW(emu),
        "DefFrameProcA" => DefFrameProcA(emu),
        "DefFrameProcW" => DefFrameProcW(emu),
        "DefMDIChildProcA" => DefMDIChildProcA(emu),
        "DefMDIChildProcW" => DefMDIChildProcW(emu),
        "DefRawInputProc" => DefRawInputProc(emu),
        "DeferWindowPos" => DeferWindowPos(emu),
        "DeferWindowPosAndBand" => DeferWindowPosAndBand(emu),
        "DelegateInput" => DelegateInput(emu),
        "DeleteMenu" => DeleteMenu(emu),
        "DeregisterShellHookWindow" => DeregisterShellHookWindow(emu),
        "DestroyAcceleratorTable" => DestroyAcceleratorTable(emu),
        "DestroyCaret" => DestroyCaret(emu),
        "DestroyCursor" => DestroyCursor(emu),
        "DestroyDCompositionHwndTarget" => DestroyDCompositionHwndTarget(emu),
        "DestroyIcon" => DestroyIcon(emu),
        "DestroyMenu" => DestroyMenu(emu),
        "DestroyReasons" => DestroyReasons(emu),
        "DestroySyntheticPointerDevice" => DestroySyntheticPointerDevice(emu),
        "DialogBoxIndirectParamA" => DialogBoxIndirectParamA(emu),
        "DialogBoxIndirectParamAorW" => DialogBoxIndirectParamAorW(emu),
        "DialogBoxIndirectParamW" => DialogBoxIndirectParamW(emu),
        "DialogBoxParamA" => DialogBoxParamA(emu),
        "DialogBoxParamW" => DialogBoxParamW(emu),
        "DisableProcessWindowsGhosting" => DisableProcessWindowsGhosting(emu),
        "DisplayConfigGetDeviceInfo" => DisplayConfigGetDeviceInfo(emu),
        "DisplayConfigSetDeviceInfo" => DisplayConfigSetDeviceInfo(emu),
        "DisplayExitWindowsWarnings" => DisplayExitWindowsWarnings(emu),
        "DlgDirListA" => DlgDirListA(emu),
        "DlgDirListComboBoxA" => DlgDirListComboBoxA(emu),
        "DlgDirListComboBoxW" => DlgDirListComboBoxW(emu),
        "DlgDirListW" => DlgDirListW(emu),
        "DlgDirSelectComboBoxExA" => DlgDirSelectComboBoxExA(emu),
        "DlgDirSelectComboBoxExW" => DlgDirSelectComboBoxExW(emu),
        "DlgDirSelectExA" => DlgDirSelectExA(emu),
        "DlgDirSelectExW" => DlgDirSelectExW(emu),
        "DoSoundConnect" => DoSoundConnect(emu),
        "DoSoundDisconnect" => DoSoundDisconnect(emu),
        "DragDetect" => DragDetect(emu),
        "DragObject" => DragObject(emu),
        "DrawAnimatedRects" => DrawAnimatedRects(emu),
        "DrawCaption" => DrawCaption(emu),
        "DrawCaptionTempA" => DrawCaptionTempA(emu),
        "DrawCaptionTempW" => DrawCaptionTempW(emu),
        "DrawEdge" => DrawEdge(emu),
        "DrawFocusRect" => DrawFocusRect(emu),
        "DrawFrame" => DrawFrame(emu),
        "DrawFrameControl" => DrawFrameControl(emu),
        "DrawIcon" => DrawIcon(emu),
        "DrawIconEx" => DrawIconEx(emu),
        "DrawMenuBar" => DrawMenuBar(emu),
        "DrawMenuBarTemp" => DrawMenuBarTemp(emu),
        "DrawStateA" => DrawStateA(emu),
        "DrawStateW" => DrawStateW(emu),
        "DrawTextExA" => DrawTextExA(emu),
        "DrawTextExW" => DrawTextExW(emu),
        "DwmGetDxRgn" => DwmGetDxRgn(emu),
        "DwmGetDxSharedSurface" => DwmGetDxSharedSurface(emu),
        "DwmGetRemoteSessionOcclusionEvent" => DwmGetRemoteSessionOcclusionEvent(emu),
        "DwmGetRemoteSessionOcclusionState" => DwmGetRemoteSessionOcclusionState(emu),
        "DwmKernelShutdown" => DwmKernelShutdown(emu),
        "DwmKernelStartup" => DwmKernelStartup(emu),
        "DwmLockScreenUpdates" => DwmLockScreenUpdates(emu),
        "DwmValidateWindow" => DwmValidateWindow(emu),
        "DwmWindowNotificationsEnabled" => DwmWindowNotificationsEnabled(emu),
        "EditWndProc" => EditWndProc(emu),
        "EmptyClipboard" => EmptyClipboard(emu),
        "EnableMenuItem" => EnableMenuItem(emu),
        "EnableMouseInPointer" => EnableMouseInPointer(emu),
        "EnableNonClientDpiScaling" => EnableNonClientDpiScaling(emu),
        "EnableOneCoreTransformMode" => EnableOneCoreTransformMode(emu),
        "EnableScrollBar" => EnableScrollBar(emu),
        "EnableSessionForMMCSS" => EnableSessionForMMCSS(emu),
        "EnableWindowShellWindowManagementBehavior" => {
            EnableWindowShellWindowManagementBehavior(emu)
        }
        "EndDeferWindowPos" => EndDeferWindowPos(emu),
        "EndDeferWindowPosEx" => EndDeferWindowPosEx(emu),
        "EndDialog" => EndDialog(emu),
        "EndMenu" => EndMenu(emu),
        "EndTask" => EndTask(emu),
        "EnterMoveSizeLoop" => EnterMoveSizeLoop(emu),
        "EnterReaderModeHelper" => EnterReaderModeHelper(emu),
        "EnumChildWindows" => EnumChildWindows(emu),
        "EnumClipboardFormats" => EnumClipboardFormats(emu),
        "EnumDesktopWindows" => EnumDesktopWindows(emu),
        "EnumDesktopsA" => EnumDesktopsA(emu),
        "EnumDesktopsW" => EnumDesktopsW(emu),
        "EnumDisplayDevicesA" => EnumDisplayDevicesA(emu),
        "EnumDisplayDevicesW" => EnumDisplayDevicesW(emu),
        "EnumDisplayMonitors" => EnumDisplayMonitors(emu),
        "EnumDisplaySettingsA" => EnumDisplaySettingsA(emu),
        "EnumDisplaySettingsExA" => EnumDisplaySettingsExA(emu),
        "EnumDisplaySettingsExW" => EnumDisplaySettingsExW(emu),
        "EnumDisplaySettingsW" => EnumDisplaySettingsW(emu),
        "EnumPropsA" => EnumPropsA(emu),
        "EnumPropsExA" => EnumPropsExA(emu),
        "EnumPropsExW" => EnumPropsExW(emu),
        "EnumPropsW" => EnumPropsW(emu),
        "EnumThreadWindows" => EnumThreadWindows(emu),
        "EnumWindowStationsA" => EnumWindowStationsA(emu),
        "EnumWindowStationsW" => EnumWindowStationsW(emu),
        "EqualRect" => EqualRect(emu),
        "EvaluateProximityToPolygon" => EvaluateProximityToPolygon(emu),
        "EvaluateProximityToRect" => EvaluateProximityToRect(emu),
        "ExcludeUpdateRgn" => ExcludeUpdateRgn(emu),
        "ExitWindowsEx" => ExitWindowsEx(emu),
        "FlashWindow" => FlashWindow(emu),
        "FlashWindowEx" => FlashWindowEx(emu),
        "FrameRect" => FrameRect(emu),
        "FreeDDElParam" => FreeDDElParam(emu),
        "FrostCrashedWindow" => FrostCrashedWindow(emu),
        "GetActiveWindow" => GetActiveWindow(emu),
        "GetAltTabInfo" => GetAltTabInfo(emu),
        "GetAltTabInfoA" => GetAltTabInfoA(emu),
        "GetAltTabInfoW" => GetAltTabInfoW(emu),
        "GetAncestor" => GetAncestor(emu),
        "GetAppCompatFlags" => GetAppCompatFlags(emu),
        "GetAppCompatFlags2" => GetAppCompatFlags2(emu),
        "GetAutoRotationState" => GetAutoRotationState(emu),
        "GetAwarenessFromDpiAwarenessContext" => GetAwarenessFromDpiAwarenessContext(emu),
        "GetCIMSSM" => GetCIMSSM(emu),
        "GetCapture" => GetCapture(emu),
        "GetCaretBlinkTime" => GetCaretBlinkTime(emu),
        "GetCaretPos" => GetCaretPos(emu),
        "GetClassInfoA" => GetClassInfoA(emu),
        "GetClassInfoW" => GetClassInfoW(emu),
        "GetClassLongA" => GetClassLongA(emu),
        "GetClassLongPtrA" => GetClassLongPtrA(emu),
        "GetClassLongPtrW" => GetClassLongPtrW(emu),
        "GetClassLongW" => GetClassLongW(emu),
        "GetClassNameA" => GetClassNameA(emu),
        "GetClassNameW" => GetClassNameW(emu),
        "GetClassWord" => GetClassWord(emu),
        "GetClipCursor" => GetClipCursor(emu),
        "GetClipboardAccessToken" => GetClipboardAccessToken(emu),
        "GetClipboardData" => GetClipboardData(emu),
        "GetClipboardFormatNameA" => GetClipboardFormatNameA(emu),
        "GetClipboardFormatNameW" => GetClipboardFormatNameW(emu),
        "GetClipboardMetadata" => GetClipboardMetadata(emu),
        "GetClipboardOwner" => GetClipboardOwner(emu),
        "GetClipboardSequenceNumber" => GetClipboardSequenceNumber(emu),
        "GetClipboardViewer" => GetClipboardViewer(emu),
        "GetComboBoxInfo" => GetComboBoxInfo(emu),
        "GetCurrentInputMessageSource" => GetCurrentInputMessageSource(emu),
        "GetCurrentMonitorTopologyId" => GetCurrentMonitorTopologyId(emu),
        "GetCursor" => GetCursor(emu),
        "GetCursorFrameInfo" => GetCursorFrameInfo(emu),
        "GetCursorInfo" => GetCursorInfo(emu),
        "GetDCEx" => GetDCEx(emu),
        "GetDCompositionHwndBitmap" => GetDCompositionHwndBitmap(emu),
        "GetDesktopID" => GetDesktopID(emu),
        "GetDialogBaseUnits" => GetDialogBaseUnits(emu),
        "GetDialogControlDpiChangeBehavior" => GetDialogControlDpiChangeBehavior(emu),
        "GetDialogDpiChangeBehavior" => GetDialogDpiChangeBehavior(emu),
        "GetDisplayAutoRotationPreferences" => GetDisplayAutoRotationPreferences(emu),
        "GetDisplayConfigBufferSizes" => GetDisplayConfigBufferSizes(emu),
        "GetDlgCtrlID" => GetDlgCtrlID(emu),
        "GetDlgItem" => GetDlgItem(emu),
        "GetDlgItemInt" => GetDlgItemInt(emu),
        "GetDlgItemTextA" => GetDlgItemTextA(emu),
        "GetDlgItemTextW" => GetDlgItemTextW(emu),
        "GetDoubleClickTime" => GetDoubleClickTime(emu),
        "GetDpiAwarenessContextForProcess" => GetDpiAwarenessContextForProcess(emu),
        "GetDpiForMonitorInternal" => GetDpiForMonitorInternal(emu),
        "GetDpiForSystem" => GetDpiForSystem(emu),
        "GetDpiForWindow" => GetDpiForWindow(emu),
        "GetDpiFromDpiAwarenessContext" => GetDpiFromDpiAwarenessContext(emu),
        "GetExtendedPointerDeviceProperty" => GetExtendedPointerDeviceProperty(emu),
        "GetGUIThreadInfo" => GetGUIThreadInfo(emu),
        "GetGestureConfig" => GetGestureConfig(emu),
        "GetGestureExtraArgs" => GetGestureExtraArgs(emu),
        "GetGestureInfo" => GetGestureInfo(emu),
        "GetGuiResources" => GetGuiResources(emu),
        "GetIconInfo" => GetIconInfo(emu),
        "GetIconInfoExA" => GetIconInfoExA(emu),
        "GetIconInfoExW" => GetIconInfoExW(emu),
        "GetInputDesktop" => GetInputDesktop(emu),
        "GetInputLocaleInfo" => GetInputLocaleInfo(emu),
        "GetInputState" => GetInputState(emu),
        "GetInternalWindowPos" => GetInternalWindowPos(emu),
        "GetKBCodePage" => GetKBCodePage(emu),
        "GetKeyNameTextA" => GetKeyNameTextA(emu),
        "GetKeyNameTextW" => GetKeyNameTextW(emu),
        "GetKeyboardLayoutList" => GetKeyboardLayoutList(emu),
        "GetKeyboardLayoutNameA" => GetKeyboardLayoutNameA(emu),
        "GetKeyboardLayoutNameW" => GetKeyboardLayoutNameW(emu),
        "GetKeyboardState" => GetKeyboardState(emu),
        "GetLastActivePopup" => GetLastActivePopup(emu),
        "GetLastInputInfo" => GetLastInputInfo(emu),
        "GetLayeredWindowAttributes" => GetLayeredWindowAttributes(emu),
        "GetListBoxInfo" => GetListBoxInfo(emu),
        "GetMagnificationDesktopColorEffect" => GetMagnificationDesktopColorEffect(emu),
        "GetMagnificationDesktopMagnification" => GetMagnificationDesktopMagnification(emu),
        "GetMagnificationDesktopSamplingMode" => GetMagnificationDesktopSamplingMode(emu),
        "GetMagnificationLensCtxInformation" => GetMagnificationLensCtxInformation(emu),
        "GetMenu" => GetMenu(emu),
        "GetMenuBarInfo" => GetMenuBarInfo(emu),
        "GetMenuCheckMarkDimensions" => GetMenuCheckMarkDimensions(emu),
        "GetMenuContextHelpId" => GetMenuContextHelpId(emu),
        "GetMenuDefaultItem" => GetMenuDefaultItem(emu),
        "GetMenuInfo" => GetMenuInfo(emu),
        "GetMenuItemCount" => GetMenuItemCount(emu),
        "GetMenuItemID" => GetMenuItemID(emu),
        "GetMenuItemInfoA" => GetMenuItemInfoA(emu),
        "GetMenuItemInfoW" => GetMenuItemInfoW(emu),
        "GetMenuItemRect" => GetMenuItemRect(emu),
        "GetMenuState" => GetMenuState(emu),
        "GetMenuStringA" => GetMenuStringA(emu),
        "GetMenuStringW" => GetMenuStringW(emu),
        "GetMessageExtraInfo" => GetMessageExtraInfo(emu),
        "GetMessagePos" => GetMessagePos(emu),
        "GetMessageTime" => GetMessageTime(emu),
        "GetMonitorInfoA" => GetMonitorInfoA(emu),
        "GetMonitorInfoW" => GetMonitorInfoW(emu),
        "GetMouseMovePointsEx" => GetMouseMovePointsEx(emu),
        "GetNextDlgGroupItem" => GetNextDlgGroupItem(emu),
        "GetNextDlgTabItem" => GetNextDlgTabItem(emu),
        "GetOpenClipboardWindow" => GetOpenClipboardWindow(emu),
        "GetParent" => GetParent(emu),
        "GetPointerCursorId" => GetPointerCursorId(emu),
        "GetPointerDevice" => GetPointerDevice(emu),
        "GetPointerDeviceCursors" => GetPointerDeviceCursors(emu),
        "GetPointerDeviceInputSpace" => GetPointerDeviceInputSpace(emu),
        "GetPointerDeviceOrientation" => GetPointerDeviceOrientation(emu),
        "GetPointerDeviceProperties" => GetPointerDeviceProperties(emu),
        "GetPointerDeviceRects" => GetPointerDeviceRects(emu),
        "GetPointerDevices" => GetPointerDevices(emu),
        "GetPointerFrameArrivalTimes" => GetPointerFrameArrivalTimes(emu),
        "GetPointerFrameInfo" => GetPointerFrameInfo(emu),
        "GetPointerFrameInfoHistory" => GetPointerFrameInfoHistory(emu),
        "GetPointerFramePenInfo" => GetPointerFramePenInfo(emu),
        "GetPointerFramePenInfoHistory" => GetPointerFramePenInfoHistory(emu),
        "GetPointerFrameTimes" => GetPointerFrameTimes(emu),
        "GetPointerFrameTouchInfo" => GetPointerFrameTouchInfo(emu),
        "GetPointerFrameTouchInfoHistory" => GetPointerFrameTouchInfoHistory(emu),
        "GetPointerFrameTouchpadInfo" => GetPointerFrameTouchpadInfo(emu),
        "GetPointerFrameTouchpadInfoHistory" => GetPointerFrameTouchpadInfoHistory(emu),
        "GetPointerInfo" => GetPointerInfo(emu),
        "GetPointerInfoHistory" => GetPointerInfoHistory(emu),
        "GetPointerInputTransform" => GetPointerInputTransform(emu),
        "GetPointerPenInfo" => GetPointerPenInfo(emu),
        "GetPointerPenInfoHistory" => GetPointerPenInfoHistory(emu),
        "GetPointerTouchInfo" => GetPointerTouchInfo(emu),
        "GetPointerTouchInfoHistory" => GetPointerTouchInfoHistory(emu),
        "GetPointerTouchpadInfo" => GetPointerTouchpadInfo(emu),
        "GetPointerTouchpadInfoHistory" => GetPointerTouchpadInfoHistory(emu),
        "GetPointerType" => GetPointerType(emu),
        "GetPriorityClipboardFormat" => GetPriorityClipboardFormat(emu),
        "GetProcessDefaultLayout" => GetProcessDefaultLayout(emu),
        "GetProcessDpiAwarenessInternal" => GetProcessDpiAwarenessInternal(emu),
        "GetProcessUIContextInformation" => GetProcessUIContextInformation(emu),
        "GetProgmanWindow" => GetProgmanWindow(emu),
        "GetPropA" => GetPropA(emu),
        "GetPropW" => GetPropW(emu),
        "GetQueueStatus" => GetQueueStatus(emu),
        "GetRawInputBuffer" => GetRawInputBuffer(emu),
        "GetRawInputData" => GetRawInputData(emu),
        "GetRawInputDeviceInfoA" => GetRawInputDeviceInfoA(emu),
        "GetRawInputDeviceInfoW" => GetRawInputDeviceInfoW(emu),
        "GetRawInputDeviceList" => GetRawInputDeviceList(emu),
        "GetRawPointerDeviceData" => GetRawPointerDeviceData(emu),
        "GetReasonTitleFromReasonCode" => GetReasonTitleFromReasonCode(emu),
        "GetRegisteredRawInputDevices" => GetRegisteredRawInputDevices(emu),
        "GetScrollBarInfo" => GetScrollBarInfo(emu),
        "GetScrollInfo" => GetScrollInfo(emu),
        "GetScrollPos" => GetScrollPos(emu),
        "GetScrollRange" => GetScrollRange(emu),
        "GetSendMessageReceiver" => GetSendMessageReceiver(emu),
        "GetShellChangeNotifyWindow" => GetShellChangeNotifyWindow(emu),
        "GetShellWindow" => GetShellWindow(emu),
        "GetSubMenu" => GetSubMenu(emu),
        "GetSysColor" => GetSysColor(emu),
        "GetSysColorBrush" => GetSysColorBrush(emu),
        "GetSystemDpiForProcess" => GetSystemDpiForProcess(emu),
        "GetSystemMenu" => GetSystemMenu(emu),
        "GetSystemMetricsForDpi" => GetSystemMetricsForDpi(emu),
        "GetTabbedTextExtentA" => GetTabbedTextExtentA(emu),
        "GetTabbedTextExtentW" => GetTabbedTextExtentW(emu),
        "GetTaskmanWindow" => GetTaskmanWindow(emu),
        "GetThreadDesktop" => GetThreadDesktop(emu),
        "GetThreadDpiAwarenessContext" => GetThreadDpiAwarenessContext(emu),
        "GetThreadDpiHostingBehavior" => GetThreadDpiHostingBehavior(emu),
        "GetTitleBarInfo" => GetTitleBarInfo(emu),
        "GetTopLevelWindow" => GetTopLevelWindow(emu),
        "GetTopWindow" => GetTopWindow(emu),
        "GetTouchInputInfo" => GetTouchInputInfo(emu),
        "GetUnpredictedMessagePos" => GetUnpredictedMessagePos(emu),
        "GetUpdateRect" => GetUpdateRect(emu),
        "GetUpdateRgn" => GetUpdateRgn(emu),
        "GetUpdatedClipboardFormats" => GetUpdatedClipboardFormats(emu),
        "GetUserObjectInformationA" => GetUserObjectInformationA(emu),
        "GetUserObjectSecurity" => GetUserObjectSecurity(emu),
        "GetWinStationInfo" => GetWinStationInfo(emu),
        "GetWindow" => GetWindow(emu),
        "GetWindowBand" => GetWindowBand(emu),
        "GetWindowCompositionAttribute" => GetWindowCompositionAttribute(emu),
        "GetWindowCompositionInfo" => GetWindowCompositionInfo(emu),
        "GetWindowContextHelpId" => GetWindowContextHelpId(emu),
        "GetWindowDC" => GetWindowDC(emu),
        "GetWindowDisplayAffinity" => GetWindowDisplayAffinity(emu),
        "GetWindowDpiAwarenessContext" => GetWindowDpiAwarenessContext(emu),
        "GetWindowDpiHostingBehavior" => GetWindowDpiHostingBehavior(emu),
        "GetWindowFeedbackSetting" => GetWindowFeedbackSetting(emu),
        "GetWindowInfo" => GetWindowInfo(emu),
        "GetWindowMinimizeRect" => GetWindowMinimizeRect(emu),
        "GetWindowModuleFileName" => GetWindowModuleFileName(emu),
        "GetWindowModuleFileNameA" => GetWindowModuleFileNameA(emu),
        "GetWindowModuleFileNameW" => GetWindowModuleFileNameW(emu),
        "GetWindowPlacement" => GetWindowPlacement(emu),
        "GetWindowProcessHandle" => GetWindowProcessHandle(emu),
        "GetWindowRgn" => GetWindowRgn(emu),
        "GetWindowRgnBox" => GetWindowRgnBox(emu),
        "GetWindowRgnEx" => GetWindowRgnEx(emu),
        "GetWindowTextLengthA" => GetWindowTextLengthA(emu),
        "GetWindowTextLengthW" => GetWindowTextLengthW(emu),
        "GetWindowThreadProcessId" => GetWindowThreadProcessId(emu),
        "GetWindowWord" => GetWindowWord(emu),
        "GhostWindowFromHungWindow" => GhostWindowFromHungWindow(emu),
        "GrayStringA" => GrayStringA(emu),
        "GrayStringW" => GrayStringW(emu),
        "HandleDelegatedInput" => HandleDelegatedInput(emu),
        "HideCaret" => HideCaret(emu),
        "HiliteMenuItem" => HiliteMenuItem(emu),
        "HungWindowFromGhostWindow" => HungWindowFromGhostWindow(emu),
        "IMPGetIMEA" => IMPGetIMEA(emu),
        "IMPGetIMEW" => IMPGetIMEW(emu),
        "IMPQueryIMEA" => IMPQueryIMEA(emu),
        "IMPQueryIMEW" => IMPQueryIMEW(emu),
        "IMPSetIMEA" => IMPSetIMEA(emu),
        "IMPSetIMEW" => IMPSetIMEW(emu),
        "ImpersonateDdeClientWindow" => ImpersonateDdeClientWindow(emu),
        "InSendMessage" => InSendMessage(emu),
        "InSendMessageEx" => InSendMessageEx(emu),
        "InflateRect" => InflateRect(emu),
        "InheritWindowMonitor" => InheritWindowMonitor(emu),
        "InitDManipHook" => InitDManipHook(emu),
        "InitializeGenericHidInjection" => InitializeGenericHidInjection(emu),
        "InitializeInputDeviceInjection" => InitializeInputDeviceInjection(emu),
        "InitializeLpkHooks" => InitializeLpkHooks(emu),
        "InitializePointerDeviceInjection" => InitializePointerDeviceInjection(emu),
        "InitializePointerDeviceInjectionEx" => InitializePointerDeviceInjectionEx(emu),
        "InitializeTouchInjection" => InitializeTouchInjection(emu),
        "InjectDeviceInput" => InjectDeviceInput(emu),
        "InjectGenericHidInput" => InjectGenericHidInput(emu),
        "InjectKeyboardInput" => InjectKeyboardInput(emu),
        "InjectMouseInput" => InjectMouseInput(emu),
        "InjectPointerInput" => InjectPointerInput(emu),
        "InjectSyntheticPointerInput" => InjectSyntheticPointerInput(emu),
        "InjectTouchInput" => InjectTouchInput(emu),
        "InputSpaceRegionFromPoint" => InputSpaceRegionFromPoint(emu),
        "InsertMenuA" => InsertMenuA(emu),
        "InsertMenuItemA" => InsertMenuItemA(emu),
        "InsertMenuItemW" => InsertMenuItemW(emu),
        "InsertMenuW" => InsertMenuW(emu),
        "InternalGetWindowIcon" => InternalGetWindowIcon(emu),
        "InternalGetWindowText" => InternalGetWindowText(emu),
        "IntersectRect" => IntersectRect(emu),
        "InvalidateRgn" => InvalidateRgn(emu),
        "InvertRect" => InvertRect(emu),
        "IsCharAlphaA" => IsCharAlphaA(emu),
        "IsCharAlphaNumericA" => IsCharAlphaNumericA(emu),
        "IsCharAlphaNumericW" => IsCharAlphaNumericW(emu),
        "IsCharAlphaW" => IsCharAlphaW(emu),
        "IsCharLowerA" => IsCharLowerA(emu),
        "IsCharLowerW" => IsCharLowerW(emu),
        "IsCharUpperA" => IsCharUpperA(emu),
        "IsCharUpperW" => IsCharUpperW(emu),
        "IsChild" => IsChild(emu),
        "IsClipboardFormatAvailable" => IsClipboardFormatAvailable(emu),
        "IsDialogMessage" => IsDialogMessage(emu),
        "IsDialogMessageA" => IsDialogMessageA(emu),
        "IsDialogMessageW" => IsDialogMessageW(emu),
        "IsDlgButtonChecked" => IsDlgButtonChecked(emu),
        "IsGUIThread" => IsGUIThread(emu),
        "IsHungAppWindow" => IsHungAppWindow(emu),
        "IsIconic" => IsIconic(emu),
        "IsImmersiveProcess" => IsImmersiveProcess(emu),
        "IsInDesktopWindowBand" => IsInDesktopWindowBand(emu),
        "IsInterceptWindow" => IsInterceptWindow(emu),
        "IsMenu" => IsMenu(emu),
        "IsMouseInPointerEnabled" => IsMouseInPointerEnabled(emu),
        "IsOneCoreTransformMode" => IsOneCoreTransformMode(emu),
        "IsProcessDPIAware" => IsProcessDPIAware(emu),
        "IsRectEmpty" => IsRectEmpty(emu),
        "IsSETEnabled" => IsSETEnabled(emu),
        "IsServerSideWindow" => IsServerSideWindow(emu),
        "IsThreadDesktopComposited" => IsThreadDesktopComposited(emu),
        "IsThreadMessageQueueAttached" => IsThreadMessageQueueAttached(emu),
        "IsThreadTSFEventAware" => IsThreadTSFEventAware(emu),
        "IsTopLevelWindow" => IsTopLevelWindow(emu),
        "IsTouchWindow" => IsTouchWindow(emu),
        "IsValidDpiAwarenessContext" => IsValidDpiAwarenessContext(emu),
        "IsWinEventHookInstalled" => IsWinEventHookInstalled(emu),
        "IsWindowArranged" => IsWindowArranged(emu),
        "IsWindowDisplayChangeSuppressed" => IsWindowDisplayChangeSuppressed(emu),
        "IsWindowEnabled" => IsWindowEnabled(emu),
        "IsWindowInDestroy" => IsWindowInDestroy(emu),
        "IsWindowRedirectedForPrint" => IsWindowRedirectedForPrint(emu),
        "IsWindowUnicode" => IsWindowUnicode(emu),
        "IsWow64Message" => IsWow64Message(emu),
        "IsZoomed" => IsZoomed(emu),
        "KillTimer" => KillTimer(emu),
        "LoadAcceleratorsA" => LoadAcceleratorsA(emu),
        "LoadAcceleratorsW" => LoadAcceleratorsW(emu),
        "LoadBitmapA" => LoadBitmapA(emu),
        "LoadBitmapW" => LoadBitmapW(emu),
        "LoadCursorFromFileA" => LoadCursorFromFileA(emu),
        "LoadCursorFromFileW" => LoadCursorFromFileW(emu),
        "LoadCursorW" => LoadCursorW(emu),
        "LoadIconW" => LoadIconW(emu),
        "LoadImageA" => LoadImageA(emu),
        "LoadImageW" => LoadImageW(emu),
        "LoadKeyboardLayoutA" => LoadKeyboardLayoutA(emu),
        "LoadKeyboardLayoutEx" => LoadKeyboardLayoutEx(emu),
        "LoadKeyboardLayoutW" => LoadKeyboardLayoutW(emu),
        "LoadLocalFonts" => LoadLocalFonts(emu),
        "LoadMenuA" => LoadMenuA(emu),
        "LoadMenuIndirectA" => LoadMenuIndirectA(emu),
        "LoadMenuIndirectW" => LoadMenuIndirectW(emu),
        "LoadMenuW" => LoadMenuW(emu),
        "LoadRemoteFonts" => LoadRemoteFonts(emu),
        "LockSetForegroundWindow" => LockSetForegroundWindow(emu),
        "LockWindowStation" => LockWindowStation(emu),
        "LockWindowUpdate" => LockWindowUpdate(emu),
        "LockWorkStation" => LockWorkStation(emu),
        "LogicalToPhysicalPoint" => LogicalToPhysicalPoint(emu),
        "LogicalToPhysicalPointForPerMonitorDPI" => LogicalToPhysicalPointForPerMonitorDPI(emu),
        "LookupIconIdFromDirectory" => LookupIconIdFromDirectory(emu),
        "LookupIconIdFromDirectoryEx" => LookupIconIdFromDirectoryEx(emu),
        "MBToWCSEx" => MBToWCSEx(emu),
        "MBToWCSExt" => MBToWCSExt(emu),
        "MB_GetString" => MB_GetString(emu),
        "MITGetCursorUpdateHandle" => MITGetCursorUpdateHandle(emu),
        "MITSetLastInputRecipient" => MITSetLastInputRecipient(emu),
        "MITSynthesizeTouchInput" => MITSynthesizeTouchInput(emu),
        "MakeThreadTSFEventAware" => MakeThreadTSFEventAware(emu),
        "MapDialogRect" => MapDialogRect(emu),
        "MapPointsByVisualIdentifier" => MapPointsByVisualIdentifier(emu),
        "MapVirtualKeyExA" | "MapVirtualKeyExW" => MapVirtualKeyExA(emu),
        "MapVisualRelativePoints" => MapVisualRelativePoints(emu),
        "MapWindowPoints" => MapWindowPoints(emu),
        "MenuItemFromPoint" => MenuItemFromPoint(emu),
        "MenuWindowProcA" => MenuWindowProcA(emu),
        "MenuWindowProcW" => MenuWindowProcW(emu),
        "MessageBeep" => MessageBeep(emu),
        "MessageBoxExA" => MessageBoxExA(emu),
        "MessageBoxExW" => MessageBoxExW(emu),
        "MessageBoxIndirectA" => MessageBoxIndirectA(emu),
        "MessageBoxIndirectW" => MessageBoxIndirectW(emu),
        "MessageBoxTimeoutA" => MessageBoxTimeoutA(emu),
        "MessageBoxTimeoutW" => MessageBoxTimeoutW(emu),
        "ModifyMenuA" => ModifyMenuA(emu),
        "ModifyMenuW" => ModifyMenuW(emu),
        "MonitorFromPoint" => MonitorFromPoint(emu),
        "MonitorFromRect" => MonitorFromRect(emu),
        "MonitorFromWindow" => MonitorFromWindow(emu),
        "MoveWindow" => MoveWindow(emu),
        "MsgWaitForMultipleObjects" => MsgWaitForMultipleObjects(emu),
        "MsgWaitForMultipleObjectsEx" => MsgWaitForMultipleObjectsEx(emu),
        "NotifyOverlayWindow" => NotifyOverlayWindow(emu),
        "NotifyWinEvent" => NotifyWinEvent(emu),
        "OemKeyScan" => OemKeyScan(emu),
        "OemToCharA" => OemToCharA(emu),
        "OemToCharBuffA" => OemToCharBuffA(emu),
        "OemToCharBuffW" => OemToCharBuffW(emu),
        "OemToCharW" => OemToCharW(emu),
        "OffsetRect" => OffsetRect(emu),
        "OpenClipboard" => OpenClipboard(emu),
        "OpenDesktopA" => OpenDesktopA(emu),
        "OpenDesktopW" => OpenDesktopW(emu),
        "OpenIcon" => OpenIcon(emu),
        "OpenInputDesktop" => OpenInputDesktop(emu),
        "OpenThreadDesktop" => OpenThreadDesktop(emu),
        "OpenWindowStationA" => OpenWindowStationA(emu),
        "OpenWindowStationW" => OpenWindowStationW(emu),
        "PackDDElParam" => PackDDElParam(emu),
        "PackTouchHitTestingProximityEvaluation" => PackTouchHitTestingProximityEvaluation(emu),
        "PaintDesktop" => PaintDesktop(emu),
        "PaintMenuBar" => PaintMenuBar(emu),
        "PaintMonitor" => PaintMonitor(emu),
        "PhysicalToLogicalPoint" => PhysicalToLogicalPoint(emu),
        "PhysicalToLogicalPointForPerMonitorDPI" => PhysicalToLogicalPointForPerMonitorDPI(emu),
        "PostThreadMessageA" => PostThreadMessageA(emu),
        "PostThreadMessageW" => PostThreadMessageW(emu),
        "PrintWindow" => PrintWindow(emu),
        "PrivateExtractIconExA" => PrivateExtractIconExA(emu),
        "PrivateExtractIconExW" => PrivateExtractIconExW(emu),
        "PrivateExtractIconsA" => PrivateExtractIconsA(emu),
        "PrivateExtractIconsW" => PrivateExtractIconsW(emu),
        "PrivateRegisterICSProc" => PrivateRegisterICSProc(emu),
        "PtInRect" => PtInRect(emu),
        "QueryBSDRWindow" => QueryBSDRWindow(emu),
        "QueryDisplayConfig" => QueryDisplayConfig(emu),
        "QuerySendMessage" => QuerySendMessage(emu),
        "RIMAddInputObserver" => RIMAddInputObserver(emu),
        "RIMAreSiblingDevices" => RIMAreSiblingDevices(emu),
        "RIMDeviceIoControl" => RIMDeviceIoControl(emu),
        "RIMEnableMonitorMappingForDevice" => RIMEnableMonitorMappingForDevice(emu),
        "RIMFreeInputBuffer" => RIMFreeInputBuffer(emu),
        "RIMGetDevicePreparsedData" => RIMGetDevicePreparsedData(emu),
        "RIMGetDevicePreparsedDataLockfree" => RIMGetDevicePreparsedDataLockfree(emu),
        "RIMGetDeviceProperties" => RIMGetDeviceProperties(emu),
        "RIMGetDevicePropertiesLockfree" => RIMGetDevicePropertiesLockfree(emu),
        "RIMGetPhysicalDeviceRect" => RIMGetPhysicalDeviceRect(emu),
        "RIMGetSourceProcessId" => RIMGetSourceProcessId(emu),
        "RIMObserveNextInput" => RIMObserveNextInput(emu),
        "RIMOnAsyncPnpWorkNotification" => RIMOnAsyncPnpWorkNotification(emu),
        "RIMOnPnpNotification" => RIMOnPnpNotification(emu),
        "RIMOnTimerNotification" => RIMOnTimerNotification(emu),
        "RIMQueryDevicePath" => RIMQueryDevicePath(emu),
        "RIMReadInput" => RIMReadInput(emu),
        "RIMRegisterForInput" => RIMRegisterForInput(emu),
        "RIMRegisterForInputEx" => RIMRegisterForInputEx(emu),
        "RIMRemoveInputObserver" => RIMRemoveInputObserver(emu),
        "RIMSetExtendedDeviceProperty" => RIMSetExtendedDeviceProperty(emu),
        "RIMSetTestModeStatus" => RIMSetTestModeStatus(emu),
        "RIMUnregisterForInput" => RIMUnregisterForInput(emu),
        "RIMUpdateInputObserverRegistration" => RIMUpdateInputObserverRegistration(emu),
        "RaiseLowerShellWindow" => RaiseLowerShellWindow(emu),
        "RealChildWindowFromPoint" => RealChildWindowFromPoint(emu),
        "RealGetWindowClass" => RealGetWindowClass(emu),
        "RealGetWindowClassA" => RealGetWindowClassA(emu),
        "RealGetWindowClassW" => RealGetWindowClassW(emu),
        "ReasonCodeNeedsBugID" => ReasonCodeNeedsBugID(emu),
        "ReasonCodeNeedsComment" => ReasonCodeNeedsComment(emu),
        "RecordShutdownReason" => RecordShutdownReason(emu),
        "RedrawWindow" => RedrawWindow(emu),
        "RegisterBSDRWindow" => RegisterBSDRWindow(emu),
        "RegisterClipboardFormatA" => RegisterClipboardFormatA(emu),
        "RegisterClipboardFormatW" => RegisterClipboardFormatW(emu),
        "RegisterCloakedNotification" => RegisterCloakedNotification(emu),
        "RegisterDManipHook" => RegisterDManipHook(emu),
        "RegisterDeviceNotificationA" => RegisterDeviceNotificationA(emu),
        "RegisterDeviceNotificationW" => RegisterDeviceNotificationW(emu),
        "RegisterErrorReportingDialog" => RegisterErrorReportingDialog(emu),
        "RegisterForCustomDockTargets" => RegisterForCustomDockTargets(emu),
        "RegisterForTooltipDismissNotification" => RegisterForTooltipDismissNotification(emu),
        "RegisterFrostWindow" => RegisterFrostWindow(emu),
        "RegisterGhostWindow" => RegisterGhostWindow(emu),
        "RegisterHotKey" => RegisterHotKey(emu),
        "RegisterLogonProcess" => RegisterLogonProcess(emu),
        "RegisterMessagePumpHook" => RegisterMessagePumpHook(emu),
        "RegisterPointerDeviceNotifications" => RegisterPointerDeviceNotifications(emu),
        "RegisterPointerInputTarget" => RegisterPointerInputTarget(emu),
        "RegisterPointerInputTargetEx" => RegisterPointerInputTargetEx(emu),
        "RegisterPowerSettingNotification" => RegisterPowerSettingNotification(emu),
        "RegisterRawInputDevices" => RegisterRawInputDevices(emu),
        "RegisterServicesProcess" => RegisterServicesProcess(emu),
        "RegisterSessionPort" => RegisterSessionPort(emu),
        "RegisterShellHookWindow" => RegisterShellHookWindow(emu),
        "RegisterSuspendResumeNotification" => RegisterSuspendResumeNotification(emu),
        "RegisterSystemThread" => RegisterSystemThread(emu),
        "RegisterTasklist" => RegisterTasklist(emu),
        "RegisterTouchHitTestingWindow" => RegisterTouchHitTestingWindow(emu),
        "RegisterTouchWindow" => RegisterTouchWindow(emu),
        "RegisterTouchpadCapableThread" => RegisterTouchpadCapableThread(emu),
        "RegisterTouchpadCapableWindow" => RegisterTouchpadCapableWindow(emu),
        "RegisterUserApiHook" => RegisterUserApiHook(emu),
        "RegisterWindowMessageA" => RegisterWindowMessageA(emu),
        "RegisterWindowMessageW" => RegisterWindowMessageW(emu),
        "ReleaseCapture" => ReleaseCapture(emu),
        "ReleaseDwmHitTestWaiters" => ReleaseDwmHitTestWaiters(emu),
        "RemoveClipboardFormatListener" => RemoveClipboardFormatListener(emu),
        "RemoveInjectionDevice" => RemoveInjectionDevice(emu),
        "RemoveMenu" => RemoveMenu(emu),
        "RemovePropA" => RemovePropA(emu),
        "RemovePropW" => RemovePropW(emu),
        "RemoveThreadTSFEventAwareness" => RemoveThreadTSFEventAwareness(emu),
        "RemoveVisualIdentifier" => RemoveVisualIdentifier(emu),
        "ReplyMessage" => ReplyMessage(emu),
        "ReportInertia" => ReportInertia(emu),
        "ResolveDesktopForWOW" => ResolveDesktopForWOW(emu),
        "ReuseDDElParam" => ReuseDDElParam(emu),
        "ScreenToClient" => ScreenToClient(emu),
        "ScrollChildren" => ScrollChildren(emu),
        "ScrollDC" => ScrollDC(emu),
        "ScrollWindow" => ScrollWindow(emu),
        "ScrollWindowEx" => ScrollWindowEx(emu),
        "SendDlgItemMessageA" => SendDlgItemMessageA(emu),
        "SendDlgItemMessageW" => SendDlgItemMessageW(emu),
        "SendIMEMessageExA" => SendIMEMessageExA(emu),
        "SendIMEMessageExW" => SendIMEMessageExW(emu),
        "SendInput" => SendInput(emu),
        "SendMessageCallbackA" => SendMessageCallbackA(emu),
        "SendMessageCallbackW" => SendMessageCallbackW(emu),
        "SendMessageTimeoutA" => SendMessageTimeoutA(emu),
        "SendMessageTimeoutW" => SendMessageTimeoutW(emu),
        "SendNotifyMessageA" => SendNotifyMessageA(emu),
        "SendNotifyMessageW" => SendNotifyMessageW(emu),
        "SetActiveWindow" => SetActiveWindow(emu),
        "SetAdditionalForegroundBoostProcesses" => SetAdditionalForegroundBoostProcesses(emu),
        "SetCapture" => SetCapture(emu),
        "SetCaretBlinkTime" => SetCaretBlinkTime(emu),
        "SetCaretPos" => SetCaretPos(emu),
        "SetClassLongA" => SetClassLongA(emu),
        "SetClassLongPtrA" => SetClassLongPtrA(emu),
        "SetClassLongPtrW" => SetClassLongPtrW(emu),
        "SetClassLongW" => SetClassLongW(emu),
        "SetClassWord" => SetClassWord(emu),
        "SetClipboardData" => SetClipboardData(emu),
        "SetClipboardViewer" => SetClipboardViewer(emu),
        "SetCoalescableTimer" => SetCoalescableTimer(emu),
        "SetCoreWindow" => SetCoreWindow(emu),
        "SetCoveredWindowStates" => SetCoveredWindowStates(emu),
        "SetCursorContents" => SetCursorContents(emu),
        "SetDebugErrorLevel" => SetDebugErrorLevel(emu),
        "SetDeskWallpaper" => SetDeskWallpaper(emu),
        "SetDesktopColorTransform" => SetDesktopColorTransform(emu),
        "SetDialogControlDpiChangeBehavior" => SetDialogControlDpiChangeBehavior(emu),
        "SetDialogDpiChangeBehavior" => SetDialogDpiChangeBehavior(emu),
        "SetDisplayAutoRotationPreferences" => SetDisplayAutoRotationPreferences(emu),
        "SetDisplayConfig" => SetDisplayConfig(emu),
        "SetDlgItemInt" => SetDlgItemInt(emu),
        "SetDlgItemTextA" => SetDlgItemTextA(emu),
        "SetDlgItemTextW" => SetDlgItemTextW(emu),
        "SetDoubleClickTime" => SetDoubleClickTime(emu),
        "SetFeatureReportResponse" => SetFeatureReportResponse(emu),
        "SetForegroundRedirectionForActivationObject" => {
            SetForegroundRedirectionForActivationObject(emu)
        }
        "SetFullscreenMagnifierOffsetsDWMUpdated" => SetFullscreenMagnifierOffsetsDWMUpdated(emu),
        "SetGestureConfig" => SetGestureConfig(emu),
        "SetInternalWindowPos" => SetInternalWindowPos(emu),
        "SetKeyboardState" => SetKeyboardState(emu),
        "SetLastErrorEx" => SetLastErrorEx(emu),
        "SetLayeredWindowAttributes" => SetLayeredWindowAttributes(emu),
        "SetMagnificationDesktopColorEffect" => SetMagnificationDesktopColorEffect(emu),
        "SetMagnificationDesktopMagnification" => SetMagnificationDesktopMagnification(emu),
        "SetMagnificationDesktopMagnifierOffsetsDWMUpdated" => {
            SetMagnificationDesktopMagnifierOffsetsDWMUpdated(emu)
        }
        "SetMagnificationDesktopSamplingMode" => SetMagnificationDesktopSamplingMode(emu),
        "SetMagnificationLensCtxInformation" => SetMagnificationLensCtxInformation(emu),
        "SetMaxTouchpadSensitivity" => SetMaxTouchpadSensitivity(emu),
        "SetMenu" => SetMenu(emu),
        "SetMenuContextHelpId" => SetMenuContextHelpId(emu),
        "SetMenuDefaultItem" => SetMenuDefaultItem(emu),
        "SetMenuInfo" => SetMenuInfo(emu),
        "SetMenuItemBitmaps" => SetMenuItemBitmaps(emu),
        "SetMenuItemInfoA" => SetMenuItemInfoA(emu),
        "SetMenuItemInfoW" => SetMenuItemInfoW(emu),
        "SetMessageExtraInfo" => SetMessageExtraInfo(emu),
        "SetMessageQueue" => SetMessageQueue(emu),
        "SetMirrorRendering" => SetMirrorRendering(emu),
        "SetParent" => SetParent(emu),
        "SetPhysicalCursorPos" => SetPhysicalCursorPos(emu),
        "SetPointerDeviceInputSpace" => SetPointerDeviceInputSpace(emu),
        "SetProcessDPIAware" => SetProcessDPIAware(emu),
        "SetProcessDefaultLayout" => SetProcessDefaultLayout(emu),
        "SetProcessDpiAwarenessContext" => SetProcessDpiAwarenessContext(emu),
        "SetProcessDpiAwarenessInternal" => SetProcessDpiAwarenessInternal(emu),
        "SetProcessLaunchForegroundPolicy" => SetProcessLaunchForegroundPolicy(emu),
        "SetProcessRestrictionExemption" => SetProcessRestrictionExemption(emu),
        "SetProcessWindowStation" => SetProcessWindowStation(emu),
        "SetProgmanWindow" => SetProgmanWindow(emu),
        "SetPropA" => SetPropA(emu),
        "SetPropW" => SetPropW(emu),
        "SetRect" => SetRect(emu),
        "SetRectEmpty" => SetRectEmpty(emu),
        "SetScrollInfo" => SetScrollInfo(emu),
        "SetScrollPos" => SetScrollPos(emu),
        "SetScrollRange" => SetScrollRange(emu),
        "SetShellChangeNotifyWindow" => SetShellChangeNotifyWindow(emu),
        "SetShellWindow" => SetShellWindow(emu),
        "SetShellWindowEx" => SetShellWindowEx(emu),
        "SetSysColors" => SetSysColors(emu),
        "SetSysColorsTemp" => SetSysColorsTemp(emu),
        "SetSystemCursor" => SetSystemCursor(emu),
        "SetSystemMenu" => SetSystemMenu(emu),
        "SetTaskmanWindow" => SetTaskmanWindow(emu),
        "SetThreadCursorCreationScaling" => SetThreadCursorCreationScaling(emu),
        "SetThreadDesktop" => SetThreadDesktop(emu),
        "SetThreadDpiAwarenessContext" => SetThreadDpiAwarenessContext(emu),
        "SetThreadDpiHostingBehavior" => SetThreadDpiHostingBehavior(emu),
        "SetThreadInputBlocked" => SetThreadInputBlocked(emu),
        "SetTimer" => SetTimer(emu),
        "SetUserObjectCapability" => SetUserObjectCapability(emu),
        "SetUserObjectInformationA" => SetUserObjectInformationA(emu),
        "SetUserObjectInformationW" => SetUserObjectInformationW(emu),
        "SetUserObjectSecurity" => SetUserObjectSecurity(emu),
        "SetWinEventHook" => SetWinEventHook(emu),
        "SetWindowBand" => SetWindowBand(emu),
        "SetWindowCompositionAttribute" => SetWindowCompositionAttribute(emu),
        "SetWindowCompositionTransition" => SetWindowCompositionTransition(emu),
        "SetWindowContextHelpId" => SetWindowContextHelpId(emu),
        "SetWindowDisplayAffinity" => SetWindowDisplayAffinity(emu),
        "SetWindowFeedbackSetting" => SetWindowFeedbackSetting(emu),
        "SetWindowMessageCapability" => SetWindowMessageCapability(emu),
        "SetWindowPlacement" => SetWindowPlacement(emu),
        "SetWindowRgn" => SetWindowRgn(emu),
        "SetWindowRgnEx" => SetWindowRgnEx(emu),
        "SetWindowStationUser" => SetWindowStationUser(emu),
        "SetWindowWord" => SetWindowWord(emu),
        "SetWindowsHookA" => SetWindowsHookA(emu),
        "SetWindowsHookExAW" => SetWindowsHookExAW(emu),
        "SetWindowsHookExW" => SetWindowsHookExW(emu),
        "SetWindowsHookW" => SetWindowsHookW(emu),
        "ShellForegroundBoostProcess" => ShellForegroundBoostProcess(emu),
        "ShellHandwritingDelegateInput" => ShellHandwritingDelegateInput(emu),
        "ShellHandwritingHandleDelegatedInput" => ShellHandwritingHandleDelegatedInput(emu),
        "ShellHandwritingUndelegateInput" => ShellHandwritingUndelegateInput(emu),
        "ShellMigrateWindow" => ShellMigrateWindow(emu),
        "ShellRegisterHotKey" => ShellRegisterHotKey(emu),
        "ShellSetWindowPos" => ShellSetWindowPos(emu),
        "ShowCaret" => ShowCaret(emu),
        "ShowOwnedPopups" => ShowOwnedPopups(emu),
        "ShowScrollBar" => ShowScrollBar(emu),
        "ShowStartGlass" => ShowStartGlass(emu),
        "ShowSystemCursor" => ShowSystemCursor(emu),
        "ShowWindowAsync" => ShowWindowAsync(emu),
        "ShutdownBlockReasonCreate" => ShutdownBlockReasonCreate(emu),
        "ShutdownBlockReasonDestroy" => ShutdownBlockReasonDestroy(emu),
        "ShutdownBlockReasonQuery" => ShutdownBlockReasonQuery(emu),
        "SignalRedirectionStartComplete" => SignalRedirectionStartComplete(emu),
        "SkipPointerFrameMessages" => SkipPointerFrameMessages(emu),
        "SoftModalMessageBox" => SoftModalMessageBox(emu),
        "SoundSentry" => SoundSentry(emu),
        "SubtractRect" => SubtractRect(emu),
        "SuppressWindowDisplayChange" => SuppressWindowDisplayChange(emu),
        "SwapMouseButton" => SwapMouseButton(emu),
        "SwitchDesktop" => SwitchDesktop(emu),
        "SwitchDesktopWithFade" => SwitchDesktopWithFade(emu),
        "SwitchToThisWindow" => SwitchToThisWindow(emu),
        "SystemParametersInfoForDpi" => SystemParametersInfoForDpi(emu),
        "SystemParametersInfoW" => SystemParametersInfoW(emu),
        "TabbedTextOutA" => TabbedTextOutA(emu),
        "TabbedTextOutW" => TabbedTextOutW(emu),
        "TileChildWindows" => TileChildWindows(emu),
        "TileWindows" => TileWindows(emu),
        "ToAscii" => ToAscii(emu),
        "ToAsciiEx" => ToAsciiEx(emu),
        "ToUnicode" => ToUnicode(emu),
        "ToUnicodeEx" => ToUnicodeEx(emu),
        "TrackMouseEvent" => TrackMouseEvent(emu),
        "TrackPopupMenu" => TrackPopupMenu(emu),
        "TrackPopupMenuEx" => TrackPopupMenuEx(emu),
        "TranslateAccelerator" => TranslateAccelerator(emu),
        "TranslateAcceleratorA" => TranslateAcceleratorA(emu),
        "TranslateAcceleratorW" => TranslateAcceleratorW(emu),
        "TranslateMDISysAccel" => TranslateMDISysAccel(emu),
        "TranslateMessageEx" => TranslateMessageEx(emu),
        "UndelegateInput" => UndelegateInput(emu),
        "UnhookWinEvent" => UnhookWinEvent(emu),
        "UnhookWindowsHook" => UnhookWindowsHook(emu),
        "UnhookWindowsHookEx" => UnhookWindowsHookEx(emu),
        "UnionRect" => UnionRect(emu),
        "UnloadKeyboardLayout" => UnloadKeyboardLayout(emu),
        "UnlockWindowStation" => UnlockWindowStation(emu),
        "UnpackDDElParam" => UnpackDDElParam(emu),
        "UnregisterDeviceNotification" => UnregisterDeviceNotification(emu),
        "UnregisterHotKey" => UnregisterHotKey(emu),
        "UnregisterMessagePumpHook" => UnregisterMessagePumpHook(emu),
        "UnregisterPointerInputTarget" => UnregisterPointerInputTarget(emu),
        "UnregisterPointerInputTargetEx" => UnregisterPointerInputTargetEx(emu),
        "UnregisterPowerSettingNotification" => UnregisterPowerSettingNotification(emu),
        "UnregisterSessionPort" => UnregisterSessionPort(emu),
        "UnregisterSuspendResumeNotification" => UnregisterSuspendResumeNotification(emu),
        "UnregisterTouchWindow" => UnregisterTouchWindow(emu),
        "UnregisterUserApiHook" => UnregisterUserApiHook(emu),
        "UpdateDefaultDesktopThumbnail" => UpdateDefaultDesktopThumbnail(emu),
        "UpdateLayeredWindow" => UpdateLayeredWindow(emu),
        "UpdateLayeredWindowIndirect" => UpdateLayeredWindowIndirect(emu),
        "UpdatePerUserSystemParameters" => UpdatePerUserSystemParameters(emu),
        "UpdateWindowInputSinkHints" => UpdateWindowInputSinkHints(emu),
        "User32InitializeImmEntryTable" => User32InitializeImmEntryTable(emu),
        "UserClientDllInitialize" => UserClientDllInitialize(emu),
        "UserHandleGrantAccess" => UserHandleGrantAccess(emu),
        "UserLpkPSMTextOut" => UserLpkPSMTextOut(emu),
        "UserLpkTabbedTextOut" => UserLpkTabbedTextOut(emu),
        "UserRealizePalette" => UserRealizePalette(emu),
        "UserRegisterWowHandlers" => UserRegisterWowHandlers(emu),
        "VRipOutput" => VRipOutput(emu),
        "VTagOutput" => VTagOutput(emu),
        "ValidateRect" => ValidateRect(emu),
        "ValidateRgn" => ValidateRgn(emu),
        "VkKeyScanA" => VkKeyScanA(emu),
        "VkKeyScanExA" => VkKeyScanExA(emu),
        "VkKeyScanExW" => VkKeyScanExW(emu),
        "VkKeyScanW" => VkKeyScanW(emu),
        "WCSToMBEx" => WCSToMBEx(emu),
        "WINNLSEnableIME" => WINNLSEnableIME(emu),
        "WINNLSGetEnableStatus" => WINNLSGetEnableStatus(emu),
        "WINNLSGetIMEHotkey" => WINNLSGetIMEHotkey(emu),
        "WaitForInputIdle" => WaitForInputIdle(emu),
        "WaitForRedirectionStartComplete" => WaitForRedirectionStartComplete(emu),
        "WaitMessage" => WaitMessage(emu),
        "WinHelpA" => WinHelpA(emu),
        "WinHelpW" => WinHelpW(emu),
        "WindowFromDC" => WindowFromDC(emu),
        "WindowFromPhysicalPoint" => WindowFromPhysicalPoint(emu),
        "WindowFromPoint" => WindowFromPoint(emu),
        "_UserTestTokenForInteractive" => _UserTestTokenForInteractive(emu),
        "gSharedInfo" => gSharedInfo(emu),
        "gapfnScSendMessage" => gapfnScSendMessage(emu),
        "keybd_event" => keybd_event(emu),
        "mouse_event" => mouse_event(emu),
        "wvsprintfA" => wvsprintfA(emu),
        "wvsprintfW" => wvsprintfW(emu),
        "Wow64Transition" => Wow64Transition(emu),
        "RegisterClassExA" => RegisterClassExA(emu),
        "RegisterClassExW" => RegisterClassExW(emu),
        "UnregisterClassA" => UnregisterClassA(emu),
        "UnregisterClassW" => UnregisterClassW(emu),
        "EnumWindows" => EnumWindows(emu),
        "FindWindowExA" => FindWindowExA(emu),
        "FindWindowExW" => FindWindowExW(emu),
        "GetWindowLongPtrA" => GetWindowLongPtrA(emu),
        "GetWindowLongPtrW" => GetWindowLongPtrW(emu),
        "SetWindowLongPtrA" => SetWindowLongPtrA(emu),
        "SetWindowLongPtrW" => SetWindowLongPtrW(emu),
        "GetPhysicalCursorPos" => GetPhysicalCursorPos(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}
