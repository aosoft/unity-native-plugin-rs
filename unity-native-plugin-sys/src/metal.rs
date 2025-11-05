
// facades for objc-dependent types
type NSBundle = *mut u8;
type MTLRenderPassDescriptor = *mut u8;

include!("metal_api.rs");