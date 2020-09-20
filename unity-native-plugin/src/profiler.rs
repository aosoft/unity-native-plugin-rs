use unity_native_plugin_sys::*;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerCategoryId {
    Render = UnityBuiltinProfilerCategory__kUnityProfilerCategoryRender as u16,
    Scripts = UnityBuiltinProfilerCategory__kUnityProfilerCategoryScripts as u16,
    ManagedJobs = UnityBuiltinProfilerCategory__kUnityProfilerCategoryManagedJobs as u16,
    BurstJobs = UnityBuiltinProfilerCategory__kUnityProfilerCategoryBurstJobs as u16,
    GUI = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGUI as u16,
    Physics = UnityBuiltinProfilerCategory__kUnityProfilerCategoryPhysics as u16,
    Animation = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAnimation as u16,
    AI = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAI as u16,
    Audio = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudio as u16,
    AudioJob = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudioJob as u16,
    AudioUpdateJob = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudioUpdateJob as u16,
    Video = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVideo as u16,
    Particles = UnityBuiltinProfilerCategory__kUnityProfilerCategoryParticles as u16,
    Gi = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGi as u16,
    Network = UnityBuiltinProfilerCategory__kUnityProfilerCategoryNetwork as u16,
    Loading = UnityBuiltinProfilerCategory__kUnityProfilerCategoryLoading as u16,
    Other = UnityBuiltinProfilerCategory__kUnityProfilerCategoryOther as u16,
    GC = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGC as u16,
    VSync = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVSync as u16,
    Overhead = UnityBuiltinProfilerCategory__kUnityProfilerCategoryOverhead as u16,
    PlayerLoop = UnityBuiltinProfilerCategory__kUnityProfilerCategoryPlayerLoop as u16,
    Director = UnityBuiltinProfilerCategory__kUnityProfilerCategoryDirector as u16,
    VR = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVR as u16,
    Allocation = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAllocation as u16,
    Internal = UnityBuiltinProfilerCategory__kUnityProfilerCategoryInternal as u16,
    FileIO = UnityBuiltinProfilerCategory__kUnityProfilerCategoryFileIO as u16,
    UISystemLayout = UnityBuiltinProfilerCategory__kUnityProfilerCategoryUISystemLayout as u16,
    UISystemRender = UnityBuiltinProfilerCategory__kUnityProfilerCategoryUISystemRender as u16,
    VFX = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVFX as u16,
    BuildInterface = UnityBuiltinProfilerCategory__kUnityProfilerCategoryBuildInterface as u16,
    Input = UnityBuiltinProfilerCategory__kUnityProfilerCategoryInput as u16,
    VirtualTexturing = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVirtualTexturing as u16,
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerFlag {
    Default = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagDefault as u16,
    ScriptUser = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptUser as u16,
    ScriptInvoke = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptInvoke as u16,
    ScriptEnterLeave = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptEnterLeave as u16,
    AvailabilityEditor = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityEditor as u16,
    AvailabilityNonDev = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityNonDev as u16,
    Warning = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagWarning as u16,
    Counter = UnityProfilerMarkerFlag__kCounter as u16,
    VerbosityDebug = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityDebug as u16,
    VerbosityInternal = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityInternal as u16,
    VerbosityAdvanced = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityAdvanced as u16,
}

#[derive(Copy, Clone)]
pub struct ProfilerMarkerFlags {
    pub flag: ProfilerMarkerFlag,
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerEventType {
    Begin = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeBegin as u16,
    End = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeEnd as u16,
    Single = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeSingle as u16,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerDataType {
    None = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeNone as u8,
    InstanceId = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInstanceId as u8,
    Int32 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInt32 as u8,
    UInt32 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeUInt32 as u8,
    Int64 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInt64 as u8,
    UInt64 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeUInt64 as u8,
    Float = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeFloat as u8,
    Double = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeDouble as u8,
    String = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeString as u8,
    String16 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeString16 as u8,
    Blob8 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeBlob8 as u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerDataUnit {
    Undefined = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitUndefined as u8,
    TimeNanoseconds = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitTimeNanoseconds as u8,
    Bytes = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitBytes as u8,
    Count = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitCount as u8,
    Percent = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitPercent as u8,
    FrequencyHz = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitFrequencyHz as u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerFlowEventType {
    Begin = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeBegin as u8,
    Next = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeNext as u8,
    End = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeEnd as u8,
}
