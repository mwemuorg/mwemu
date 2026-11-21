use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AppPolicyGetProcessTerminationMethod" => {
            kernelbase::AppPolicyGetProcessTerminationMethod(emu)
        }
        "AcquireStateLock" => kernelbase::AcquireStateLock(emu),
        "AddDependencyToProcessPackageGraph" => kernelbase::AddDependencyToProcessPackageGraph(emu),
        "AddExtensionProgId" => kernelbase::AddExtensionProgId(emu),
        "AddPackageDependency" => kernelbase::AddPackageDependency(emu),
        "AddPackageDependency2" => kernelbase::AddPackageDependency2(emu),
        "AddPackageNameAliasesByPackageDependency" => {
            kernelbase::AddPackageNameAliasesByPackageDependency(emu)
        }
        "AddPackageNameAliasesByPackageFullName" => {
            kernelbase::AddPackageNameAliasesByPackageFullName(emu)
        }
        "AppContainerFreeMemory" => kernelbase::AppContainerFreeMemory(emu),
        "AppContainerLookupDisplayNameMrtReference" => {
            kernelbase::AppContainerLookupDisplayNameMrtReference(emu)
        }
        "AppContainerLookupMoniker" => kernelbase::AppContainerLookupMoniker(emu),
        "AppXCheckPplSupport" => kernelbase::AppXCheckPplSupport(emu),
        "AppXFreeMemory" => kernelbase::AppXFreeMemory(emu),
        "AppXGetApplicationData" => kernelbase::AppXGetApplicationData(emu),
        "AppXGetDevelopmentMode" => kernelbase::AppXGetDevelopmentMode(emu),
        "AppXGetOSMinVersion" => kernelbase::AppXGetOSMinVersion(emu),
        "AppXGetPackageCapabilities" => kernelbase::AppXGetPackageCapabilities(emu),
        "AppXLookupDisplayName" => kernelbase::AppXLookupDisplayName(emu),
        "AppXLookupMoniker" => kernelbase::AppXLookupMoniker(emu),
        "AppXPostSuccessExtension" => kernelbase::AppXPostSuccessExtension(emu),
        "AppXPreCreationExtension" => kernelbase::AppXPreCreationExtension(emu),
        "AppXReleaseAppXContext" => kernelbase::AppXReleaseAppXContext(emu),
        "AppXUpdatePackageCapabilities" => kernelbase::AppXUpdatePackageCapabilities(emu),
        "ApplicationUserModelIdFromProductId" => {
            kernelbase::ApplicationUserModelIdFromProductId(emu)
        }
        "ArmFeatureUsageSubscriberFlushNotification" => {
            kernelbase::ArmFeatureUsageSubscriberFlushNotification(emu)
        }
        "BaseFreeAppCompatDataForProcess" => kernelbase::BaseFreeAppCompatDataForProcess(emu),
        "BaseReadAppCompatDataForProcess" => kernelbase::BaseReadAppCompatDataForProcess(emu),
        "CheckGroupPolicyEnabled" => kernelbase::CheckGroupPolicyEnabled(emu),
        "CheckIfStateChangeNotificationExists" => {
            kernelbase::CheckIfStateChangeNotificationExists(emu)
        }
        "CheckIsMSIXPackage" => kernelbase::CheckIsMSIXPackage(emu),
        "CloseStateAtom" => kernelbase::CloseStateAtom(emu),
        "CloseStateChangeNotification" => kernelbase::CloseStateChangeNotification(emu),
        "CloseStateContainer" => kernelbase::CloseStateContainer(emu),
        "CloseStateLock" => kernelbase::CloseStateLock(emu),
        "CouldMultiUserAppsBehaviorBePossibleForPackage" => {
            kernelbase::CouldMultiUserAppsBehaviorBePossibleForPackage(emu)
        }
        "CreateStateAtom" => kernelbase::CreateStateAtom(emu),
        "CreateStateChangeNotification" => kernelbase::CreateStateChangeNotification(emu),
        "CreateStateContainer" => kernelbase::CreateStateContainer(emu),
        "CreateStateLock" => kernelbase::CreateStateLock(emu),
        "CreateStateSubcontainer" => kernelbase::CreateStateSubcontainer(emu),
        "DeletePackageDependency" => kernelbase::DeletePackageDependency(emu),
        "DeleteStateAtomValue" => kernelbase::DeleteStateAtomValue(emu),
        "DeleteStateContainer" => kernelbase::DeleteStateContainer(emu),
        "DeleteStateContainerValue" => kernelbase::DeleteStateContainerValue(emu),
        "EnterCriticalPolicySectionInternal" => kernelbase::EnterCriticalPolicySectionInternal(emu),
        "EnumerateExtensionNames" => kernelbase::EnumerateExtensionNames(emu),
        "EnumerateStateAtomValues" => kernelbase::EnumerateStateAtomValues(emu),
        "EnumerateStateContainerItems" => kernelbase::EnumerateStateContainerItems(emu),
        "ExtensionProgIdExists" => kernelbase::ExtensionProgIdExists(emu),
        "FindPackageDependency" => kernelbase::FindPackageDependency(emu),
        "ForceSyncFgPolicyInternal" => kernelbase::ForceSyncFgPolicyInternal(emu),
        "FormatApplicationUserModelIdA" => kernelbase::FormatApplicationUserModelIdA(emu),
        "FreeGPOListInternalA" => kernelbase::FreeGPOListInternalA(emu),
        "FreeGPOListInternalW" => kernelbase::FreeGPOListInternalW(emu),
        "GenerateGPNotificationInternal" => kernelbase::GenerateGPNotificationInternal(emu),
        "GetAlternatePackageRoots" => kernelbase::GetAlternatePackageRoots(emu),
        "GetAppDataFolder" => kernelbase::GetAppDataFolder(emu),
        "GetAppModelVersion" => kernelbase::GetAppModelVersion(emu),
        "GetAppliedGPOListInternalA" => kernelbase::GetAppliedGPOListInternalA(emu),
        "GetAppliedGPOListInternalW" => kernelbase::GetAppliedGPOListInternalW(emu),
        "GetCurrentPackageApplicationContext" => {
            kernelbase::GetCurrentPackageApplicationContext(emu)
        }
        "GetCurrentPackageApplicationResourcesContext" => {
            kernelbase::GetCurrentPackageApplicationResourcesContext(emu)
        }
        "GetCurrentPackageContext" => kernelbase::GetCurrentPackageContext(emu),
        "GetCurrentPackageInfo2" => kernelbase::GetCurrentPackageInfo2(emu),
        "GetCurrentPackageInfo3" => kernelbase::GetCurrentPackageInfo3(emu),
        "GetCurrentPackageInfo_PackageNameAliases" => {
            kernelbase::GetCurrentPackageInfo_PackageNameAliases(emu)
        }
        "GetCurrentPackagePath2" => kernelbase::GetCurrentPackagePath2(emu),
        "GetCurrentPackageResourcesContext" => kernelbase::GetCurrentPackageResourcesContext(emu),
        "GetEffectivePackageStatusForUser" => kernelbase::GetEffectivePackageStatusForUser(emu),
        "GetExtensionApplicationUserModelId" => kernelbase::GetExtensionApplicationUserModelId(emu),
        "GetExtensionProgIds" => kernelbase::GetExtensionProgIds(emu),
        "GetExtensionProperty" => kernelbase::GetExtensionProperty(emu),
        "GetExtensionProperty2" => kernelbase::GetExtensionProperty2(emu),
        "GetFeatureToggleConfiguration" => kernelbase::GetFeatureToggleConfiguration(emu),
        "GetFeatureTogglesChangeToken" => kernelbase::GetFeatureTogglesChangeToken(emu),
        "GetGPOListInternalA" => kernelbase::GetGPOListInternalA(emu),
        "GetGPOListInternalW" => kernelbase::GetGPOListInternalW(emu),
        "GetIdForPackageDependencyContext" => kernelbase::GetIdForPackageDependencyContext(emu),
        "GetNextFgPolicyRefreshInfoInternal" => kernelbase::GetNextFgPolicyRefreshInfoInternal(emu),
        "GetPackageApplicationContext" => kernelbase::GetPackageApplicationContext(emu),
        "GetPackageApplicationProperty" => kernelbase::GetPackageApplicationProperty(emu),
        "GetPackageApplicationPropertyString" => {
            kernelbase::GetPackageApplicationPropertyString(emu)
        }
        "GetPackageApplicationResourcesContext" => {
            kernelbase::GetPackageApplicationResourcesContext(emu)
        }
        "GetPackageContext" => kernelbase::GetPackageContext(emu),
        "GetPackageDependencyInformation" => kernelbase::GetPackageDependencyInformation(emu),
        "GetPackageFamilyNameFromFilePath" => kernelbase::GetPackageFamilyNameFromFilePath(emu),
        "GetPackageFamilyNameFromProgId" => kernelbase::GetPackageFamilyNameFromProgId(emu),
        "GetPackageGraphRevisionId" => kernelbase::GetPackageGraphRevisionId(emu),
        "GetPackageInfo2" => kernelbase::GetPackageInfo2(emu),
        "GetPackageInfo3" => kernelbase::GetPackageInfo3(emu),
        "GetPackageNameAliasesByPackageFullName" => {
            kernelbase::GetPackageNameAliasesByPackageFullName(emu)
        }
        "GetPackageOSMaxVersionTested" => kernelbase::GetPackageOSMaxVersionTested(emu),
        "GetPackagePathByFullName2" => kernelbase::GetPackagePathByFullName2(emu),
        "GetPackagePathOnVolume" => kernelbase::GetPackagePathOnVolume(emu),
        "GetPackageProperty" => kernelbase::GetPackageProperty(emu),
        "GetPackagePropertyString" => kernelbase::GetPackagePropertyString(emu),
        "GetPackageResourcesContext" => kernelbase::GetPackageResourcesContext(emu),
        "GetPackageResourcesProperty" => kernelbase::GetPackageResourcesProperty(emu),
        "GetPackageStatus" => kernelbase::GetPackageStatus(emu),
        "GetPackageStatusForUser" => kernelbase::GetPackageStatusForUser(emu),
        "GetPackageTargetPlatformProperty" => kernelbase::GetPackageTargetPlatformProperty(emu),
        "GetPackageVolumeSisPath" => kernelbase::GetPackageVolumeSisPath(emu),
        "GetPackagedDataForFile" => kernelbase::GetPackagedDataForFile(emu),
        "GetPreviousFgPolicyRefreshInfoInternal" => {
            kernelbase::GetPreviousFgPolicyRefreshInfoInternal(emu)
        }
        "GetProcessesUsingPackageDependency" => kernelbase::GetProcessesUsingPackageDependency(emu),
        "GetPublisherCacheFolder" => kernelbase::GetPublisherCacheFolder(emu),
        "GetPublisherRootFolder" => kernelbase::GetPublisherRootFolder(emu),
        "GetResolvedPackageFullNameForPackageDependency" => {
            kernelbase::GetResolvedPackageFullNameForPackageDependency(emu)
        }
        "GetResolvedPackageFullNameForPackageDependency2" => {
            kernelbase::GetResolvedPackageFullNameForPackageDependency2(emu)
        }
        "GetSecureSystemAppDataFolder" => kernelbase::GetSecureSystemAppDataFolder(emu),
        "GetSharedLocalFolder" => kernelbase::GetSharedLocalFolder(emu),
        "GetStagedPackageOrigin" => kernelbase::GetStagedPackageOrigin(emu),
        "GetStagedPackagePathByFullName2" => kernelbase::GetStagedPackagePathByFullName2(emu),
        "GetStateContainerDepth" => kernelbase::GetStateContainerDepth(emu),
        "GetStateRootFolder" => kernelbase::GetStateRootFolder(emu),
        "GetStateRootFolderBase" => kernelbase::GetStateRootFolderBase(emu),
        "GetStateSettingsFolder" => kernelbase::GetStateSettingsFolder(emu),
        "GetStateVersion" => kernelbase::GetStateVersion(emu),
        "GetSystemAppDataFolder" => kernelbase::GetSystemAppDataFolder(emu),
        "GetSystemMetadataPathForPackage" => kernelbase::GetSystemMetadataPathForPackage(emu),
        "GetSystemMetadataPathForPackageFamily" => {
            kernelbase::GetSystemMetadataPathForPackageFamily(emu)
        }
        "GetSystemStateRootFolder" => kernelbase::GetSystemStateRootFolder(emu),
        "HasPolicyForegroundProcessingCompletedInternal" => {
            kernelbase::HasPolicyForegroundProcessingCompletedInternal(emu)
        }
        "IncrementPackageStatusVersion" => kernelbase::IncrementPackageStatusVersion(emu),
        "InvalidateAppModelVersionCache" => kernelbase::InvalidateAppModelVersionCache(emu),
        "IsApiSetImplemented" => kernelbase::IsApiSetImplemented(emu),
        "IsDeveloperModeEnabled" => kernelbase::IsDeveloperModeEnabled(emu),
        "IsDeveloperModePolicyApplied" => kernelbase::IsDeveloperModePolicyApplied(emu),
        "IsOnDemandRegistrationSupportedForExtensionCategory" => {
            kernelbase::IsOnDemandRegistrationSupportedForExtensionCategory(emu)
        }
        "IsSideloadingPolicyApplied" => kernelbase::IsSideloadingPolicyApplied(emu),
        "IsSyncForegroundPolicyRefresh" => kernelbase::IsSyncForegroundPolicyRefresh(emu),
        "LeaveCriticalPolicySectionInternal" => kernelbase::LeaveCriticalPolicySectionInternal(emu),
        "LogStagedFeatureUsage" => kernelbase::LogStagedFeatureUsage(emu),
        "MsixIsSystemPackageByAppUserModelId" => {
            kernelbase::MsixIsSystemPackageByAppUserModelId(emu)
        }
        "MsixIsSystemPackageByPackageFullName" => {
            kernelbase::MsixIsSystemPackageByPackageFullName(emu)
        }
        "NotifyFeatureToggleUsage" => kernelbase::NotifyFeatureToggleUsage(emu),
        "OpenPackageInfoByFullNameForMachine" => {
            kernelbase::OpenPackageInfoByFullNameForMachine(emu)
        }
        "OpenPackageInfoByFullNameForUser" => kernelbase::OpenPackageInfoByFullNameForUser(emu),
        "OpenStateAtom" => kernelbase::OpenStateAtom(emu),
        "OpenStateExplicitForUserSid" => kernelbase::OpenStateExplicitForUserSid(emu),
        "OpenStateExplicitForUserSidString" => kernelbase::OpenStateExplicitForUserSidString(emu),
        "PackageFamilyNameFromFullNameA" => kernelbase::PackageFamilyNameFromFullNameA(emu),
        "PackageFamilyNameFromIdA" => kernelbase::PackageFamilyNameFromIdA(emu),
        "PackageFamilyNameFromProductId" => kernelbase::PackageFamilyNameFromProductId(emu),
        "PackageFullNameFromIdA" => kernelbase::PackageFullNameFromIdA(emu),
        "PackageFullNameFromProductId" => kernelbase::PackageFullNameFromProductId(emu),
        "PackageIdFromFullNameA" => kernelbase::PackageIdFromFullNameA(emu),
        "PackageIdFromProductId" => kernelbase::PackageIdFromProductId(emu),
        "PackageIsEffectiveSupportedUsersMultiple" => {
            kernelbase::PackageIsEffectiveSupportedUsersMultiple(emu)
        }
        "PackageNameAndPublisherIdFromFamilyNameA" => {
            kernelbase::PackageNameAndPublisherIdFromFamilyNameA(emu)
        }
        "PackagePublisherAddUserIfNecessary" => kernelbase::PackagePublisherAddUserIfNecessary(emu),
        "PackagePublisherIdFromPublisher" => kernelbase::PackagePublisherIdFromPublisher(emu),
        "PackageRelativeApplicationIdFromProductId" => {
            kernelbase::PackageRelativeApplicationIdFromProductId(emu)
        }
        "ParseApplicationUserModelIdA" => kernelbase::ParseApplicationUserModelIdA(emu),
        "PathAddExtensionA" => kernelbase::PathAddExtensionA(emu),
        "PathAddExtensionW" => kernelbase::PathAddExtensionW(emu),
        "PathCchAddExtension" => kernelbase::PathCchAddExtension(emu),
        "PathCchFindExtension" => kernelbase::PathCchFindExtension(emu),
        "PathCchRemoveExtension" => kernelbase::PathCchRemoveExtension(emu),
        "PathCchRenameExtension" => kernelbase::PathCchRenameExtension(emu),
        "PathFindExtensionA" => kernelbase::PathFindExtensionA(emu),
        "PathFindExtensionW" => kernelbase::PathFindExtensionW(emu),
        "PathRemoveExtensionA" => kernelbase::PathRemoveExtensionA(emu),
        "PathRemoveExtensionW" => kernelbase::PathRemoveExtensionW(emu),
        "PathRenameExtensionA" => kernelbase::PathRenameExtensionA(emu),
        "PathRenameExtensionW" => kernelbase::PathRenameExtensionW(emu),
        "ProductIdFromPackageFamilyName" => kernelbase::ProductIdFromPackageFamilyName(emu),
        "PsmCreateKey" => kernelbase::PsmCreateKey(emu),
        "PsmCreateKeyWithDynamicId" => kernelbase::PsmCreateKeyWithDynamicId(emu),
        "PsmEqualApplication" => kernelbase::PsmEqualApplication(emu),
        "PsmEqualPackage" => kernelbase::PsmEqualPackage(emu),
        "PsmGetApplicationNameFromKey" => kernelbase::PsmGetApplicationNameFromKey(emu),
        "PsmGetAumidFromKey" => kernelbase::PsmGetAumidFromKey(emu),
        "PsmGetDynamicIdFromKey" => kernelbase::PsmGetDynamicIdFromKey(emu),
        "PsmGetKeyFromProcess" => kernelbase::PsmGetKeyFromProcess(emu),
        "PsmGetKeyFromToken" => kernelbase::PsmGetKeyFromToken(emu),
        "PsmGetPackageFullNameFromKey" => kernelbase::PsmGetPackageFullNameFromKey(emu),
        "PsmIsChildKey" => kernelbase::PsmIsChildKey(emu),
        "PsmIsDynamicKey" => kernelbase::PsmIsDynamicKey(emu),
        "PsmIsValidKey" => kernelbase::PsmIsValidKey(emu),
        "PublishStateChangeNotification" => kernelbase::PublishStateChangeNotification(emu),
        "PublisherFromPackageFullName" => kernelbase::PublisherFromPackageFullName(emu),
        "QueryStateAtomValueInfo" => kernelbase::QueryStateAtomValueInfo(emu),
        "QueryStateContainerCreatedNew" => kernelbase::QueryStateContainerCreatedNew(emu),
        "QueryStateContainerItemInfo" => kernelbase::QueryStateContainerItemInfo(emu),
        "QuirkGetData" => kernelbase::QuirkGetData(emu),
        "QuirkGetData2" => kernelbase::QuirkGetData2(emu),
        "QuirkIsEnabled" => kernelbase::QuirkIsEnabled(emu),
        "QuirkIsEnabled2" => kernelbase::QuirkIsEnabled2(emu),
        "QuirkIsEnabled3" => kernelbase::QuirkIsEnabled3(emu),
        "QuirkIsEnabledForPackage" => kernelbase::QuirkIsEnabledForPackage(emu),
        "QuirkIsEnabledForPackage2" => kernelbase::QuirkIsEnabledForPackage2(emu),
        "QuirkIsEnabledForPackage3" => kernelbase::QuirkIsEnabledForPackage3(emu),
        "QuirkIsEnabledForPackage4" => kernelbase::QuirkIsEnabledForPackage4(emu),
        "QuirkIsEnabledForProcess" => kernelbase::QuirkIsEnabledForProcess(emu),
        "ReadStateAtomValue" => kernelbase::ReadStateAtomValue(emu),
        "ReadStateContainerValue" => kernelbase::ReadStateContainerValue(emu),
        "RecordFeatureUsage2" => kernelbase::RecordFeatureUsage2(emu),
        "RefreshPackageInfo" => kernelbase::RefreshPackageInfo(emu),
        "RefreshPolicyExInternal" => kernelbase::RefreshPolicyExInternal(emu),
        "RefreshPolicyInternal" => kernelbase::RefreshPolicyInternal(emu),
        "RegisterGPNotificationInternal" => kernelbase::RegisterGPNotificationInternal(emu),
        "ReleasePackagedDataForFile" => kernelbase::ReleasePackagedDataForFile(emu),
        "ReleaseStateLock" => kernelbase::ReleaseStateLock(emu),
        "RemoveExtensionProgIds" => kernelbase::RemoveExtensionProgIds(emu),
        "RemovePackageDependency" => kernelbase::RemovePackageDependency(emu),
        "RemovePackageStatus" => kernelbase::RemovePackageStatus(emu),
        "RemovePackageStatusForUser" => kernelbase::RemovePackageStatusForUser(emu),
        "ResetState" => kernelbase::ResetState(emu),
        "SaveAlternatePackageRootPath" => kernelbase::SaveAlternatePackageRootPath(emu),
        "SaveStateRootFolderPath" => kernelbase::SaveStateRootFolderPath(emu),
        "SetExtensionProperty" => kernelbase::SetExtensionProperty(emu),
        "SetIsDeveloperModeEnabled" => kernelbase::SetIsDeveloperModeEnabled(emu),
        "SetStateVersion" => kernelbase::SetStateVersion(emu),
        "SharedLocalIsEnabled" => kernelbase::SharedLocalIsEnabled(emu),
        "SubscribeEdpEnabledStateChange" => kernelbase::SubscribeEdpEnabledStateChange(emu),
        "SubscribeFeatureUsageFlush" => kernelbase::SubscribeFeatureUsageFlush(emu),
        "SubscribeStateChangeNotification" => kernelbase::SubscribeStateChangeNotification(emu),
        "SubscribeWdagEnabledStateChange" => kernelbase::SubscribeWdagEnabledStateChange(emu),
        "TestClose" => kernelbase::TestClose(emu),
        "TestControlReporting" => kernelbase::TestControlReporting(emu),
        "TestCreate" => kernelbase::TestCreate(emu),
        "TestOpen" => kernelbase::TestOpen(emu),
        "TestQueryData" => kernelbase::TestQueryData(emu),
        "TestRecoverResults" => kernelbase::TestRecoverResults(emu),
        "TestReport" => kernelbase::TestReport(emu),
        "TestUnlockData" => kernelbase::TestUnlockData(emu),
        "TryCreatePackageDependency" => kernelbase::TryCreatePackageDependency(emu),
        "TryCreatePackageDependency2" => kernelbase::TryCreatePackageDependency2(emu),
        "UnregisterGPNotificationInternal" => kernelbase::UnregisterGPNotificationInternal(emu),
        "UnregisterStateChangeNotification" => kernelbase::UnregisterStateChangeNotification(emu),
        "UnregisterStateLock" => kernelbase::UnregisterStateLock(emu),
        "UnsubscribeEdpEnabledStateChange" => kernelbase::UnsubscribeEdpEnabledStateChange(emu),
        "UnsubscribeFeatureUsageFlush" => kernelbase::UnsubscribeFeatureUsageFlush(emu),
        "UnsubscribeStateChangeNotification" => kernelbase::UnsubscribeStateChangeNotification(emu),
        "UnsubscribeWdagEnabledStateChange" => kernelbase::UnsubscribeWdagEnabledStateChange(emu),
        "UpdatePackageStatus" => kernelbase::UpdatePackageStatus(emu),
        "UpdatePackageStatusForUser" => kernelbase::UpdatePackageStatusForUser(emu),
        "VerifyApplicationUserModelId" => kernelbase::VerifyApplicationUserModelId(emu),
        "VerifyApplicationUserModelIdA" => kernelbase::VerifyApplicationUserModelIdA(emu),
        "VerifyPackageFamilyName" => kernelbase::VerifyPackageFamilyName(emu),
        "VerifyPackageFamilyNameA" => kernelbase::VerifyPackageFamilyNameA(emu),
        "VerifyPackageFullName" => kernelbase::VerifyPackageFullName(emu),
        "VerifyPackageFullNameA" => kernelbase::VerifyPackageFullNameA(emu),
        "VerifyPackageId" => kernelbase::VerifyPackageId(emu),
        "VerifyPackageIdA" => kernelbase::VerifyPackageIdA(emu),
        "VerifyPackageName" => kernelbase::VerifyPackageName(emu),
        "VerifyPackageNameA" => kernelbase::VerifyPackageNameA(emu),
        "VerifyPackagePublisher" => kernelbase::VerifyPackagePublisher(emu),
        "VerifyPackagePublisherA" => kernelbase::VerifyPackagePublisherA(emu),
        "VerifyPackageRelativeApplicationId" => kernelbase::VerifyPackageRelativeApplicationId(emu),
        "VerifyPackageRelativeApplicationIdA" => {
            kernelbase::VerifyPackageRelativeApplicationIdA(emu)
        }
        "WaitForMachinePolicyForegroundProcessingInternal" => {
            kernelbase::WaitForMachinePolicyForegroundProcessingInternal(emu)
        }
        "WaitForUserPolicyForegroundProcessingInternal" => {
            kernelbase::WaitForUserPolicyForegroundProcessingInternal(emu)
        }
        "WriteStateAtomValue" => kernelbase::WriteStateAtomValue(emu),
        "WriteStateContainerValue" => kernelbase::WriteStateContainerValue(emu),
        "AddScopedPolicyIDAce" => kernelbase::AddScopedPolicyIDAce(emu),
        "AppPolicyGetClrCompat" => kernelbase::AppPolicyGetClrCompat(emu),
        "AppPolicyGetCreateFileAccess" => kernelbase::AppPolicyGetCreateFileAccess(emu),
        "AppPolicyGetLifecycleManagement" => kernelbase::AppPolicyGetLifecycleManagement(emu),
        "AppPolicyGetMediaFoundationCodecLoading" => {
            kernelbase::AppPolicyGetMediaFoundationCodecLoading(emu)
        }
        "AppPolicyGetShowDeveloperDiagnostic" => {
            kernelbase::AppPolicyGetShowDeveloperDiagnostic(emu)
        }
        "AppPolicyGetThreadInitializationType" => {
            kernelbase::AppPolicyGetThreadInitializationType(emu)
        }
        "AppPolicyGetWindowingModel" => kernelbase::AppPolicyGetWindowingModel(emu),
        "AppXGetOSMaxVersionTested" => kernelbase::AppXGetOSMaxVersionTested(emu),
        "BaseCheckAppcompatCache" => kernelbase::BaseCheckAppcompatCache(emu),
        "BaseCheckAppcompatCacheEx" => kernelbase::BaseCheckAppcompatCacheEx(emu),
        "BaseCleanupAppcompatCacheSupport" => kernelbase::BaseCleanupAppcompatCacheSupport(emu),
        "BaseDumpAppcompatCache" => kernelbase::BaseDumpAppcompatCache(emu),
        "BaseFlushAppcompatCache" => kernelbase::BaseFlushAppcompatCache(emu),
        "BaseInitAppcompatCacheSupport" => kernelbase::BaseInitAppcompatCacheSupport(emu),
        "BaseIsAppcompatInfrastructureDisabled" => {
            kernelbase::BaseIsAppcompatInfrastructureDisabled(emu)
        }
        "BaseUpdateAppcompatCache" => kernelbase::BaseUpdateAppcompatCache(emu),
        "CheckAllowDecryptedRemoteDestinationPolicy" => {
            kernelbase::CheckAllowDecryptedRemoteDestinationPolicy(emu)
        }
        "ClosePackageInfo" => kernelbase::ClosePackageInfo(emu),
        "CloseState" => kernelbase::CloseState(emu),
        "CompareFileTime" => kernelbase::CompareFileTime(emu),
        "CompareStringA" => kernelbase::CompareStringA(emu),
        "CompareStringEx" => kernelbase::CompareStringEx(emu),
        "CompareStringOrdinal" => kernelbase::CompareStringOrdinal(emu),
        "CompareStringW" => kernelbase::CompareStringW(emu),
        "FindPackagesByPackageFamily" => kernelbase::FindPackagesByPackageFamily(emu),
        "FormatApplicationUserModelId" => kernelbase::FormatApplicationUserModelId(emu),
        "GetAppContainerNamedObjectPath" => kernelbase::GetAppContainerNamedObjectPath(emu),
        "GetApplicationRecoveryCallback" => kernelbase::GetApplicationRecoveryCallback(emu),
        "GetApplicationRestartSettings" => kernelbase::GetApplicationRestartSettings(emu),
        "GetApplicationUserModelId" => kernelbase::GetApplicationUserModelId(emu),
        "GetCurrentApplicationUserModelId" => kernelbase::GetCurrentApplicationUserModelId(emu),
        "GetCurrentPackageFamilyName" => kernelbase::GetCurrentPackageFamilyName(emu),
        "GetCurrentPackageFullName" => kernelbase::GetCurrentPackageFullName(emu),
        "GetCurrentPackageId" => kernelbase::GetCurrentPackageId(emu),
        "GetCurrentPackageInfo" => kernelbase::GetCurrentPackageInfo(emu),
        "GetCurrentPackagePath" => kernelbase::GetCurrentPackagePath(emu),
        "GetPackageApplicationIds" => kernelbase::GetPackageApplicationIds(emu),
        "GetPackageFamilyName" => kernelbase::GetPackageFamilyName(emu),
        "GetPackageFullName" => kernelbase::GetPackageFullName(emu),
        "GetPackageId" => kernelbase::GetPackageId(emu),
        "GetPackageInfo" => kernelbase::GetPackageInfo(emu),
        "GetPackagePath" => kernelbase::GetPackagePath(emu),
        "GetPackagePathByFullName" => kernelbase::GetPackagePathByFullName(emu),
        "GetPackagesByPackageFamily" => kernelbase::GetPackagesByPackageFamily(emu),
        "GetProcessMitigationPolicy" => kernelbase::GetProcessMitigationPolicy(emu),
        "GetStagedPackagePathByFullName" => kernelbase::GetStagedPackagePathByFullName(emu),
        "GetStateFolder" => kernelbase::GetStateFolder(emu),
        "LoadPackagedLibrary" => kernelbase::LoadPackagedLibrary(emu),
        "NlsCheckPolicy" => kernelbase::NlsCheckPolicy(emu),
        "OpenPackageInfoByFullName" => kernelbase::OpenPackageInfoByFullName(emu),
        "OpenState" => kernelbase::OpenState(emu),
        "OpenStateExplicit" => kernelbase::OpenStateExplicit(emu),
        "PackageFamilyNameFromFullName" => kernelbase::PackageFamilyNameFromFullName(emu),
        "PackageFamilyNameFromId" => kernelbase::PackageFamilyNameFromId(emu),
        "PackageFullNameFromId" => kernelbase::PackageFullNameFromId(emu),
        "PackageIdFromFullName" => kernelbase::PackageIdFromFullName(emu),
        "PackageNameAndPublisherIdFromFamilyName" => {
            kernelbase::PackageNameAndPublisherIdFromFamilyName(emu)
        }
        "ParseApplicationUserModelId" => kernelbase::ParseApplicationUserModelId(emu),
        "QueryProtectedPolicy" => kernelbase::QueryProtectedPolicy(emu),
        "SetProcessMitigationPolicy" => kernelbase::SetProcessMitigationPolicy(emu),
        "SetProtectedPolicy" => kernelbase::SetProtectedPolicy(emu),
        "UnregisterApplicationRestart" => kernelbase::UnregisterApplicationRestart(emu),
        _ => return false,
    }
    true
}
