Unity Native Plugin API for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP
* Currently only D3D11 is supported
* API is not stable.

## How to use

* Define in Cargo.toml
```cargo
[dependencies]
unity-native-plugin = { version = "*", features = ["d3d11"] }
```
* Use UnityInterface::get_interface, which is equivalent to IUnityInterfaces::GetInterface, to get the interface.
```rust
let intf = unity_native_plugin::interface::UnityInterfaces::get_unity_interfaces()
    .get_interface::<unity_native_plugin::d3d11::UnityGraphicsD3D11>();
```
