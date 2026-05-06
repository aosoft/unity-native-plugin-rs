use crate::bitflag;
use crate::graphics;
use crate::interface::UnityInterface;
use crate::windows::ComPtr;
use unity_native_plugin_sys::*;

pub type ResourceState = UnityGraphicsD3D12ResourceState;
pub type PhysicalVideoMemoryControlValues = UnityGraphicsD3D12PhysicalVideoMemoryControlValues;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GraphicsQueueAccess {
    DontCare = UnityD3D12GraphicsQueueAccess_kUnityD3D12GraphicsQueueAccess_DontCare,
    Allow = UnityD3D12GraphicsQueueAccess_kUnityD3D12GraphicsQueueAccess_Allow,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EventConfigFlagBit {
    EnsurePreviousFrameSubmission =
        UnityD3D12EventConfigFlagBits_kUnityD3D12EventConfigFlag_EnsurePreviousFrameSubmission,
    FlushCommandBuffers =
        UnityD3D12EventConfigFlagBits_kUnityD3D12EventConfigFlag_FlushCommandBuffers,
    SyncWorkerThreads = UnityD3D12EventConfigFlagBits_kUnityD3D12EventConfigFlag_SyncWorkerThreads,
    ModifiesCommandBuffersState =
        UnityD3D12EventConfigFlagBits_kUnityD3D12EventConfigFlag_ModifiesCommandBuffersState,
}

bitflag!(EventConfigFlagBits, EventConfigFlagBit, u32);

#[derive(Copy, Clone)]
pub struct PluginEventConfig {
    pub graphics_queue_access: GraphicsQueueAccess,
    pub flags: EventConfigFlagBits,
    pub ensure_active_render_texture_is_bound: bool,
}

pub trait UnityGraphicsD3D12v2Interface {
    unsafe fn device(&self) -> ComPtr;
    unsafe fn frame_fence(&self) -> ComPtr;
    fn next_frame_fence_value(&self) -> u64;
}

pub use UnityGraphicsD3D12v2Interface as IUnityGraphicsD3D12v2;

macro_rules! impl_d3d12_v2 {
    ($intf:ty) => {
        impl UnityGraphicsD3D12v2Interface for $intf {
            unsafe fn device(&self) -> ComPtr {
                unsafe { self.interface().GetDevice.expect("GetDevice")() as ComPtr }
            }

            unsafe fn frame_fence(&self) -> ComPtr {
                unsafe { self.interface().GetFrameFence.expect("GetFrameFence")() as ComPtr }
            }

            fn next_frame_fence_value(&self) -> u64 {
                unsafe {
                    self.interface()
                        .GetNextFrameFenceValue
                        .expect("GetNextFrameFenceValue")() as u64
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v2,
    unity_native_plugin_sys::IUnityGraphicsD3D12v2,
    0xEC39D2F18446C745_u64,
    0xB1A2626641D6B11F_u64
);
impl_d3d12_v2!(UnityGraphicsD3D12v2);

pub trait UnityGraphicsD3D12Interface {
    unsafe fn device(&self) -> ComPtr;
    unsafe fn command_queue(&self) -> ComPtr;
    unsafe fn frame_fence(&self) -> ComPtr;
    fn next_frame_fence_value(&self) -> u64;
    fn resource_state(&self, resource: ComPtr) -> Option<i32>;
    fn set_resource_state(&self, resource: ComPtr, state: i32);
}

pub use UnityGraphicsD3D12Interface as IUnityGraphicsD3D12;

macro_rules! impl_d3d12 {
    ($intf:ty) => {
        impl UnityGraphicsD3D12Interface for $intf {
            unsafe fn device(&self) -> ComPtr {
                unsafe { self.interface().GetDevice.expect("GetDevice")() as ComPtr }
            }

            unsafe fn command_queue(&self) -> ComPtr {
                unsafe { self.interface().GetCommandQueue.expect("GetCommandQueue")() as ComPtr }
            }

            unsafe fn frame_fence(&self) -> ComPtr {
                unsafe { self.interface().GetFrameFence.expect("GetFrameFence")() as ComPtr }
            }

            fn next_frame_fence_value(&self) -> u64 {
                unsafe {
                    self.interface()
                        .GetNextFrameFenceValue
                        .expect("GetNextFrameFenceValue")() as u64
                }
            }

            fn resource_state(&self, resource: ComPtr) -> Option<i32> {
                unsafe {
                    let mut ret: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES::default();
                    if self.interface().GetResourceState.expect("GetResourceState")(
                        resource as *mut ID3D12Resource,
                        &mut ret as *mut D3D12_RESOURCE_STATES,
                    ) {
                        Some(ret)
                    } else {
                        None
                    }
                }
            }

            fn set_resource_state(&self, resource: ComPtr, state: i32) {
                unsafe {
                    self.interface().SetResourceState.expect("SetResourceState")(
                        resource as *mut ID3D12Resource,
                        state,
                    )
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12,
    unity_native_plugin_sys::IUnityGraphicsD3D12,
    0xEF4CEC88A45F4C4C_u64,
    0xBD295B6F2A38D9DE_u64
);
impl_d3d12!(UnityGraphicsD3D12);

pub trait UnityGraphicsD3D12v3Interface: UnityGraphicsD3D12v2Interface {
    fn set_physical_video_memory_control_values(&self, mem_info: &PhysicalVideoMemoryControlValues);
}

pub use UnityGraphicsD3D12v3Interface as IUnityGraphicsD3D12v3;

macro_rules! impl_d3d12_v3 {
    ($intf:ty) => {
        impl_d3d12_v2!($intf);

        impl UnityGraphicsD3D12v3Interface for $intf {
            fn set_physical_video_memory_control_values(
                &self,
                mem_info: &PhysicalVideoMemoryControlValues,
            ) {
                unsafe {
                    self.interface()
                        .SetPhysicalVideoMemoryControlValues
                        .expect("SetPhysicalVideoMemoryControlValues")(
                        mem_info as *const UnityGraphicsD3D12PhysicalVideoMemoryControlValues,
                    )
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v3,
    unity_native_plugin_sys::IUnityGraphicsD3D12v3,
    0x57C3FAFE59E5E843_u64,
    0xBF4F5998474BB600_u64
);
impl_d3d12_v3!(UnityGraphicsD3D12v3);

pub trait UnityGraphicsD3D12v4Interface: UnityGraphicsD3D12v3Interface {
    unsafe fn command_queue(&self) -> ComPtr;
}

pub use UnityGraphicsD3D12v4Interface as IUnityGraphicsD3D12v4;

macro_rules! impl_d3d12_v4 {
    ($intf:ty) => {
        impl_d3d12_v3!($intf);

        impl UnityGraphicsD3D12v4Interface for $intf {
            unsafe fn command_queue(&self) -> ComPtr {
                unsafe { self.interface().GetCommandQueue.expect("GetCommandQueue")() as ComPtr }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v4,
    unity_native_plugin_sys::IUnityGraphicsD3D12v4,
    0x498FFCC13EC94006_u64,
    0xB18F8B0FF67778C8_u64
);
impl_d3d12_v4!(UnityGraphicsD3D12v4);

pub trait UnityGraphicsD3D12v5Interface: UnityGraphicsD3D12v4Interface {
    unsafe fn texture_from_render_buffer(&self, rb: graphics::RenderBuffer) -> ComPtr;
}

pub use UnityGraphicsD3D12v5Interface as IUnityGraphicsD3D12v5;

macro_rules! impl_d3d12_v5 {
    ($intf:ty) => {
        impl_d3d12_v4!($intf);

        impl UnityGraphicsD3D12v5Interface for $intf {
            unsafe fn texture_from_render_buffer(&self, rb: graphics::RenderBuffer) -> ComPtr {
                unsafe {
                    self.interface()
                        .TextureFromRenderBuffer
                        .expect("TextureFromRenderBuffer")(rb) as ComPtr
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v5,
    unity_native_plugin_sys::IUnityGraphicsD3D12v5,
    0xF5C8D8A37D37BC42_u64,
    0xB02DFE93B5064A27_u64
);
impl_d3d12_v5!(UnityGraphicsD3D12v5);

pub trait UnityGraphicsD3D12v6Interface: UnityGraphicsD3D12v5Interface {
    fn configure_event(&self, event_id: i32, plugin_event_config: &PluginEventConfig);
    unsafe fn command_recording_state(&self) -> Option<ComPtr>;
}

pub use UnityGraphicsD3D12v6Interface as IUnityGraphicsD3D12v6;

macro_rules! impl_d3d12_v6 {
    ($intf:ty) => {
        impl_d3d12_v5!($intf);

        impl UnityGraphicsD3D12v6Interface for $intf {
            fn configure_event(&self, event_id: i32, plugin_event_config: &PluginEventConfig) {
                unsafe {
                    let cfg = UnityD3D12PluginEventConfig {
                        graphicsQueueAccess: plugin_event_config.graphics_queue_access
                            as UnityD3D12GraphicsQueueAccess,
                        flags: plugin_event_config.flags.flag,
                        ensureActiveRenderTextureIsBound: plugin_event_config
                            .ensure_active_render_texture_is_bound,
                    };
                    self.interface().ConfigureEvent.expect("ConfigureEvent")(event_id, &cfg)
                }
            }

            unsafe fn command_recording_state(&self) -> Option<ComPtr> {
                unsafe {
                    let mut state: UnityGraphicsD3D12RecordingState = std::mem::zeroed();
                    if self
                        .interface()
                        .CommandRecordingState
                        .expect("CommandRecordingState")(&mut state)
                    {
                        Some(state.commandList as ComPtr)
                    } else {
                        None
                    }
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v6,
    unity_native_plugin_sys::IUnityGraphicsD3D12v6,
    0xA396DCE58CAC4D78_u64,
    0xAFDD9B281F20B840_u64
);
impl_d3d12_v6!(UnityGraphicsD3D12v6);

pub trait UnityGraphicsD3D12v7Interface: UnityGraphicsD3D12v6Interface {
    unsafe fn swap_chain(&self) -> ComPtr;
    fn sync_interval(&self) -> u32;
    fn present_flags(&self) -> u32;
}

pub use UnityGraphicsD3D12v7Interface as IUnityGraphicsD3D12v7;

macro_rules! impl_d3d12_v7 {
    ($intf:ty) => {
        impl_d3d12_v6!($intf);

        impl UnityGraphicsD3D12v7Interface for $intf {
            unsafe fn swap_chain(&self) -> ComPtr {
                unsafe { self.interface().GetSwapChain.expect("GetSwapChain")() as ComPtr }
            }

            fn sync_interval(&self) -> u32 {
                unsafe { self.interface().GetSyncInterval.expect("GetSyncInterval")() }
            }

            fn present_flags(&self) -> u32 {
                unsafe { self.interface().GetPresentFlags.expect("GetPresentFlags")() }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v7,
    unity_native_plugin_sys::IUnityGraphicsD3D12v7,
    0x4624B0DA41B64AAC_u64,
    0x915AABCB9BC3F0D3_u64
);
impl_d3d12_v7!(UnityGraphicsD3D12v7);

pub trait UnityGraphicsD3D12v8Interface: UnityGraphicsD3D12v7Interface {
    fn request_resource_state(&self, resource: ComPtr, state: i32);
    fn notify_resource_state(&self, resource: ComPtr, state: i32, uav_access: bool);
}

pub use UnityGraphicsD3D12v8Interface as IUnityGraphicsD3D12v8;

macro_rules! impl_d3d12_v8 {
    ($intf:ty) => {
        impl_d3d12_v7!($intf);

        impl UnityGraphicsD3D12v8Interface for $intf {
            fn request_resource_state(&self, resource: ComPtr, state: i32) {
                unsafe {
                    self.interface()
                        .RequestResourceState
                        .expect("RequestResourceState")(
                        resource as *mut ID3D12Resource, state
                    )
                }
            }

            fn notify_resource_state(&self, resource: ComPtr, state: i32, uav_access: bool) {
                unsafe {
                    self.interface()
                        .NotifyResourceState
                        .expect("NotifyResourceState")(
                        resource as *mut ID3D12Resource,
                        state,
                        uav_access,
                    )
                }
            }
        }
    };
}

define_unity_interface!(
    UnityGraphicsD3D12v8,
    unity_native_plugin_sys::IUnityGraphicsD3D12v8,
    0x9D303045D00D4CFD_u64,
    0x8FEBB42968B423B6_u64
);
impl_d3d12_v8!(UnityGraphicsD3D12v8);
