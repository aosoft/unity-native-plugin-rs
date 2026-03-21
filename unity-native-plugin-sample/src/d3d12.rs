use winapi::Interface;
use winapi::um::d3d12::*;
use wio::com::ComPtr;

pub fn fill_texture(unity_texture: *mut std::ffi::c_void, x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        if unity_texture.is_null() {
            return;
        }

        let interfaces = unity_native_plugin::interface::UnityInterfaces::get();

        // device / set_resource_state は v1 (UnityGraphicsD3D12) から取得
        let intf_v1 = match interfaces.interface::<unity_native_plugin::d3d12::UnityGraphicsD3D12>()
        {
            Some(i) => i,
            None => return,
        };

        // command_recording_state は v7 → v6 の順に試みる
        let cmd_state = if let Some(i) =
            interfaces.interface::<unity_native_plugin::d3d12::UnityGraphicsD3D12v7>()
        {
            i.command_recording_state()
        } else if let Some(i) =
            interfaces.interface::<unity_native_plugin::d3d12::UnityGraphicsD3D12v6>()
        {
            i.command_recording_state()
        } else {
            return;
        };

        let device = intf_v1.device() as *mut ID3D12Device;
        let resource = unity_texture as *mut ID3D12Resource;

        // CPU 専用 RTV DescriptorHeap を作成
        let heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: 1,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        let mut heap: *mut ID3D12DescriptorHeap = std::ptr::null_mut();
        if (*device).CreateDescriptorHeap(
            &heap_desc,
            &ID3D12DescriptorHeap::uuidof(),
            &mut heap as *mut _ as *mut _,
        ) < 0
        {
            return;
        }
        let heap = ComPtr::from_raw(heap);
        let cpu_handle = (*heap.as_raw()).GetCPUDescriptorHandleForHeapStart();

        // RTV 作成（デフォルトビュー、desc = null）
        (*device).CreateRenderTargetView(resource, std::ptr::null(), cpu_handle);

        // リソース状態を RENDER_TARGET に設定（Unity がバリアを挿入）
        intf_v1.set_resource_state(unity_texture, D3D12_RESOURCE_STATE_RENDER_TARGET as i32);

        // 現在の Unity コマンドリストに記録
        if let Some(cmd_list_ptr) = cmd_state {
            let cmd_list = cmd_list_ptr as *mut ID3D12GraphicsCommandList;
            (*cmd_list).ClearRenderTargetView(cpu_handle, &[x, y, z, w], 0, std::ptr::null());
        }

        // リソース状態を PRESENT に戻す
        intf_v1.set_resource_state(unity_texture, D3D12_RESOURCE_STATE_PRESENT as i32);
    }
}
