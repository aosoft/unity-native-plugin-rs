Unity Native Plugin API Tester Library
====

This library is for test runs of native plugin code written in Rust without Unity.

## Notice

It should only be used for testing and debugging of native plugins.

## How to use

* Define in Cargo.toml
```cargo
[dev-dependencies]
unity-native-plugin-tester = { git = "https://github.com/aosoft/unity-native-plugin-tester", branch = "main", features = ["d3d11"] }

# * Support features
#    * d3d11 - IUnityGraphicsD3D11
```

* Write the test code in lib.rs.
```rust
#[test]
fn test() {
    let instant = std::time::Instant::now();
    unity_native_plugin_tester::d3d11::test_plugin_d3d11(
        (256, 256),                                                             // (a)
        |_window, _context| {},                                                 // (b)
        |_window, context| {                                                    // (c)
            let n = (instant.elapsed().as_millis() % 1000) as f32 / 1000.0;
            FillTexture(context.back_buffer().as_raw() as _, 0.0, 0.0, n, 1.0);
            unity_native_plugin_tester::window::LoopResult::Continue
        },
        |_, _| {},                                                              // (d)
        unity_plugin_load,                                                      // (e)
        unity_plugin_unload,                                                    // (f)
    )
    .unwrap();
}
```
* usage
    * (a) Size of the client area (back buffer)
    * (b) Initialize function
    * (c) Test code function
    * (d) Finalize function
    * (e) "unity_plugin_load" function (Defined by the unity_native_plugin_entry_point macro)
    * (f) "unity_plugin_unload" function (Defined by the unity_native_plugin_entry_point macro)

* Run the test
```
% cargo test test
```

