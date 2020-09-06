use crate::define_unity_interface;
use std::ffi::c_void;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsD3D12,
    IUnityGraphicsD3D12,
    0xEF4CEC88A45F4C4C_u64,
    0xBD295B6F2A38D9DE_u64
);

impl UnityGraphicsD3D12 {
    pub unsafe fn get_device(&self) -> *mut c_void {
        self.get_interface().GetDevice.expect("GetDevice")() as *mut c_void
    }

    pub unsafe fn get_command_queue(&self) -> *mut c_void {
        self.get_interface()
            .GetCommandQueue
            .expect("GetCommandQueue")() as *mut c_void
    }

    pub unsafe fn get_frame_fence(&self) -> *mut c_void {
        self.get_interface().GetFrameFence.expect("GetFrameFence")() as *mut c_void
    }

    pub fn get_next_frame_fence_value(&self) -> u64 {
        unsafe {
            self.get_interface()
                .GetNextFrameFenceValue
                .expect("GetNextFrameFenceValue")()
        }
    }

    pub fn get_resource_state(&self, resource: *mut c_void) -> Option<i32> {
        unsafe {
            let mut ret: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES::default();
            if self
                .get_interface()
                .GetResourceState
                .expect("GetResourceState")(
                resource as *mut ID3D12Resource,
                &mut ret as *mut D3D12_RESOURCE_STATES,
            ) {
                Some(ret)
            } else {
                None
            }
        }
    }

    pub fn set_resource_state(&self, resource: *mut c_void, state: i32) {
        unsafe {
            self.get_interface()
                .SetResourceState
                .expect("SetResourceState")(resource as *mut ID3D12Resource, state)
        }
    }
}

define_unity_interface!(
    UnityGraphicsD3D12v2,
    IUnityGraphicsD3D12v2,
    0xEC39D2F18446C745_u64,
    0xB1A2626641D6B11F_u64
);

pub type ResourceState = UnityGraphicsD3D12ResourceState;

impl UnityGraphicsD3D12v2 {
    pub unsafe fn get_device(&self) -> *mut c_void {
        self.get_interface().GetDevice.expect("GetDevice")() as *mut c_void
    }

    pub unsafe fn get_frame_fence(&self) -> *mut c_void {
        self.get_interface().GetFrameFence.expect("GetFrameFence")() as *mut c_void
    }

    pub fn get_next_frame_fence_value(&self) -> u64 {
        unsafe {
            self.get_interface()
                .GetNextFrameFenceValue
                .expect("GetNextFrameFenceValue")()
        }
    }

    pub unsafe fn execute_command_list(
        &self,
        command_list: *mut c_void,
        states: &[ResourceState],
    ) -> u64 {
        self.get_interface()
            .ExecuteCommandList
            .expect("ExecuteCommandList")(
            command_list as *mut ID3D12GraphicsCommandList,
            states.len() as ::std::os::raw::c_int,
            states.as_ptr() as *mut UnityGraphicsD3D12ResourceState,
        )
    }
}

define_unity_interface!(
    UnityGraphicsD3D12v3,
    IUnityGraphicsD3D12v3,
    0x57C3FAFE59E5E843_u64,
    0xBF4F5998474BB600_u64
);

pub type PhysicalVideoMemoryControlValues = UnityGraphicsD3D12PhysicalVideoMemoryControlValues;

impl UnityGraphicsD3D12v3 {
    pub unsafe fn get_device(&self) -> *mut c_void {
        self.get_interface().GetDevice.expect("GetDevice")() as *mut c_void
    }

    pub unsafe fn get_frame_fence(&self) -> *mut c_void {
        self.get_interface().GetFrameFence.expect("GetFrameFence")() as *mut c_void
    }

    pub fn get_next_frame_fence_value(&self) -> u64 {
        unsafe {
            self.get_interface()
                .GetNextFrameFenceValue
                .expect("GetNextFrameFenceValue")()
        }
    }

    pub unsafe fn execute_command_list(
        &self,
        command_list: *mut c_void,
        states: &[ResourceState],
    ) -> u64 {
        self.get_interface()
            .ExecuteCommandList
            .expect("ExecuteCommandList")(
            command_list as *mut ID3D12GraphicsCommandList,
            states.len() as ::std::os::raw::c_int,
            states.as_ptr() as *mut UnityGraphicsD3D12ResourceState,
        )
    }

    pub fn set_physical_video_memory_control_values(
        &self,
        mem_info: &PhysicalVideoMemoryControlValues,
    ) {
        unsafe {
            self.get_interface()
                .SetPhysicalVideoMemoryControlValues
                .expect("SetPhysicalVideoMemoryControlValues")(
                mem_info as *const UnityGraphicsD3D12PhysicalVideoMemoryControlValues,
            )
        }
    }
}

define_unity_interface!(
    UnityGraphicsD3D12v4,
    IUnityGraphicsD3D12v4,
    0x498FFCC13EC94006_u64,
    0xB18F8B0FF67778C8_u64
);


impl UnityGraphicsD3D12v4 {
    pub unsafe fn get_device(&self) -> *mut c_void {
        self.get_interface().GetDevice.expect("GetDevice")() as *mut c_void
    }

    pub unsafe fn get_frame_fence(&self) -> *mut c_void {
        self.get_interface().GetFrameFence.expect("GetFrameFence")() as *mut c_void
    }

    pub fn get_next_frame_fence_value(&self) -> u64 {
        unsafe {
            self.get_interface()
                .GetNextFrameFenceValue
                .expect("GetNextFrameFenceValue")()
        }
    }

    pub unsafe fn execute_command_list(
        &self,
        command_list: *mut c_void,
        states: &[ResourceState],
    ) -> u64 {
        self.get_interface()
            .ExecuteCommandList
            .expect("ExecuteCommandList")(
            command_list as *mut ID3D12GraphicsCommandList,
            states.len() as ::std::os::raw::c_int,
            states.as_ptr() as *mut UnityGraphicsD3D12ResourceState,
        )
    }

    pub fn set_physical_video_memory_control_values(
        &self,
        mem_info: &PhysicalVideoMemoryControlValues,
    ) {
        unsafe {
            self.get_interface()
                .SetPhysicalVideoMemoryControlValues
                .expect("SetPhysicalVideoMemoryControlValues")(
                mem_info as *const UnityGraphicsD3D12PhysicalVideoMemoryControlValues,
            )
        }
    }

    pub unsafe fn get_command_queue(&self) -> *mut c_void {
        self.get_interface()
            .GetCommandQueue
            .expect("GetCommandQueue")() as *mut c_void
    }
}

define_unity_interface!(
    UnityGraphicsD3D12v5,
    IUnityGraphicsD3D12v5,
    0xF5C8D8A37D37BC42_u64,
    0xB02DFE93B5064A27_u64
);
