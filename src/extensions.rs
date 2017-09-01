use raw_api::{RENDERDOC_DevicePointer, RENDERDOC_InputButton, RENDERDOC_WindowHandle};

pub trait GetWindowHandle {
    unsafe fn get_window_handle(&self) -> RENDERDOC_WindowHandle;
}

impl GetWindowHandle for RENDERDOC_WindowHandle {
    #[inline(always)]
    unsafe fn get_window_handle(&self) -> RENDERDOC_WindowHandle {
        *self
    }
}

pub trait GetDevicePointer {
    unsafe fn get_device_pointer(&self) -> RENDERDOC_DevicePointer;
}

impl GetDevicePointer for RENDERDOC_DevicePointer {
    #[inline(always)]
    unsafe fn get_device_pointer(&self) -> RENDERDOC_DevicePointer {
        *self
    }
}

#[cfg(feature = "use-winit")]
mod winit_exts {
    use super::GetWindowHandle;
    use raw_api::RENDERDOC_WindowHandle;
    use winit::Window;

    #[cfg(target_os = "windows")]
    impl GetWindowHandle for Window {
        #[inline]
        unsafe fn get_window_handle(&self) -> RENDERDOC_WindowHandle {
            use winit::os::windows::WindowExt;
            self.get_hwnd() as RENDERDOC_WindowHandle
        }
    }

    #[cfg(target_os = "linux")]
    impl GetWindowHandle for Window {
        #[inline]
        unsafe fn get_window_handle(&self) -> RENDERDOC_WindowHandle {
            use winit::os::windows::WindowExt;
            self.get_x11_connection().display as RENDERDOC_WindowHandle
        }
    }
}
