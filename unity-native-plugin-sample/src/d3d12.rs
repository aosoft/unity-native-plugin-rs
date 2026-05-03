use unity_native_plugin::d3d12::{UnityGraphicsD3D12V2Ext, UnityGraphicsD3D12v6Ext};
use winapi::Interface;
use winapi::um::d3d12::*;
use wio::com::ComPtr;

pub fn fill_texture(unity_texture: *mut std::ffi::c_void, x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        if unity_texture.is_null() {
            return;
        }

        let interfaces = unity_native_plugin::interface::UnityInterfaces::get();

        let intf = match interfaces.interface::<unity_native_plugin::d3d12::UnityGraphicsD3D12v8>()
        {
            Some(i) => i,
            None => return,
        };

        let cmd_list_ptr = match intf.command_recording_state() {
            Some(p) => p,
            None => return,
        };

        let device = intf.device() as *mut ID3D12Device;
        let resource = unity_texture as *mut ID3D12Resource;

        let heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: 1,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        let mut heap: *mut ID3D12DescriptorHeap = std::ptr::null_mut();
        let hr = (*device).CreateDescriptorHeap(
            &heap_desc,
            &ID3D12DescriptorHeap::uuidof(),
            &mut heap as *mut _ as *mut _,
        );
        if hr < 0 {
            return;
        }
        let heap = ComPtr::from_raw(heap);
        let cpu_handle = (*heap.as_raw()).GetCPUDescriptorHandleForHeapStart();

        (*device).CreateRenderTargetView(resource, std::ptr::null(), cpu_handle);

        intf.request_resource_state(unity_texture, D3D12_RESOURCE_STATE_RENDER_TARGET as i32);

        let cmd_list = cmd_list_ptr as *mut ID3D12GraphicsCommandList;
        (*cmd_list).ClearRenderTargetView(cpu_handle, &[x, y, z, w], 0, std::ptr::null());

        intf.notify_resource_state(
            unity_texture,
            D3D12_RESOURCE_STATE_RENDER_TARGET as i32,
            false,
        );
    }
}
