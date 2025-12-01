#[cfg(target_vendor = "apple")]
mod metal {
    type NSBundle = *mut u8;
    type MTLRenderPassDescriptor = *mut u8;

    include!("metal_api.rs");
}

#[cfg(target_vendor = "apple")]
pub use metal::*;