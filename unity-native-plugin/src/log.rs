use crate::define_unity_interface;
use crate::interface::UnityInterface;
use std::ffi::CStr;
use unity_native_plugin_sys::*;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LogType
{
    Error = UnityLogType_kUnityLogTypeError,
    Warning = UnityLogType_kUnityLogTypeWarning,
    Log = UnityLogType_kUnityLogTypeLog,
    Exception = UnityLogType_kUnityLogTypeException,
}

define_unity_interface!(
    UnityLog,
    IUnityLog,
    0x9E7507fA5B444D5D_u64,
    0x92FB979515EA83FC_u64
);

impl UnityLog {
    pub fn log(&self, log_type: LogType, message: &CStr, file_name: &CStr, file_line: i32) {
        unsafe {
            self.interface().Log.expect("Log")(log_type as UnityLogType, message.as_ptr(), file_name.as_ptr(), file_line);
        }
    }
}