use crate::define_unity_interface;
use crate::interface::UnityInterface;
use crate::profiler::*;
use std::ffi::c_void;
use std::fmt::*;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityProfilerCallbacks,
    unity_native_plugin_sys::IUnityProfilerCallbacks,
    0x572FDB38CE3C4B1F_u64,
    0xA6071A9A7C4F52D8_u64
);

#[derive(Debug)]
pub struct ProfilerCategoryDesc {
    pub(crate) native: *const UnityProfilerCategoryDesc,
}

impl ProfilerCategoryDesc {
    pub fn id(&self) -> ProfilerCategoryId {
        unsafe { (*self.native).id as ProfilerCategoryId }
    }

    pub fn rgba_color(&self) -> u32 {
        unsafe { (*self.native).rgbaColor }
    }

    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((*self.native).name) }
    }
}

pub struct ProfilerThreadDesc {
    pub(crate) native: *const UnityProfilerThreadDesc,
}

impl ProfilerThreadDesc {
    pub fn thread_id(&self) -> ProfilerThreadId {
        unsafe { (*self.native).threadId as ProfilerThreadId }
    }

    pub fn group_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((*self.native).groupName) }
    }

    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((*self.native).name) }
    }
}

pub struct ProfilerMarkerEvent<'a> {
    pub desc: ProfilerMarkerDesc,
    pub event_type: ProfilerMarkerEventType,
    event_data: &'a [UnityProfilerMarkerData],
}

impl<'a> std::fmt::Debug for ProfilerMarkerEvent<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "[event desc={:?}, ty={:?}, data_len={:?}]",
            self.desc,
            self.event_type,
            self.event_data.len(),
        )
    }
}

extern "system" fn create_category_bridge(
    _desc: *const UnityProfilerCategoryDesc,
    _userdata: *mut c_void,
) {
    let ptr = _userdata as *mut Box<dyn FnMut(&ProfilerCategoryDesc)>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    let desc = ProfilerCategoryDesc { native: _desc };
    cb(&desc);
    std::mem::forget(cb);
}

extern "system" fn create_marker_bridge(
    _desc: *const UnityProfilerMarkerDesc,
    _userdata: *mut c_void,
) {
    let ptr = _userdata as *mut Box<dyn FnMut(&ProfilerMarkerDesc)>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    let desc = ProfilerMarkerDesc { native: _desc };
    cb(&desc);
    std::mem::forget(cb);
}

extern "system" fn marker_event_bridge(
    _desc: *const UnityProfilerMarkerDesc,
    _event_type: UnityProfilerMarkerEventType,
    _event_data_count: u16,
    _event_data: *const UnityProfilerMarkerData,
    _userdata: *mut c_void,
) {
    let desc = ProfilerMarkerDesc { native: _desc };
    let event_type = match ProfilerMarkerEventType::from(_event_type) {
        Some(v) => v,
        None => return,
    };

    let event_data = unsafe { std::slice::from_raw_parts(_event_data, _event_data_count as usize) };

    let desc = ProfilerMarkerEvent {
        desc,
        event_type,
        event_data,
    };

    let ptr = _userdata as *mut Box<dyn FnMut(&ProfilerMarkerEvent)>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    cb(&desc);
    std::mem::forget(cb);
}

extern "system" fn frame_bridge(_userdata: *mut c_void) {
    let ptr = _userdata as *mut Box<dyn FnMut()>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    cb();
    std::mem::forget(cb);
}

extern "system" fn create_thread_bridge(
    desc: *const UnityProfilerThreadDesc,
    _userdata: *mut c_void,
) {
    let desc = ProfilerThreadDesc { native: desc };

    let ptr = _userdata as *mut Box<dyn FnMut(&ProfilerThreadDesc)>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    cb(&desc);
    std::mem::forget(cb);
}

pub struct CreateCategoryRegister(*mut c_void);
pub struct CreateMarkerRegister(*mut c_void);
pub struct MarkerEventRegister {
    ptr: *mut c_void,
    desc: *const UnityProfilerMarkerDesc,
}
pub struct FrameRegister(*mut c_void);
pub struct CreateThreadRegister(*mut c_void);

macro_rules! iface_fn {
    ($self:tt, $name:tt) => {
        $self.interface().$name.expect(stringify!($name))
    };
}

macro_rules! common_impl {
    ($name: tt) => {
        impl $name {
            pub fn register_create_category(
                &self,
                f: Box<dyn FnMut(&ProfilerCategoryDesc) + Send + Sync>,
            ) -> CreateCategoryRegister {
                let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

                unsafe {
                    iface_fn!(self, RegisterCreateCategoryCallback)(
                        Some(create_category_bridge),
                        ptr,
                    );
                }
                CreateCategoryRegister(ptr)
            }

            pub fn unregister_create_category(&self, register: CreateCategoryRegister) {
                unsafe {
                    iface_fn!(self, UnregisterCreateCategoryCallback)(
                        Some(create_category_bridge),
                        register.0,
                    );
                }
            }

            pub fn register_create_marker(
                &self,
                f: Box<dyn FnMut(&ProfilerMarkerDesc) + Send + Sync>,
            ) -> CreateMarkerRegister {
                let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

                unsafe {
                    iface_fn!(self, RegisterCreateMarkerCallback)(Some(create_marker_bridge), ptr);
                }
                CreateMarkerRegister(ptr)
            }

            pub fn unregister_create_marker(&self, register: CreateMarkerRegister) {
                unsafe {
                    iface_fn!(self, UnregisterCreateMarkerCallback)(
                        Some(create_marker_bridge),
                        register.0,
                    );
                }
            }

            pub fn register_marker_event(
                &self,
                desc: &ProfilerMarkerDesc,
                f: Box<dyn FnMut(&ProfilerMarkerEvent) + Send + Sync>,
            ) -> MarkerEventRegister {
                let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

                unsafe {
                    iface_fn!(self, RegisterMarkerEventCallback)(
                        desc.native,
                        Some(marker_event_bridge),
                        ptr,
                    );
                }
                MarkerEventRegister {
                    ptr,
                    desc: desc.native,
                }
            }

            pub fn unregister_marker_event(&self, register: MarkerEventRegister) {
                unsafe {
                    iface_fn!(self, UnregisterMarkerEventCallback)(
                        register.desc,
                        Some(marker_event_bridge),
                        register.ptr,
                    );
                }
            }

            pub fn register_frame(&self, f: Box<dyn FnMut() + Send + Sync>) -> FrameRegister {
                let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

                unsafe {
                    iface_fn!(self, RegisterFrameCallback)(Some(frame_bridge), ptr);
                }
                FrameRegister(ptr)
            }

            pub fn unregister_frame(&self, register: FrameRegister) {
                unsafe {
                    iface_fn!(self, UnregisterFrameCallback)(Some(frame_bridge), register.0);
                }
            }

            pub fn register_create_thread(
                &self,
                f: Box<dyn FnMut(&ProfilerThreadDesc) + Send + Sync>,
            ) -> CreateThreadRegister {
                let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

                unsafe {
                    iface_fn!(self, RegisterCreateThreadCallback)(Some(create_thread_bridge), ptr);
                }
                CreateThreadRegister(ptr)
            }

            pub fn unregister_create_thread(&self, register: CreateThreadRegister) {
                unsafe {
                    iface_fn!(self, UnregisterCreateThreadCallback)(
                        Some(create_thread_bridge),
                        register.0,
                    );
                }
            }
        }
    };
}

common_impl!(UnityProfilerCallbacks);

define_unity_interface!(
    UnityProfilerCallbacksV2,
    unity_native_plugin_sys::IUnityProfilerCallbacksV2,
    0x5DEB59E88F2D4571_u64,
    0x81E8583069A5E33C_u64
);

#[derive(Clone, Copy, Debug)]
pub struct ProfilerFlowEvent {
    pub ty: ProfilerFlowEventType,
    pub flow_id: u32,
}

extern "system" fn flow_event_bridge(
    ty: UnityProfilerFlowEventType,
    flow_id: u32,
    _userdata: *mut c_void,
) {
    let ty = match ProfilerFlowEventType::from(ty) {
        Some(ty) => ty,
        None => return,
    };

    let desc = ProfilerFlowEvent { ty, flow_id };

    let ptr = _userdata as *mut Box<dyn FnMut(&ProfilerFlowEvent)>;
    let mut cb = unsafe { Box::from_raw(ptr) };

    cb(&desc);
    std::mem::forget(cb);
}

pub struct FlowEventRegister(*mut c_void);

common_impl!(UnityProfilerCallbacksV2);
impl UnityProfilerCallbacksV2 {
    pub fn register_flow_event(
        &self,
        f: Box<dyn FnMut(&ProfilerFlowEvent) + Send + Sync>,
    ) -> FlowEventRegister {
        let ptr = Box::into_raw(Box::new(f)) as *mut c_void;

        unsafe {
            iface_fn!(self, RegisterFlowEventCallback)(Some(flow_event_bridge), ptr);
        }
        FlowEventRegister(ptr)
    }

    pub fn unregister_flow_event(&self, register: FlowEventRegister) {
        unsafe {
            iface_fn!(self, UnregisterFlowEventCallback)(Some(flow_event_bridge), register.0);
        }
    }
}
