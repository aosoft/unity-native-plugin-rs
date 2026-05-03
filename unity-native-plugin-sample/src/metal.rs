use std::ffi::c_void;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{
    MTLClearColor, MTLCommandBuffer, MTLCommandEncoder, MTLLoadAction, MTLRenderPassDescriptor,
    MTLStoreAction, MTLTexture,
};

use unity_native_plugin::interface::UnityInterfaces;
use unity_native_plugin::metal::{
    UnityGraphicsMetalV1, UnityGraphicsMetalV1Interface, UnityGraphicsMetalV2,
};

pub fn fill_texture(unity_texture: *mut c_void, x: f32, y: f32, z: f32, w: f32) {
    if unity_texture.is_null() {
        return;
    }

    unsafe {
        let interfaces = UnityInterfaces::get();

        let cmd_buffer: Retained<ProtocolObject<dyn MTLCommandBuffer>> =
            if let Some(intf) = interfaces.interface::<UnityGraphicsMetalV2>() {
                intf.end_current_command_encoder();
                match intf.current_command_buffer() {
                    Some(cb) => cb,
                    None => return,
                }
            } else if let Some(intf) = interfaces.interface::<UnityGraphicsMetalV1>() {
                intf.end_current_command_encoder();
                match intf.current_command_buffer() {
                    Some(cb) => cb,
                    None => return,
                }
            } else {
                return;
            };

        // Unity returns id<MTLTexture> directly from GetNativeTexturePtr() under Metal.
        let texture: Retained<ProtocolObject<dyn MTLTexture>> =
            match Retained::retain(unity_texture as *mut ProtocolObject<dyn MTLTexture>) {
                Some(t) => t,
                None => return,
            };

        let pass = MTLRenderPassDescriptor::new();
        let attachment = pass.colorAttachments().objectAtIndexedSubscript(0);
        attachment.setTexture(Some(&*texture));
        attachment.setLoadAction(MTLLoadAction::Clear);
        attachment.setStoreAction(MTLStoreAction::Store);
        attachment.setClearColor(MTLClearColor {
            red: x as f64,
            green: y as f64,
            blue: z as f64,
            alpha: w as f64,
        });

        if let Some(encoder) = cmd_buffer.renderCommandEncoderWithDescriptor(&pass) {
            encoder.endEncoding();
        }
    }
}
