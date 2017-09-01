#![allow(bad_style, missing_documentation)]

use Version;
use libc::{c_char, c_float, c_int, c_void, uint32_t, uint64_t};

pub const RENDERDOC_ShaderDebugMagicValue_bytearray: &[u8] = &[
    0x20,
    0x55,
    0xb2,
    0xea,
    0x70,
    0x66,
    0x65,
    0x48,
    0x84,
    0x29,
    0x6c,
    0x8,
    0x51,
    0x54,
    0x00,
    0xff,
];
pub const RENDERDOC_ShaderDebugMagicValue_truncated: u64 = 0x48656670eab25520;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RENDERDOC_CaptureOption {
    eRENDERDOC_Option_AllowVSync = 0,
    eRENDERDOC_Option_AllowFullscreen = 1,
    eRENDERDOC_Option_APIValidation = 2,
    eRENDERDOC_Option_CaptureCallstacks = 3,
    eRENDERDOC_Option_CaptureCallstacksOnlyDraws = 4,
    eRENDERDOC_Option_DelayForDebugger = 5,
    eRENDERDOC_Option_VerifyMapWrites = 6,
    eRENDERDOC_Option_HookIntoChildren = 7,
    eRENDERDOC_Option_RefAllResources = 8,
    eRENDERDOC_Option_SaveAllInitials = 9,
    eRENDERDOC_Option_CaptureAllCmdLists = 10,
    eRENDERDOC_Option_DebugOutputMute = 11,
}

pub type pRENDERDOC_SetCaptureOptionU32 = unsafe extern "C" fn(opt: RENDERDOC_CaptureOption,
                                                               val: uint32_t)
                                                               -> c_int;
pub type pRENDERDOC_SetCaptureOptionF32 = unsafe extern "C" fn(opt: RENDERDOC_CaptureOption,
                                                               val: c_float)
                                                               -> c_int;

pub type pRENDERDOC_GetCaptureOptionU32 = unsafe extern "C" fn(opt: RENDERDOC_CaptureOption)
                                                               -> uint32_t;
pub type pRENDERDOC_GetCaptureOptionF32 = unsafe extern "C" fn(opt: RENDERDOC_CaptureOption)
                                                               -> c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RENDERDOC_InputButton {
    eRENDERDOC_Key_0 = 0x30,
    eRENDERDOC_Key_1 = 0x31,
    eRENDERDOC_Key_2 = 0x32,
    eRENDERDOC_Key_3 = 0x33,
    eRENDERDOC_Key_4 = 0x34,
    eRENDERDOC_Key_5 = 0x35,
    eRENDERDOC_Key_6 = 0x36,
    eRENDERDOC_Key_7 = 0x37,
    eRENDERDOC_Key_8 = 0x38,
    eRENDERDOC_Key_9 = 0x39,

    eRENDERDOC_Key_A = 0x41,
    eRENDERDOC_Key_B = 0x42,
    eRENDERDOC_Key_C = 0x43,
    eRENDERDOC_Key_D = 0x44,
    eRENDERDOC_Key_E = 0x45,
    eRENDERDOC_Key_F = 0x46,
    eRENDERDOC_Key_G = 0x47,
    eRENDERDOC_Key_H = 0x48,
    eRENDERDOC_Key_I = 0x49,
    eRENDERDOC_Key_J = 0x4A,
    eRENDERDOC_Key_K = 0x4B,
    eRENDERDOC_Key_L = 0x4C,
    eRENDERDOC_Key_M = 0x4D,
    eRENDERDOC_Key_N = 0x4E,
    eRENDERDOC_Key_O = 0x4F,
    eRENDERDOC_Key_P = 0x50,
    eRENDERDOC_Key_Q = 0x51,
    eRENDERDOC_Key_R = 0x52,
    eRENDERDOC_Key_S = 0x53,
    eRENDERDOC_Key_T = 0x54,
    eRENDERDOC_Key_U = 0x55,
    eRENDERDOC_Key_V = 0x56,
    eRENDERDOC_Key_W = 0x57,
    eRENDERDOC_Key_X = 0x58,
    eRENDERDOC_Key_Y = 0x59,
    eRENDERDOC_Key_Z = 0x5A,

    eRENDERDOC_Key_NonPrintable = 0x100,

    eRENDERDOC_Key_Divide,
    eRENDERDOC_Key_Multiply,
    eRENDERDOC_Key_Subtract,
    eRENDERDOC_Key_Plus,

    eRENDERDOC_Key_F1,
    eRENDERDOC_Key_F2,
    eRENDERDOC_Key_F3,
    eRENDERDOC_Key_F4,
    eRENDERDOC_Key_F5,
    eRENDERDOC_Key_F6,
    eRENDERDOC_Key_F7,
    eRENDERDOC_Key_F8,
    eRENDERDOC_Key_F9,
    eRENDERDOC_Key_F10,
    eRENDERDOC_Key_F11,
    eRENDERDOC_Key_F12,

    eRENDERDOC_Key_Home,
    eRENDERDOC_Key_End,
    eRENDERDOC_Key_Insert,
    eRENDERDOC_Key_Delete,
    eRENDERDOC_Key_PageUp,
    eRENDERDOC_Key_PageDn,

    eRENDERDOC_Key_Backspace,
    eRENDERDOC_Key_Tab,
    eRENDERDOC_Key_PrtScrn,
    eRENDERDOC_Key_Pause,

    eRENDERDOC_Key_Max,
}

pub type pRENDERDOC_SetFocusToggleKeys = unsafe extern "C" fn(keys: *mut RENDERDOC_InputButton,
                                                              num: c_int);
pub type pRENDERDOC_SetCaptureKeys = unsafe extern "C" fn(keys: *mut RENDERDOC_InputButton,
                                                          num: c_int);

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RENDERDOC_OverlayBits {
    eRENDERDOC_Overlay_Enabled = 0x1,
    eRENDERDOC_Overlay_FrameRate = 0x2,
    eRENDERDOC_Overlay_FrameNumber = 0x4,
    eRENDERDOC_Overlay_CaptureList = 0x8,
    eRENDERDOC_Overlay_Default = RENDERDOC_OverlayBits::eRENDERDOC_Overlay_Enabled as u32 |
        RENDERDOC_OverlayBits::eRENDERDOC_Overlay_FrameRate as u32 |
        RENDERDOC_OverlayBits::eRENDERDOC_Overlay_FrameNumber as u32 |
        RENDERDOC_OverlayBits::eRENDERDOC_Overlay_CaptureList as u32,
    eRENDERDOC_Overlay_All = !0,
    eRENDERDOC_Overlay_None = 0,
}

pub type pRENDERDOC_GetOverlayBits = unsafe extern "C" fn() -> uint32_t;
pub type pRENDERDOC_MaskOverlayBits = unsafe extern "C" fn(And: uint32_t, Or: uint32_t);

pub type pRENDERDOC_Shutdown = unsafe extern "C" fn();

pub type pRENDERDOC_UnloadCrashHandler = unsafe extern "C" fn();

pub type pRENDERDOC_SetLogFilePathTemplate = unsafe extern "C" fn(pathtemplate: *const c_char);
pub type pRENDERDOC_GetLogFilePathTemplate = unsafe extern "C" fn() -> *const c_char;

pub type pRENDERDOC_GetNumCaptures = unsafe extern "C" fn() -> uint32_t;

// This function returns the details of a capture, by index. New captures are added
// to the end of the list.
//
// logfile will be filled with the absolute path to the capture file, as a UTF-8 string
// pathlength will be written with the length in bytes of the logfile string
// timestamp will be written with the time of the capture, in seconds since the Unix epoch
//
// Any of the parameters can be NULL and they'll be skipped.
//
// The function will return 1 if the capture index is valid, or 0 if the index is invalid
// If the index is invalid, the values will be unchanged
//
// Note: when captures are deleted in the UI they will remain in this list, so the
// logfile path may not exist anymore.
pub type pRENDERDOC_GetCapture = unsafe extern "C" fn(idx: uint32_t,
                                                      logfile: *mut c_char,
                                                      pathlength: *mut uint32_t,
                                                      timestamp: *mut uint64_t)
                                                      -> uint32_t;


pub type pRENDERDOC_IsRemoteAccessConnected = unsafe extern "C" fn() -> uint32_t;
pub type pRENDERDOC_IsTargetControlConnected = unsafe extern "C" fn() -> uint32_t;

pub type pRENDERDOC_LaunchReplayUI = unsafe extern "C" fn(connectTargetControl: uint32_t,
                                                          cmdline: *const c_char)
                                                          -> uint32_t;
pub type pRENDERDOC_GetAPIVersion = unsafe extern "C" fn(major: *mut c_int,
                                                         minor: *mut c_int,
                                                         patch: *mut c_int);

pub type RENDERDOC_DevicePointer = *mut c_void;
pub type RENDERDOC_WindowHandle = *mut c_void;

pub type pRENDERDOC_SetActiveWindow = unsafe extern "C" fn(device: RENDERDOC_DevicePointer,
                                                           wndHandle: RENDERDOC_WindowHandle);

pub type pRENDERDOC_TriggerCapture = unsafe extern "C" fn();

pub type pRENDERDOC_TriggerMultiFrameCapture = unsafe extern "C" fn(numFrames: uint32_t);

pub type pRENDERDOC_StartFrameCapture = unsafe extern "C" fn(device: RENDERDOC_DevicePointer,
                                                             wndHandle: RENDERDOC_WindowHandle);

pub type pRENDERDOC_IsFrameCapturing = unsafe extern "C" fn() -> uint32_t;

pub type pRENDERDOC_EndFrameCapture = unsafe extern "C" fn(device: RENDERDOC_DevicePointer,
                                                           wndHandle: RENDERDOC_WindowHandle)
                                                           -> uint32_t;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RENDERDOC_Version {
    eRENDERDOC_API_Version_1_0_0 = 10000,
    eRENDERDOC_API_Version_1_0_1 = 10001,
    eRENDERDOC_API_Version_1_0_2 = 10002,
    eRENDERDOC_API_Version_1_1_0 = 10100,
    eRENDERDOC_API_Version_1_1_1 = 10101,
}

impl Into<RENDERDOC_Version> for Version {
    #[inline]
    fn into(self) -> RENDERDOC_Version {
        match self {
            Version::V1_0_0 => RENDERDOC_Version::eRENDERDOC_API_Version_1_0_0,
            Version::V1_0_1 => RENDERDOC_Version::eRENDERDOC_API_Version_1_0_1,
            Version::V1_0_2 => RENDERDOC_Version::eRENDERDOC_API_Version_1_0_2,
            Version::V1_1_0 => RENDERDOC_Version::eRENDERDOC_API_Version_1_1_0,
            Version::V1_1_1 => RENDERDOC_Version::eRENDERDOC_API_Version_1_1_1,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RENDERDOC_API_1_1_0 {
    pub GetAPIVersion: pRENDERDOC_GetAPIVersion,

    pub SetCaptureOptionU32: pRENDERDOC_SetCaptureOptionU32,
    pub SetCaptureOptionF32: pRENDERDOC_SetCaptureOptionF32,

    pub GetCaptureOptionU32: pRENDERDOC_GetCaptureOptionU32,
    pub GetCaptureOptionF32: pRENDERDOC_GetCaptureOptionF32,

    pub SetFocusToggleKeys: pRENDERDOC_SetFocusToggleKeys,
    pub SetCaptureKeys: pRENDERDOC_SetCaptureKeys,

    pub GetOverlayBits: pRENDERDOC_GetOverlayBits,
    pub MaskOverlayBits: pRENDERDOC_MaskOverlayBits,

    pub Shutdown: pRENDERDOC_Shutdown,
    pub UnloadCrashHandler: pRENDERDOC_UnloadCrashHandler,

    pub SetLogFilePathTemplate: pRENDERDOC_SetLogFilePathTemplate,
    pub GetLogFilePathTemplate: pRENDERDOC_GetLogFilePathTemplate,

    pub GetNumCaptures: pRENDERDOC_GetNumCaptures,
    pub GetCapture: pRENDERDOC_GetCapture,

    pub TriggerCapture: pRENDERDOC_TriggerCapture,

    pub IsRemoteAccessConnected: pRENDERDOC_IsRemoteAccessConnected,
    pub LaunchReplayUI: pRENDERDOC_LaunchReplayUI,

    pub SetActiveWindow: pRENDERDOC_SetActiveWindow,

    pub StartFrameCapture: pRENDERDOC_StartFrameCapture,
    pub IsFrameCapturing: pRENDERDOC_IsFrameCapturing,
    pub EndFrameCapture: pRENDERDOC_EndFrameCapture,

    pub TriggerMultiFrameCapture: pRENDERDOC_TriggerMultiFrameCapture,
}

pub type RENDERDOC_API_1_0_0 = RENDERDOC_API_1_1_0;
pub type RENDERDOC_API_1_0_1 = RENDERDOC_API_1_1_0;
pub type RENDERDOC_API_1_0_2 = RENDERDOC_API_1_1_0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RENDERDOC_API_1_1_1 {
    pub GetAPIVersion: pRENDERDOC_GetAPIVersion,

    pub SetCaptureOptionU32: pRENDERDOC_SetCaptureOptionU32,
    pub SetCaptureOptionF32: pRENDERDOC_SetCaptureOptionF32,

    pub GetCaptureOptionU32: pRENDERDOC_GetCaptureOptionU32,
    pub GetCaptureOptionF32: pRENDERDOC_GetCaptureOptionF32,

    pub SetFocusToggleKeys: pRENDERDOC_SetFocusToggleKeys,
    pub SetCaptureKeys: pRENDERDOC_SetCaptureKeys,

    pub GetOverlayBits: pRENDERDOC_GetOverlayBits,
    pub MaskOverlayBits: pRENDERDOC_MaskOverlayBits,

    pub Shutdown: pRENDERDOC_Shutdown,
    pub UnloadCrashHandler: pRENDERDOC_UnloadCrashHandler,

    pub SetLogFilePathTemplate: pRENDERDOC_SetLogFilePathTemplate,
    pub GetLogFilePathTemplate: pRENDERDOC_GetLogFilePathTemplate,

    pub GetNumCaptures: pRENDERDOC_GetNumCaptures,
    pub GetCapture: pRENDERDOC_GetCapture,

    pub TriggerCapture: pRENDERDOC_TriggerCapture,

    pub IsTargetControlConnected: pRENDERDOC_IsTargetControlConnected,
    pub LaunchReplayUI: pRENDERDOC_LaunchReplayUI,

    pub SetActiveWindow: pRENDERDOC_SetActiveWindow,

    pub StartFrameCapture: pRENDERDOC_StartFrameCapture,
    pub IsFrameCapturing: pRENDERDOC_IsFrameCapturing,
    pub EndFrameCapture: pRENDERDOC_EndFrameCapture,

    pub TriggerMultiFrameCapture: pRENDERDOC_TriggerMultiFrameCapture,
}
