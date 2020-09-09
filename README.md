Unity Native Plugin API for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP
* Currently supports D3D11 and D3D12
* API is not stable.

## How to use

* Define in Cargo.toml
```cargo
[dependencies]
unity-native-plugin = { version = "*", features = ["d3d11"] }
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
