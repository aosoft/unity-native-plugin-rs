# unity-native-plugin-sample

A Rust-based Unity native plugin sample that fills a `RenderTexture` (or `Texture`) with a solid color. Supports four Graphics APIs: Direct3D11 / Direct3D12 / Vulkan / Metal.

## Build

```
cargo build -p unity-native-plugin-sample --release
```

Place the resulting shared library (`unity_native_plugin_sample.dll` / `.so` / `.dylib` depending on the host) anywhere under your Unity project's `Assets/` folder, and configure the PluginImporter to enable the matching target platform(s) (and *Editor*, if you want it usable in the Editor).

## Exported API

```c
// Returns a UnityRenderingEventAndData-compatible callback function pointer.
// Returned signature: void (*)(int eventID, void* data)
typedef void (*UnityRenderingEventAndData)(int eventID, void* data);
UnityRenderingEventAndData GetFillTextureCallback();
```

Pass a pointer to a `FillTextureParams` struct via the `data` parameter.

```c
typedef struct {
    void* texture;   // Unity native texture pointer
    float x, y, z, w; // Clear color (RGBA)
} FillTextureParams;
```

> **Note**
> `IUnityGraphicsD3D12v8::CommandRecordingState`, `IUnityGraphicsVulkan::CommandRecordingState`, and `IUnityGraphicsMetal::CurrentCommandBuffer` return a valid command list / command buffer **only while Unity's render thread is executing a Plugin Event callback**. For this reason, this plugin's API is designed to be invoked through `CommandBuffer.IssuePluginEventAndData`. No API for performing the fill directly from the main thread (via `DllImport`) is intentionally provided.
>
> For Vulkan, the plugin internally calls `IUnityGraphicsVulkan::ConfigureEvent` during `UnityPluginLoad` with `graphicsQueueAccess = DontCare`, `renderPassPrecondition = EnsureOutside`, and `flags = EnsurePreviousFrameSubmission | ModifiesCommandBuffersState`. Without these flags Unity does not provide a valid command buffer to the event callback.
>
> For Direct3D12, the plugin requires `IUnityGraphicsD3D12v8` (it falls back to v7 / v6 only for `ConfigureEvent`, but the actual fill uses v8's `RequestResourceState` / `NotifyResourceState` API for resource state transitions). Older interfaces will not perform the transition and the fill will silently do nothing.
>
> For Metal, the plugin uses `IUnityGraphicsMetalV2` (falling back to `V1`) to obtain the current `MTLCommandBuffer`. It calls `EndCurrentCommandEncoder` to close any encoder Unity has open, then encodes a one-shot render pass with `loadAction = Clear` against the Unity-provided `id<MTLTexture>` (the value returned by `Texture.GetNativeTexturePtr()` under Metal).

## C# Usage

A working sample MonoBehaviour is provided at [`csharp/FillTextureTest.cs`](./csharp/FillTextureTest.cs). Drop it on a `GameObject` with a `Renderer`, set Graphics API to D3D11 / D3D12 / Vulkan / Metal, and the attached material's main texture is filled by the plugin every frame.

The command buffer must be dispatched in a way that runs on Unity's render thread. The correct dispatch path depends on the active render pipeline:

- **Built-in Render Pipeline**: use `Graphics.ExecuteCommandBuffer` (or `Camera.AddCommandBuffer`).
- **URP / HDRP (SRP)**: `Graphics.ExecuteCommandBuffer` from `Update` does not flow through the SRP frame. Subscribe to `RenderPipelineManager.beginCameraRendering` and call `ScriptableRenderContext.ExecuteCommandBuffer` instead. The included sample uses this path.

### Notes

- `IssuePluginEventAndData` executes **deferred on the render thread**. The pointer you pass must remain valid until the callback completes, so allocate parameters in a non-GC region such as `Marshal.AllocHGlobal` or `NativeArray<T>(Allocator.Persistent)`.
- The return value of `GetFillTextureCallback()` can be cached after the first call.
- Under Direct3D12, Vulkan, and Metal, any other path (such as calling the plugin directly from the main thread) will not have a valid current command list / command buffer, and the fill will silently do nothing.
- Under URP / HDRP, `Camera.AddCommandBuffer` is ignored. Use `RenderPipelineManager.beginCameraRendering` (or a `ScriptableRendererFeature` for URP) to inject the command buffer.

## References

- Unity Manual — Low-level native plugin interface: https://docs.unity3d.com/Manual/NativePluginInterface.html
- `CommandBuffer.IssuePluginEventAndData`: https://docs.unity3d.com/ScriptReference/Rendering.CommandBuffer.IssuePluginEventAndData.html
