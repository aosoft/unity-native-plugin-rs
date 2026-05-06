Unity Native Plugin API for Rust
=====

[LICENSE (MIT)](LICENSE)

## How to use

* Define in Cargo.toml
```cargo
[dependencies]
unity-native-plugin = { version = "*", features = ["d3d11"] }

# * Support features
#    * d3d11 - IUnityGraphicsD3D11
#    * d3d12 - IUnityGraphicsD3D12
#    * vulkan - IUnityGraphicsVulkan
#    * metal - IUnityGraphicsMetal
#    * profiler - IUnityProfiler / IUnityProfilerCallbacks
```

* Vulkan support has been integrated into `unity-native-plugin`. No separate crate needs to be added to your dependencies.

### Platform-specific features

The graphics features are gated by both a feature flag **and** a target `cfg`:

| Feature | Effective on |
| --- | --- |
| `d3d11`, `d3d12` | Windows (`cfg(windows)`) |
| `metal` | Apple platforms (`cfg(target_vendor = "apple")`) |
| `vulkan` | Cross-platform |
| `profiler` | Cross-platform |

Enabling a feature on a non-matching platform compiles silently as a no-op — the flag is accepted but the corresponding module (`unity_native_plugin::d3d11`, `unity_native_plugin::metal`, etc.) will not be present. This is intentional so that you can write a single `Cargo.toml` such as:

```toml
[dependencies]
unity-native-plugin = { version = "*", features = ["d3d11", "d3d12", "metal", "vulkan"] }
```

without per-platform `[target.'cfg(...)'.dependencies]` blocks. The unused features
have no runtime cost.

* Use a macro in lib.rs to define your entry points. Without this definition, UnityInterfaces cannot be used.
```rust
unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        // called UnityPluginLoad
    }
    fn unity_plugin_unload() {
        //  called UnityPluginUnload
    }
}
```

* Use UnityInterface::interface, which is equivalent to IUnityInterfaces::GetInterface, to get the interface.
```rust
let intf = unity_native_plugin::interface::UnityInterfaces::get()
    .interface::<unity_native_plugin::d3d11::UnityGraphicsD3D11>();
```

## Examples

* [unity-native-plugin-sample](./unity-native-plugin-sample)
* [Native code (Rust) rendering plugin example for Unity](https://github.com/aosoft/unity-native-rendering-plugin-example-rs) - a port of ["C++ Rendering Plugin example for Unity"](https://github.com/Unity-Technologies/NativeRenderingPlugin)
* [Event tracing example for unity](./unity-native-plugin-sample-profiler) - similar to ["TraceEventProfiler from Unity-Technologies"](https://github.com/Unity-Technologies/TraceEventProfiler)

## Migration guide: 0.8 → 0.9

### `Cargo.toml`

* **The `unity-native-plugin-vulkan` crate has been removed.** Vulkan support is now an opt-in feature of `unity-native-plugin` itself.

  ```toml
  # Before (0.8)
  unity-native-plugin        = "0.8"
  unity-native-plugin-vulkan = "0.8"

  # After (0.9)
  unity-native-plugin = { version = "0.9", features = ["vulkan"] }
  ```

* **The `profiler_callbacks` feature has been merged into `profiler`.** Enabling `profiler` now exposes both `IUnityProfiler` and `IUnityProfilerCallbacks`.

  ```toml
  # Before
  features = ["profiler", "profiler_callbacks"]
  # After
  features = ["profiler"]
  ```

### Module paths

* `unity_native_plugin_vulkan::vulkan::*` → `unity_native_plugin::vulkan::*`
* `unity_native_plugin::d3d11::ComPtr` / `unity_native_plugin::d3d12::ComPtr` → `unity_native_plugin::windows::ComPtr`

### Methods are now provided through `IUnityGraphics*` traits

The inherent `impl` blocks on `UnityGraphicsD3D11`, `UnityGraphicsD3D12*`, `UnityGraphicsMetal*` and `UnityGraphicsVulkan*` have been replaced with traits whose names match the underlying `IUnityGraphics*` C interfaces 1:1. To call any method, bring the matching trait into scope:

```rust
use unity_native_plugin::d3d11::IUnityGraphicsD3D11;
use unity_native_plugin::d3d12::{
    IUnityGraphicsD3D12,    // for UnityGraphicsD3D12
    IUnityGraphicsD3D12v2,  // for UnityGraphicsD3D12v2
    IUnityGraphicsD3D12v3,  // ... v3 .. v8 for the corresponding interface versions
};
use unity_native_plugin::metal::{IUnityGraphicsMetalV1, IUnityGraphicsMetalV2};
use unity_native_plugin::vulkan::{IUnityGraphicsVulkan, IUnityGraphicsVulkanV2};
```

Without the corresponding `use`, methods such as `device()`, `command_queue()` or `command_recording_state()` will appear to be missing.

Note: an earlier 0.9 pre-release used a `*Interface` suffix (e.g. `UnityGraphicsD3D11Interface`). That suffix has been dropped before the 0.9 release in favor of the shorter `IUnityGraphics*` names that match Unity's headers exactly.

### Renamed identifiers

* D3D11 typo fixes (call sites must be updated):
  * `texture_from_natvie_texture` → `texture_from_native_texture`
  * `srv_from_natvie_texture` → `srv_from_native_texture`
* `graphics::GfxRenderer::ReservedCFE` → `graphics::GfxRenderer::Nvn2`

### Vulkan: `VulkanInstance::get_instance_proc_addr` return type

The return type changed from a sys-defined enum to the standard `ash` function-pointer type, so the caller now matches on `Option` instead of the custom `None` variant.

```rust
// Before (0.8)
match instance.get_instance_proc_addr(name) {
    PFN_vkVoidFunction::None => { /* not found */ }
    other => { /* use other */ }
}

// After (0.9)
if let Some(f) = instance.get_instance_proc_addr(name) {
    // use f
}
```
