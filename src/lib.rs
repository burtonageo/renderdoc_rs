#![warn(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate libloading;
#[cfg(feature = "use-winit")]
extern crate winit;

pub mod raw_api;
mod extensions;

pub use extensions::{GetWindowHandle, GetDevicePointer};
use libc::{c_char, c_int, c_void, uint32_t};
use libloading::{Library, Symbol};
use raw_api::{RENDERDOC_API_1_0_0, RENDERDOC_API_1_0_1, RENDERDOC_API_1_0_2,
              RENDERDOC_API_1_1_0, RENDERDOC_API_1_1_1, RENDERDOC_InputButton,
              RENDERDOC_OverlayBits, RENDERDOC_Version};
use std::borrow::Cow;
use std::error::Error as StdError;
use std::ffi::{CStr, CString, NulError};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::{fmt, io, mem, ptr};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Version {
    V1_0_0,
    V1_0_1,
    V1_0_2,
    V1_1_0,
    V1_1_1,
}

bitflags! {
    #[derive(Default)]
    pub struct OverlayOptions: u32 {
        const NONE = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_None as u32;
        const ENABLED = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_Enabled as u32;
        const SHOW_AVERAGE_FRAME_RATE = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_FrameRate as u32;
        const SHOW_FRAME_NUMBER = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_FrameNumber as u32;
        const CAPTURE_LIST = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_CaptureList as u32;
        const DEFAULT = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_Default as u32;
        const ALL = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_All as u32;
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    _0 = RENDERDOC_InputButton::eRENDERDOC_Key_0 as u32,
    _1 = RENDERDOC_InputButton::eRENDERDOC_Key_1 as u32,
    _2 = RENDERDOC_InputButton::eRENDERDOC_Key_2 as u32,
    _3 = RENDERDOC_InputButton::eRENDERDOC_Key_3 as u32,
    _4 = RENDERDOC_InputButton::eRENDERDOC_Key_4 as u32,
    _5 = RENDERDOC_InputButton::eRENDERDOC_Key_5 as u32,
    _6 = RENDERDOC_InputButton::eRENDERDOC_Key_6 as u32,
    _7 = RENDERDOC_InputButton::eRENDERDOC_Key_7 as u32,
    _8 = RENDERDOC_InputButton::eRENDERDOC_Key_8 as u32,
    _9 = RENDERDOC_InputButton::eRENDERDOC_Key_9 as u32,

    A = RENDERDOC_InputButton::eRENDERDOC_Key_A as u32,
    B = RENDERDOC_InputButton::eRENDERDOC_Key_B as u32,
    C = RENDERDOC_InputButton::eRENDERDOC_Key_C as u32,
    D = RENDERDOC_InputButton::eRENDERDOC_Key_D as u32,
    E = RENDERDOC_InputButton::eRENDERDOC_Key_E as u32,
    F = RENDERDOC_InputButton::eRENDERDOC_Key_F as u32,
    G = RENDERDOC_InputButton::eRENDERDOC_Key_G as u32,
    H = RENDERDOC_InputButton::eRENDERDOC_Key_H as u32,
    I = RENDERDOC_InputButton::eRENDERDOC_Key_I as u32,
    J = RENDERDOC_InputButton::eRENDERDOC_Key_J as u32,
    K = RENDERDOC_InputButton::eRENDERDOC_Key_K as u32,
    L = RENDERDOC_InputButton::eRENDERDOC_Key_L as u32,
    M = RENDERDOC_InputButton::eRENDERDOC_Key_M as u32,
    N = RENDERDOC_InputButton::eRENDERDOC_Key_N as u32,
    O = RENDERDOC_InputButton::eRENDERDOC_Key_O as u32,
    P = RENDERDOC_InputButton::eRENDERDOC_Key_P as u32,
    Q = RENDERDOC_InputButton::eRENDERDOC_Key_Q as u32,
    R = RENDERDOC_InputButton::eRENDERDOC_Key_R as u32,
    S = RENDERDOC_InputButton::eRENDERDOC_Key_S as u32,
    T = RENDERDOC_InputButton::eRENDERDOC_Key_T as u32,
    U = RENDERDOC_InputButton::eRENDERDOC_Key_U as u32,
    V = RENDERDOC_InputButton::eRENDERDOC_Key_V as u32,
    W = RENDERDOC_InputButton::eRENDERDOC_Key_W as u32,
    X = RENDERDOC_InputButton::eRENDERDOC_Key_X as u32,
    Y = RENDERDOC_InputButton::eRENDERDOC_Key_Y as u32,
    Z = RENDERDOC_InputButton::eRENDERDOC_Key_Z as u32,

    NonPrintable = RENDERDOC_InputButton::eRENDERDOC_Key_NonPrintable as u32,

    Divide = RENDERDOC_InputButton::eRENDERDOC_Key_Divide as u32,
    Multiply = RENDERDOC_InputButton::eRENDERDOC_Key_Multiply as u32,
    Subtract = RENDERDOC_InputButton::eRENDERDOC_Key_Subtract as u32,
    Plus = RENDERDOC_InputButton::eRENDERDOC_Key_Plus as u32,

    F1 = RENDERDOC_InputButton::eRENDERDOC_Key_F1 as u32,
    F2 = RENDERDOC_InputButton::eRENDERDOC_Key_F2 as u32,
    F3 = RENDERDOC_InputButton::eRENDERDOC_Key_F3 as u32,
    F4 = RENDERDOC_InputButton::eRENDERDOC_Key_F4 as u32,
    F5 = RENDERDOC_InputButton::eRENDERDOC_Key_F5 as u32,
    F6 = RENDERDOC_InputButton::eRENDERDOC_Key_F6 as u32,
    F7 = RENDERDOC_InputButton::eRENDERDOC_Key_F7 as u32,
    F8 = RENDERDOC_InputButton::eRENDERDOC_Key_F8 as u32,
    F9 = RENDERDOC_InputButton::eRENDERDOC_Key_F9 as u32,
    F10 = RENDERDOC_InputButton::eRENDERDOC_Key_F10 as u32,
    F11 = RENDERDOC_InputButton::eRENDERDOC_Key_F11 as u32,
    F12 = RENDERDOC_InputButton::eRENDERDOC_Key_F12 as u32,

    Home = RENDERDOC_InputButton::eRENDERDOC_Key_Home as u32,
    End = RENDERDOC_InputButton::eRENDERDOC_Key_End as u32,
    Insert = RENDERDOC_InputButton::eRENDERDOC_Key_Insert as u32,
    Delete = RENDERDOC_InputButton::eRENDERDOC_Key_Delete as u32,
    PageUp = RENDERDOC_InputButton::eRENDERDOC_Key_PageUp as u32,
    PageDn = RENDERDOC_InputButton::eRENDERDOC_Key_PageDn as u32,

    Backspace = RENDERDOC_InputButton::eRENDERDOC_Key_Backspace as u32,
    Tab = RENDERDOC_InputButton::eRENDERDOC_Key_Tab as u32,
    PrtScrn = RENDERDOC_InputButton::eRENDERDOC_Key_PrtScrn as u32,
    Pause = RENDERDOC_InputButton::eRENDERDOC_Key_Pause as u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Options {
    pub is_vsync_allowed: bool,
    pub is_fullscreen_allowed: bool,
    pub enable_api_validation: bool,
    pub should_capture_cpu_callstacks: bool,
    pub debugger_delay: f32,
    pub verify_map_writes: bool,
    pub should_hook_into_children: bool,
    pub reference_all_resources: bool,
    pub save_initial_states: bool,
    pub capture_all_command_lists: bool,
    pub mute_debug_output: bool,
}

static LIB_INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

#[allow(dead_code)]
enum RenderDocApi {
    V100(RENDERDOC_API_1_0_0),
    V101(RENDERDOC_API_1_0_1),
    V102(RENDERDOC_API_1_0_2),
    V110(RENDERDOC_API_1_1_0),
    V111(RENDERDOC_API_1_1_1),
}

impl fmt::Debug for RenderDocApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderDocApi::V100(ref v) => fmt::Debug::fmt(v, f),
            RenderDocApi::V101(ref v) => fmt::Debug::fmt(v, f),
            RenderDocApi::V102(ref v) => fmt::Debug::fmt(v, f),
            RenderDocApi::V110(ref v) => fmt::Debug::fmt(v, f),
            RenderDocApi::V111(ref v) => fmt::Debug::fmt(v, f),
        }
    }
}

/// A handle to the main `renderdoc` api.
///
/// The `renderdoc` api is loaded at runtime through a dylib,
/// so all access to the api is mediated through this struct.
#[derive(Debug)]
pub struct RenderDoc {
    dylib: Library,
    api: RENDERDOC_API_1_1_1,
    _not_threadsafe: PhantomData<*mut ()>,
}

impl RenderDoc {
    /// Construct a new `RenderDoc` instance.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub fn new(version: Version) -> CreateResult<Self> {
        if LIB_INITIALIZED.fetch_xor(true, Ordering::SeqCst) {
            LIB_INITIALIZED.store(true, Ordering::SeqCst);
        } else {
            return Err(CreateError::AlreadyInitialized);
        }

        let dylib = Library::new("librenderdoc.so")?;
        let mut api = unsafe { mem::zeroed() };

        unsafe {
            let get_renderdoc_api: Symbol<
                unsafe extern "C" fn(RENDERDOC_Version,
                                     *mut *mut c_void)
                                     -> c_int,
            > = dylib.get(b"RENDERDOC_GetAPI")?;
            let api_ptr = &mut api as *mut _;
            let mut api_ptr_void = api_ptr as *mut c_void;
            let result = get_renderdoc_api(version.into(), &mut api_ptr_void as *mut *mut c_void);
            if result == 0 || api_ptr_void.is_null() {
                return Err(Error::empty().into());
            }
        }

        Ok(RenderDoc {
            dylib,
            api,
            _not_threadsafe: PhantomData,
        })
    }

    /// Construct a new `RenderDoc` instance.
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    #[allow(unused_variables)]
    #[inline]
    pub fn new(version: Version) -> CreateResult<Self> {
        Err(CreateError::UnsupportedPlatform)
    }

    /// Get the api version of the linked `renderdoc` library.
    ///
    /// The format of the return value is `(major, minor, patch)`.
    pub fn api_version(&self) -> (c_int, c_int, c_int) {
        let (mut major, mut minor, mut patch) = Default::default();
        unsafe {
            (self.api.GetAPIVersion)(&mut major, &mut minor, &mut patch);
        }
        (major, minor, patch)
    }

    pub fn set_focus_toggle_keys(&mut self, keys: &mut [Key]) {
        let (len, ptr) = (
            keys.len() as c_int,
            keys.as_mut_ptr() as *mut RENDERDOC_InputButton,
        );
        unsafe {
            (self.api.SetFocusToggleKeys)(ptr, len);
        }
    }

    pub fn set_capture_keys(&mut self, keys: &mut [Key]) {
        let (len, ptr) = (
            keys.len() as c_int,
            keys.as_mut_ptr() as *mut RENDERDOC_InputButton,
        );
        unsafe {
            (self.api.SetCaptureKeys)(ptr, len);
        }
    }

    /// Attempts to shutdown the `renderdoc` library.
    ///
    /// # Safety
    ///
    /// If any graphics APIs have been called, then this function will result
    /// in undefined behaviour.
    pub unsafe fn shutdown(self) {
        (self.api.Shutdown)();
        LIB_INITIALIZED.store(false, Ordering::SeqCst);
    }

    ///
    pub fn unload_crash_handler(&mut self) {
        unsafe {
            (self.api.UnloadCrashHandler)();
        }
    }

    ///
    pub fn set_log_file_path_template(&mut self, log_file_path_template: &str) -> StdResult<(), NulError> {
        let log_file_path_template = CString::new(log_file_path_template)?;
        unsafe {
            (self.api.SetLogFilePathTemplate)(log_file_path_template.as_ptr());
        }
        Ok(())
    }

    ///
    pub fn get_log_file_path_template(&self) -> Cow<str> {
        unsafe {
            CStr::from_ptr((self.api.GetLogFilePathTemplate)()).to_string_lossy()
        }

    }

    /// Set the Window and Device which you want this `renderdoc` instance to inspect.
    pub fn set_window<W, D>(&mut self, window_handle: &W, device_pointer: &D)
    where
        W: GetWindowHandle,
        D: GetDevicePointer,
    {
        unsafe {
            let window_handle = window_handle.get_window_handle();
            let device_pointer = device_pointer.get_device_pointer();
            if !window_handle.is_null() && !device_pointer.is_null() {
                (self.api.SetActiveWindow)(window_handle, device_pointer);
            }
        }
    }

    ///
    pub fn num_captures(&self) -> uint32_t {
        unsafe { (self.api.GetNumCaptures)() }
    }

    ///
    pub fn is_target_control_connected(&self) -> bool {
        unsafe { (self.api.IsTargetControlConnected)() == 1 }
    }

    /// Launches the replay UI. Returns the `pid` of the replay ui process if successful.
    pub fn launch_replay_ui(&self, command_line: Option<&str>) -> Result<uint32_t> {
        let (connect_target_control, command_line) = match command_line {
            Some(s) => (1, s.as_ptr()),
            None => (0, ptr::null()),
        };
        let result = unsafe {
            (self.api.LaunchReplayUI)(connect_target_control, command_line as *const c_char)
        };
        if result != 0 {
            Ok(result)
        } else {
            Err(Error::empty())
        }
    }

    /// Get the inner API struct.
    ///
    /// # Safety
    ///
    /// It is possible to circumvent the safety requirements of using this library by calling
    /// api functions directly.
    pub unsafe fn raw_renderdoc_api(&self) -> RENDERDOC_API_1_1_1 {
        self.api
    }
}

pub type StdResult<T, E> = ::std::result::Result<T, E>;

/// Error to enumerate possible fail states when constructing the `RenderDoc` instance.
#[derive(Debug)]
pub enum CreateError {
    /// An error occurred while opening the `renderdoc` shared library.
    LibLoading(io::Error),
    /// An internal `renderdoc` error occurred.
    RenderDoc(Error),
    /// `renderdoc` is not supported on the target platform.
    UnsupportedPlatform,
    /// Another `RenderDoc` instance has been created.
    AlreadyInitialized,
}

impl fmt::Display for CreateError {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let desc = self.description();
        match *self {
            CreateError::LibLoading(ref e) => write!(fmtr, "{}: {}", desc, e),
            CreateError::RenderDoc(ref e) => write!(fmtr, "{}: {}", desc, e),
            CreateError::UnsupportedPlatform |
            CreateError::AlreadyInitialized => fmtr.pad(desc),
        }
    }
}

impl StdError for CreateError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            CreateError::LibLoading(_) |
            CreateError::RenderDoc(_) => "could not create RenderDoc instance",
            CreateError::UnsupportedPlatform => "renderdoc is not supported on this platform",
            CreateError::AlreadyInitialized => "another RenderDoc instance has already been initialized",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&StdError> {
        match *self {
            CreateError::LibLoading(ref e) => Some(e),
            CreateError::RenderDoc(ref e) => Some(e),
            CreateError::UnsupportedPlatform |
            CreateError::AlreadyInitialized => None,
        }
    }
}

impl From<io::Error> for CreateError {
    #[inline]
    fn from(e: io::Error) -> Self {
        CreateError::LibLoading(e)
    }
}

impl From<Error> for CreateError {
    #[inline]
    fn from(e: Error) -> Self {
        CreateError::RenderDoc(e)
    }
}

pub type CreateResult<T> = StdResult<T, CreateError>;

/// Represents a generic error which may occur when using the `renderdoc` api.
///
/// May have an associated error code.
#[derive(Debug)]
pub struct Error {
    code: Option<c_int>,
}

impl Error {
    /// Construct a new `Error` without an associated code.
    #[inline]
    fn empty() -> Self {
        Error { code: None }
    }

    /// Construct a new `Error` with an associated code.
    #[inline]
    #[allow(dead_code)]
    fn with_code(code: c_int) -> Self {
        Error { code: Some(code) }
    }

    /// Returns the associated code if there is one, otherwise returns `None`.
    #[inline]
    pub fn code(&self) -> Option<c_int> {
        self.code
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        if let Some(code) = self.code {
            write!(fmtr, "{}: {}", self.description(), code)
        } else {
            fmtr.pad(self.description())
        }
    }
}

impl StdError for Error {
    #[inline]
    fn description(&self) -> &str {
        "an unspecified error occurred when calling the renderdoc API"
    }
}

pub type Result<T> = StdResult<T, ::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
