use crate::define_unity_interface;
use crate::interface::UnityInterface;
use std::ffi::{c_void, CStr};
use std::ptr::null_mut;
use unity_native_plugin_sys::*;


define_unity_interface!(
    UnityMemoryManager,
    IUnityMemoryManager,
    0xBAF9E57C61A811EC_u64,
    0xC5A7CC7861A811EC_u64
);

pub struct UnityAllocator {
    allocator: *mut unity_native_plugin_sys::UnityAllocator,
    memory_manager: UnityMemoryManager,
}

impl Drop for UnityAllocator {
    fn drop(&mut self) {
        unsafe {
            self.memory_manager.destroy_allocator(self.allocator);
        }
    }
}

impl UnityAllocator {
    pub unsafe fn allocate(&self,
                           size: usize,
                           align: usize,
                           file: &CStr,
                           line: i32) -> *mut c_void {
        self.memory_manager.allocate(self.allocator, size, align, file, line)
    }

    pub unsafe fn deallocate(&self,
                             ptr: *mut c_void,
                             file: &CStr,
                             line: i32) {
        self.memory_manager.deallocate(self.allocator, ptr, file, line)
    }

    pub unsafe fn reallocate(&self,
                             ptr: *mut c_void,
                             size: usize,
                             align: usize,
                             file: &CStr,
                             line: i32) -> *mut c_void {
        self.memory_manager.reallocate(self.allocator, ptr, size, align, file, line)
    }
}

impl UnityMemoryManager {
    pub unsafe fn create_allocator(&self, area_name: &CStr, object_name: &CStr) -> Option<UnityAllocator> {
        let allocator = self.interface().CreateAllocator.expect("CreateAllocator")(area_name.as_ptr(), object_name.as_ptr());
        if allocator != null_mut() {
            Some(UnityAllocator { allocator: allocator, memory_manager: self.clone() })
        } else {
            None
        }
    }

    pub(crate) unsafe fn destroy_allocator(&self, allocator: *mut unity_native_plugin_sys::UnityAllocator) {
        self.interface().DestroyAllocator.expect("DestroyAllocator")(allocator)
    }

    pub(crate) unsafe fn allocate(&self,
                                  allocator: *mut unity_native_plugin_sys::UnityAllocator,
                                  size: usize,
                                  align: usize,
                                  file: &CStr,
                                  line: i32) -> *mut c_void {
        self.interface().Allocate.expect("Allocate")(allocator, size, align, file.as_ptr(), line)
    }

    pub(crate) unsafe fn deallocate(&self,
                                    allocator: *mut unity_native_plugin_sys::UnityAllocator,
                                    ptr: *mut c_void,
                                    file: &CStr,
                                    line: i32) {
        self.interface().Deallocate.expect("Deallocate")(allocator, ptr, file.as_ptr(), line)
    }

    pub(crate) unsafe fn reallocate(&self,
                                    allocator: *mut unity_native_plugin_sys::UnityAllocator,
                                    ptr: *mut c_void,
                                    size: usize,
                                    align: usize,
                                    file: &CStr,
                                    line: i32) -> *mut c_void {
        self.interface().Reallocate.expect("Reallocate")(allocator, ptr, size, align, file.as_ptr(), line)
    }
}