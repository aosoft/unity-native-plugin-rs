use crate::define_unity_interface;
use crate::graphics;
use crate::interface::UnityInterface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsD3D12,
    IUnityGraphicsD3D12,
    0xEF4CEC88A45F4C4C_u64,
    0xBD295B6F2A38D9DE_u64
);

pub type ComPtr = *mut std::ffi::c_void;

macro_rules! impl_d3d12_v2 {
    () => {
        pub unsafe fn device(&self) -> ComPtr {
            self.interface().GetDevice.expect("GetDevice")() as ComPtr
        }

        pub unsafe fn frame_fence(&self) -> ComPtr {
            self.interface().GetFrameFence.expect("GetFrameFence")() as ComPtr
        }

        pub fn next_frame_fence_value(&self) -> u64 {
            unsafe {
                self.interface()
                    .GetNextFrameFenceValue
                    .expect("GetNextFrameFenceValue")() as u64
            }
        }
    };
}

macro_rules! impl_d3d12 {
    () => {
        impl_d3d12_v2!();

        pub unsafe fn command_queue(&self) -> ComPtr {
            self.interface().GetCommandQueue.expect("GetCommandQueue")() as ComPtr
        }

        pub fn resource_state(&self, resource: ComPtr) -> Option<i32> {
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

        pub fn set_resource_state(&self, resource: ComPtr, state: i32) {
            unsafe {
                self.interface().SetResourceState.expect("SetResourceState")(
                    resource as *mut ID3D12Resource,
                    state,
                )
            }
        }
    };
}

impl UnityGraphicsD3D12 {
    impl_d3d12!();
}

define_unity_interface!(
    UnityGraphicsD3D12v2,
    IUnityGraphicsD3D12v2,
    0xEC39D2F18446C745_u64,
    0xB1A2626641D6B11F_u64
);

pub type ResourceState = UnityGraphicsD3D12ResourceState;

impl UnityGraphicsD3D12v2 {
    impl_d3d12_v2!();
}

define_unity_interface!(
    UnityGraphicsD3D12v3,
    IUnityGraphicsD3D12v3,
    0x57C3FAFE59E5E843_u64,
    0xBF4F5998474BB600_u64
);

macro_rules! impl_d3d12_v3 {
    () => {
        impl_d3d12_v2!();

        pub fn set_physical_video_memory_control_values(
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
    };
}

pub type PhysicalVideoMemoryControlValues = UnityGraphicsD3D12PhysicalVideoMemoryControlValues;

impl UnityGraphicsD3D12v3 {
    impl_d3d12_v3!();
}

define_unity_interface!(
    UnityGraphicsD3D12v4,
    IUnityGraphicsD3D12v4,
    0x498FFCC13EC94006_u64,
    0xB18F8B0FF67778C8_u64
);

macro_rules! impl_d3d12_v4 {
    () => {
        impl_d3d12_v3!();

        pub unsafe fn command_queue(&self) -> ComPtr {
            self.interface().GetCommandQueue.expect("GetCommandQueue")() as ComPtr
        }
    }
}

impl UnityGraphicsD3D12v4 {
    impl_d3d12_v4!();
}

define_unity_interface!(
    UnityGraphicsD3D12v5,
    IUnityGraphicsD3D12v5,
    0xF5C8D8A37D37BC42_u64,
    0xB02DFE93B5064A27_u64
);

macro_rules! impl_d3d12_v5 {
    () => {
        impl_d3d12_v4!();

        pub unsafe fn texture_from_render_buffer(&self, rb: graphics::RenderBuffer) -> ComPtr {
            self.interface()
                .TextureFromRenderBuffer
                .expect("TextureFromRenderBuffer")(rb) as ComPtr
        }
    }
}

impl UnityGraphicsD3D12v5 {
    impl_d3d12_v5!();
}
