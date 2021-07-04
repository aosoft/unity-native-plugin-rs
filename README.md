Unity Native Plugin API for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP
* Currently supports D3D11, D3D12, Vulkan
* API is not stable.

## How to use

* Define in Cargo.toml
```cargo
[dependencies]
unity-native-plugin = { version = "*", features = ["d3d11"] }

# * Support features
#    * d3d11 - IUnityGraphicsD3D11
#    * d3d12 - IUnityGraphicsD3D12
#    * profiler - IUnityProfiler
#    * profiler_callbacks - IUnityProfilerCallbacks
#    * profiler_callbacks_v2 - IUnityProfilerCallbacksV2
```

* If you use Vulkan, define "unity-native-plugin-vulkan" in your dependencies.
```cargo
[dependencies]
unity-native-plugin = "*"
unity-native-plugin-vulkan = "*"
```

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

* [Native code (Rust) rendering plugin example for Unity](https://github.com/aosoft/unity-native-rendering-plugin-example-rs) - a port of ["C++ Rendering Plugin example for Unity"](https://github.com/Unity-Technologies/NativeRenderingPlugin)
* [Event tracing example for unity](./unity-native-plugin-sample-profiler) - a port of ["TraceEventProfiler from Unity-Technologies"](https://github.com/Unity-Technologies/TraceEventProfiler)
