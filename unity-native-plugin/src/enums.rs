use unity_native_plugin_sys::*;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerFlag {
    ScriptEnterLeave = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptEnterLeave as u16,
    AvailabilityEditor = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityEditor as u16,
    AvailabilityNonDev = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityNonDev as u16,
    Warning = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagWarning as u16,
    VerbosityDebug = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityDebug as u16,
    VerbosityInternal = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityInternal as u16,
    VerbosityAdvanced = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityAdvanced as u16,
}

pub struct ProfilerMarkerFlags(u16);


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

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum RenderingExtEventType {
    SetStereoTarget = UnityRenderingExtEventType_kUnityRenderingExtEventSetStereoTarget,
    SetStereoEye = UnityRenderingExtEventType_kUnityRenderingExtEventSetStereoEye,
    StereoRenderingDone = UnityRenderingExtEventType_kUnityRenderingExtEventStereoRenderingDone,
    BeforeDrawCall = UnityRenderingExtEventType_kUnityRenderingExtEventBeforeDrawCall,
    AfterDrawCall = UnityRenderingExtEventType_kUnityRenderingExtEventAfterDrawCall,
    CustomGrab = UnityRenderingExtEventType_kUnityRenderingExtEventCustomGrab,
    CustomBlit = UnityRenderingExtEventType_kUnityRenderingExtEventCustomBlit,
    //UpdateTextureBegin = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureBegin,
    //UpdateTextureEnd = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureEnd,
    UpdateTextureBeginV1 = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureBeginV1,
    UpdateTextureEndV1 = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureEndV1,
    UpdateTextureBeginV2 = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureBeginV2,
    UpdateTextureEndV2 = UnityRenderingExtEventType_kUnityRenderingExtEventUpdateTextureEndV2,
    //Count = UnityRenderingExtEventType_kUnityRenderingExtEventCount,
    UserEventsStart = UnityRenderingExtEventType_kUnityRenderingExtUserEventsStart,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum RenderingExtCustomBlitCommands {
    CustomBlitVRFlush = UnityRenderingExtCustomBlitCommands_kUnityRenderingExtCustomBlitVRFlush,
    //CustomBlitCount = UnityRenderingExtCustomBlitCommands_kUnityRenderingExtCustomBlitCount,
    UserCustomBlitStart = UnityRenderingExtCustomBlitCommands_kUnityRenderingExtUserCustomBlitStart,
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum RenderingExtQueryType {
    OverrideViewport = UnityRenderingExtQueryType_kUnityRenderingExtQueryOverrideViewport,
    OverrideScissor = UnityRenderingExtQueryType_kUnityRenderingExtQueryOverrideScissor,
    OverrideVROcclussionMesh =
        UnityRenderingExtQueryType_kUnityRenderingExtQueryOverrideVROcclussionMesh,
    OverrideVRSinglePass = UnityRenderingExtQueryType_kUnityRenderingExtQueryOverrideVRSinglePass,
    KeepOriginalDoubleWideWidth_DEPRECATED =
        UnityRenderingExtQueryType_kUnityRenderingExtQueryKeepOriginalDoubleWideWidth_DEPRECATED,
    RequestVRFlushCallback =
        UnityRenderingExtQueryType_kUnityRenderingExtQueryRequestVRFlushCallback,
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum RenderingExtTextureFormat {
    None = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatNone,
    //First = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatFirst,
    R8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8_SRGB,
    R8G8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8_SRGB,
    R8G8B8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8_SRGB,
    R8G8B8A8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8A8_SRGB,
    R8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8_UNorm,
    R8G8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8_UNorm,
    R8G8B8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8_UNorm,
    R8G8B8A8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8A8_UNorm,
    R8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8_SNorm,
    R8G8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8_SNorm,
    R8G8B8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8_SNorm,
    R8G8B8A8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8A8_SNorm,
    R8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8_UInt,
    R8G8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8_UInt,
    R8G8B8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8_UInt,
    R8G8B8A8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8A8_UInt,
    R8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8_SInt,
    R8G8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8_SInt,
    R8G8B8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8_SInt,
    R8G8B8A8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR8G8B8A8_SInt,
    R16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16_UNorm,
    R16G16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16_UNorm,
    R16G16B16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16_UNorm,
    R16G16B16A16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16A16_UNorm,
    R16_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16_SNorm,
    R16G16_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16_SNorm,
    R16G16B16_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16_SNorm,
    R16G16B16A16_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16A16_SNorm,
    R16_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16_UInt,
    R16G16_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16_UInt,
    R16G16B16_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16_UInt,
    R16G16B16A16_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16A16_UInt,
    R16_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16_SInt,
    R16G16_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16_SInt,
    R16G16B16_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16_SInt,
    R16G16B16A16_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16A16_SInt,
    R32_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32_UInt,
    R32G32_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32_UInt,
    R32G32B32_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32_UInt,
    R32G32B32A32_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32A32_UInt,
    R32_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32_SInt,
    R32G32_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32_SInt,
    R32G32B32_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32_SInt,
    R32G32B32A32_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32A32_SInt,
    R16_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16_SFloat,
    R16G16_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16_SFloat,
    R16G16B16_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16_SFloat,
    R16G16B16A16_SFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR16G16B16A16_SFloat,
    R32_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32_SFloat,
    R32G32_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32_SFloat,
    R32G32B32_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32_SFloat,
    R32G32B32A32_SFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR32G32B32A32_SFloat,
    L8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatL8_UNorm,
    A8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA8_UNorm,
    A16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA16_UNorm,
    B8G8R8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8_SRGB,
    B8G8R8A8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8A8_SRGB,
    B8G8R8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8_UNorm,
    B8G8R8A8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8A8_UNorm,
    B8G8R8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8_SNorm,
    B8G8R8A8_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8A8_SNorm,
    B8G8R8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8_UInt,
    B8G8R8A8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8A8_UInt,
    B8G8R8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8_SInt,
    B8G8R8A8_SInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB8G8R8A8_SInt,
    R4G4B4A4_UNormPack16 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR4G4B4A4_UNormPack16,
    B4G4R4A4_UNormPack16 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB4G4R4A4_UNormPack16,
    R5G6B5_UNormPack16 = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR5G6B5_UNormPack16,
    B5G6R5_UNormPack16 = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB5G6R5_UNormPack16,
    R5G5B5A1_UNormPack16 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR5G5B5A1_UNormPack16,
    B5G5R5A1_UNormPack16 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB5G5R5A1_UNormPack16,
    A1R5G5B5_UNormPack16 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA1R5G5B5_UNormPack16,
    E5B9G9R9_UFloatPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatE5B9G9R9_UFloatPack32,
    B10G11R11_UFloatPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatB10G11R11_UFloatPack32,
    A2B10G10R10_UNormPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2B10G10R10_UNormPack32,
    A2B10G10R10_UIntPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2B10G10R10_UIntPack32,
    A2B10G10R10_SIntPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2B10G10R10_SIntPack32,
    A2R10G10B10_UNormPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2R10G10B10_UNormPack32,
    A2R10G10B10_UIntPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2R10G10B10_UIntPack32,
    A2R10G10B10_SIntPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2R10G10B10_SIntPack32,
    A2R10G10B10_XRSRGBPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2R10G10B10_XRSRGBPack32,
    A2R10G10B10_XRUNormPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA2R10G10B10_XRUNormPack32,
    R10G10B10_XRSRGBPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR10G10B10_XRSRGBPack32,
    R10G10B10_XRUNormPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR10G10B10_XRUNormPack32,
    A10R10G10B10_XRSRGBPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA10R10G10B10_XRSRGBPack32,
    A10R10G10B10_XRUNormPack32 =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA10R10G10B10_XRUNormPack32,
    A8R8G8B8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA8R8G8B8_SRGB,
    A8R8G8B8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA8R8G8B8_UNorm,
    A32R32G32B32_SFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatA32R32G32B32_SFloat,
    D16_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatD16_UNorm,
    D24_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatD24_UNorm,
    D24_UNorm_S8_UInt = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatD24_UNorm_S8_UInt,
    D32_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatD32_SFloat,
    D32_SFloat_S8_Uint = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatD32_SFloat_S8_Uint,
    S8_Uint = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatS8_Uint,
    RGBA_DXT1_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT1_SRGB,
    RGBA_DXT1_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT1_UNorm,
    RGBA_DXT3_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT3_SRGB,
    RGBA_DXT3_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT3_UNorm,
    RGBA_DXT5_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT5_SRGB,
    RGBA_DXT5_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_DXT5_UNorm,
    R_BC4_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR_BC4_UNorm,
    R_BC4_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR_BC4_SNorm,
    RG_BC5_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRG_BC5_UNorm,
    RG_BC5_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRG_BC5_SNorm,
    RGB_BC6H_UFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_BC6H_UFloat,
    RGB_BC6H_SFloat = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_BC6H_SFloat,
    RGBA_BC7_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_BC7_SRGB,
    RGBA_BC7_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_BC7_UNorm,
    RGB_PVRTC_2Bpp_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_PVRTC_2Bpp_SRGB,
    RGB_PVRTC_2Bpp_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_PVRTC_2Bpp_UNorm,
    RGB_PVRTC_4Bpp_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_PVRTC_4Bpp_SRGB,
    RGB_PVRTC_4Bpp_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_PVRTC_4Bpp_UNorm,
    RGBA_PVRTC_2Bpp_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_PVRTC_2Bpp_SRGB,
    RGBA_PVRTC_2Bpp_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_PVRTC_2Bpp_UNorm,
    RGBA_PVRTC_4Bpp_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_PVRTC_4Bpp_SRGB,
    RGBA_PVRTC_4Bpp_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_PVRTC_4Bpp_UNorm,
    RGB_ETC_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_ETC_UNorm,
    RGB_ETC2_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_ETC2_SRGB,
    RGB_ETC2_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_ETC2_UNorm,
    RGB_A1_ETC2_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_A1_ETC2_SRGB,
    RGB_A1_ETC2_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGB_A1_ETC2_UNorm,
    RGBA_ETC2_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ETC2_SRGB,
    RGBA_ETC2_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ETC2_UNorm,
    R_EAC_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR_EAC_UNorm,
    R_EAC_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatR_EAC_SNorm,
    RG_EAC_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRG_EAC_UNorm,
    RG_EAC_SNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRG_EAC_SNorm,
    RGBA_ASTC4X4_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC4X4_SRGB,
    RGBA_ASTC4X4_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC4X4_UNorm,
    RGBA_ASTC5X5_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC5X5_SRGB,
    RGBA_ASTC5X5_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC5X5_UNorm,
    RGBA_ASTC6X6_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC6X6_SRGB,
    RGBA_ASTC6X6_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC6X6_UNorm,
    RGBA_ASTC8X8_SRGB = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC8X8_SRGB,
    RGBA_ASTC8X8_UNorm = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC8X8_UNorm,
    RGBA_ASTC10X10_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC10X10_SRGB,
    RGBA_ASTC10X10_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC10X10_UNorm,
    RGBA_ASTC12X12_SRGB =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC12X12_SRGB,
    RGBA_ASTC12X12_UNorm =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC12X12_UNorm,
    YUV2 = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatYUV2,
    DepthAuto = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatDepthAuto,
    ShadowAuto = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatShadowAuto,
    VideoAuto = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatVideoAuto,
    RGBA_ASTC4X4_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC4X4_UFloat,
    RGBA_ASTC5X5_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC5X5_UFloat,
    RGBA_ASTC6X6_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC6X6_UFloat,
    RGBA_ASTC8X8_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC8X8_UFloat,
    RGBA_ASTC10X10_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC10X10_UFloat,
    RGBA_ASTC12X12_UFloat =
        UnityRenderingExtTextureFormat_kUnityRenderingExtFormatRGBA_ASTC12X12_UFloat,
    //Last = UnityRenderingExtTextureFormat_kUnityRenderingExtFormatLast,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderCompilerExtCompilerPlatform {
    Unused0 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused0,
    Unused1 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused1,
    Unused2 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused2,
    Unused3 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused3,
    D3D11 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformD3D11,
    GLES20 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformGLES20,
    Unused6 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused6,
    Unused7 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused7,
    Unused8 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused8,
    GLES3Plus = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformGLES3Plus,
    Unused10 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused10,
    PS4 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformPS4,
    XboxOne = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformXboxOne,
    Unused13 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused13,
    Metal = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformMetal,
    OpenGLCore =
        UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformOpenGLCore,
    Unused16 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused16,
    Unused17 = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformUnused17,
    Vulkan = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformVulkan,
    Switch = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformSwitch,
    XboxOneD3D12 =
        UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformXboxOneD3D12,
    //Count = UnityShaderCompilerExtCompilerPlatform_kUnityShaderCompilerExtCompPlatformCount,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderCompilerExtShaderType {
    None = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderNone,
    Vertex = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderVertex,
    Fragment = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderFragment,
    Geometry = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderGeometry,
    Hull = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderHull,
    Domain = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderDomain,
    RayTracing = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderRayTracing,
    //Count = UnityShaderCompilerExtShaderType_kUnityShaderCompilerExtShaderTypeCount,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderCompilerExtGPUProgramType {
    Unknown = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnknown,
    GLLegacy = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLLegacy,
    GLES31AEP =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLES31AEP,
    GLES31 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLES31,
    GLES3 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLES3,
    GLES = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLES,
    GLCore32 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLCore32,
    GLCore41 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLCore41,
    GLCore43 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetGLCore43,
    DX9VertexSM20 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX9VertexSM20,
    DX9VertexSM30 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX9VertexSM30,
    DX9PixelSM20 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX9PixelSM20,
    DX9PixelSM30 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX9PixelSM30,
    DX10Level9Vertex = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX10Level9Vertex,
    DX10Level9Pixel =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX10Level9Pixel,
    DX11VertexSM40 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11VertexSM40,
    DX11VertexSM50 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11VertexSM50,
    DX11PixelSM40 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11PixelSM40,
    DX11PixelSM50 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11PixelSM50,
    DX11GeometrySM40 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11GeometrySM40,
    DX11GeometrySM50 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11GeometrySM50,
    DX11HullSM50 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11HullSM50,
    DX11DomainSM50 =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetDX11DomainSM50,
    MetalVS = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetMetalVS,
    MetalFS = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetMetalFS,
    SPIRV = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetSPIRV,
    Unused1 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnused1,
    Unused2 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnused2,
    Unused3 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnused3,
    Unused4 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnused4,
    Unused5 = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetUnused5,
    RayTracing =
        UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetRayTracing,
    Count = UnityShaderCompilerExtGPUProgramType_kUnityShaderCompilerExtGPUProgramTargetCount,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderCompilerExtGPUProgram {
    VS = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramVS,
    PS = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramPS,
    GS = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramGS,
    HS = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramHS,
    DS = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramDS,
    Custom = UnityShaderCompilerExtGPUProgram_kUnityShaderCompilerExtGPUProgramCustom,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderCompilerExtEventType {
    CreateCustomSourceVariant =
        UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventCreateCustomSourceVariant,
    CreateCustomSourceVariantCleanup = UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventCreateCustomSourceVariantCleanup,
    CreateCustomBinaryVariant =
        UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventCreateCustomBinaryVariant,
    CreateCustomBinaryVariantCleanup = UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventCreateCustomBinaryVariantCleanup,
    PluginConfigure = UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventPluginConfigure,
    //Count = UnityShaderCompilerExtEventType_kUnityShaderCompilerExtEventCount,
    UserEventsStart = UnityShaderCompilerExtEventType_kUnityShaderCompilerExtUserEventsStart,
}
