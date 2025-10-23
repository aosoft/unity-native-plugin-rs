use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSBundle;
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLDevice, MTLRenderPassDescriptor,
    MTLTexture,
};
use unity_native_plugin_sys::{IUnityGraphicsMetalV1, IUnityGraphicsMetalV2, UnityRenderBuffer};

use crate::interface::UnityInterface;
use crate::{define_unity_interface, graphics};

pub trait UnityGraphicsMetalV1Interface {
    fn metal_bundle(&self) -> Option<Retained<NSBundle>>;
    fn metal_device(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;
    fn current_command_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;
    fn current_command_encoder(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandEncoder>>>;
    fn end_current_command_encoder(&self);
    fn current_render_pass_descriptor(&self) -> Option<Retained<MTLRenderPassDescriptor>>;
    unsafe fn render_buffer_from_handle(
        &self,
        buffer_handle: *mut ::std::os::raw::c_void,
    ) -> graphics::RenderBuffer;
    unsafe fn texture_from_render_buffer(
        &self,
        buffer: UnityRenderBuffer,
    ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;
    unsafe fn aa_resolved_texture_from_render_buffer(
        &self,
        buffer: UnityRenderBuffer,
    ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;
    unsafe fn stencil_texture_from_render_buffer(
        &self,
        buffer: UnityRenderBuffer,
    ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;
}

macro_rules! impl_metal_v1 {
    ($intf:ty) => {
        impl UnityGraphicsMetalV1Interface for $intf {
            fn metal_bundle(&self) -> Option<Retained<NSBundle>> {
                unsafe {
                    Retained::from_raw(
                        self.interface().MetalBundle.expect("MetalBundle")() as *mut _
                    )
                }
            }

            fn metal_device(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLDevice>>::from_raw(self
                        .interface()
                        .MetalDevice
                        .expect("MetalDevice")(
                    ) as *mut _)
                }
            }

            fn current_command_buffer(
                &self,
            ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLCommandBuffer>>::from_raw(self
                        .interface()
                        .CurrentCommandBuffer
                        .expect("CurrentCommandBuffer")(
                    )
                        as *mut _)
                }
            }

            fn current_command_encoder(
                &self,
            ) -> Option<Retained<ProtocolObject<dyn MTLCommandEncoder>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLCommandEncoder>>::from_raw(self
                        .interface()
                        .CurrentCommandEncoder
                        .expect("CurrentCommandEncoder")(
                    )
                        as *mut _)
                }
            }

            fn end_current_command_encoder(&self) {
                unsafe {
                    self.interface()
                        .EndCurrentCommandEncoder
                        .expect("EndCurrentCommandEncoder")()
                }
            }

            fn current_render_pass_descriptor(&self) -> Option<Retained<MTLRenderPassDescriptor>> {
                unsafe {
                    Retained::from_raw(self
                        .interface()
                        .CurrentRenderPassDescriptor
                        .expect("CurrentRenderPassDescriptor")()
                        as *mut _)
                }
            }

            unsafe fn render_buffer_from_handle(
                &self,
                buffer_handle: *mut ::std::os::raw::c_void,
            ) -> graphics::RenderBuffer {
                unsafe {
                    (self
                        .interface()
                        .RenderBufferFromHandle
                        .expect("RenderBufferFromHandle"))(buffer_handle)
                }
            }

            unsafe fn texture_from_render_buffer(
                &self,
                buffer: graphics::RenderBuffer,
            ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLTexture>>::from_raw(self
                        .interface()
                        .TextureFromRenderBuffer
                        .expect("TextureFromRenderBuffer")(
                        buffer
                    ) as *mut _)
                }
            }

            unsafe fn aa_resolved_texture_from_render_buffer(
                &self,
                buffer: graphics::RenderBuffer,
            ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLTexture>>::from_raw(self
                        .interface()
                        .AAResolvedTextureFromRenderBuffer
                        .expect("AAResolvedTextureFromRenderBuffer")(
                        buffer
                    ) as *mut _)
                }
            }

            unsafe fn stencil_texture_from_render_buffer(
                &self,
                buffer: graphics::RenderBuffer,
            ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLTexture>>::from_raw(self
                        .interface()
                        .StencilTextureFromRenderBuffer
                        .expect("StencilTextureFromRenderBuffer")(
                        buffer
                    ) as *mut _)
                }
            }
        }
    };
}

impl_metal_v1!(UnityGraphicsMetalV1);

define_unity_interface!(
    UnityGraphicsMetalV1,
    IUnityGraphicsMetalV1,
    0x29F8F3D03833465E_u64,
    0x92138551C15D823D_u64
);

pub trait UnityGraphicsMetalV2Interface {
    fn commit_current_command_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;
    fn command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;
}

macro_rules! impl_metal_v2 {
    ($intf:ty) => {
        impl_metal_v1!($intf);

        impl UnityGraphicsMetalV2Interface for $intf {
            fn commit_current_command_buffer(
                &self,
            ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLCommandBuffer>>::from_raw(self
                        .interface()
                        .CommitCurrentCommandBuffer
                        .expect("CommitCurrentCommandBuffer")(
                    ) as *mut _)
                }
            }

            fn command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>> {
                unsafe {
                    Retained::<ProtocolObject<dyn MTLCommandQueue>>::from_raw(self
                        .interface()
                        .CommandQueue
                        .expect("CommandQueue")(
                    ) as *mut _)
                }
            }
        }
    };
}
impl_metal_v2!(UnityGraphicsMetalV2);

define_unity_interface!(
    UnityGraphicsMetalV2,
    IUnityGraphicsMetalV2,
    0xF58857784FEF46EC_u64,
    0x9DB7A8803B87DA3D_u64
);
