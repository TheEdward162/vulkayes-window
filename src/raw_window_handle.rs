use std::ops::Deref;

use ash::vk;
use vulkayes_core::{
	ash,
	prelude::{Instance, Surface, Vrc},
	surface::error::SurfaceError
};

pub use raw_window_handle;
use raw_window_handle::RawWindowHandle;

/// ### Safety
///
/// `handle` must contain a valid window handle.
pub unsafe fn create_surface(
	instance: Vrc<Instance>,
	handle: RawWindowHandle,
	host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
) -> Result<Surface, SurfaceError> {
	let surface = create_surface_raw(
		handle,
		instance.entry().deref(),
		instance.deref().deref(),
		host_memory_allocator.as_ref()
	)?;

	let vy_surface =
		vulkayes_core::surface::Surface::from_existing(instance, surface, host_memory_allocator);

	return Ok(vy_surface)
}
/// ### Safety
///
/// * `handle` must contain a valid window handle.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn create_surface_raw(
	handle: RawWindowHandle,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	match handle {
		#[cfg(target_os = "macos")]
		RawWindowHandle::MacOS(handle) => crate::from_raw_macos(
			handle.ns_window,
			handle.ns_view,
			entry,
			instance,
			allocation_callbacks
		),

		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Xlib(handle) => crate::from_raw_xlib(
			handle.window,
			handle.display as *mut _,
			entry,
			instance,
			allocation_callbacks
		),
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Xcb(handle) => crate::from_raw_xcb(
			handle.window,
			handle.connection,
			entry,
			instance,
			allocation_callbacks
		),
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Wayland(handle) => crate::from_raw_wayland(
			handle.surface,
			handle.display,
			entry,
			instance,
			allocation_callbacks
		),

		#[cfg(target_os = "windows")]
		RawWindowHandle::Windows(handle) => crate::from_raw_win32(
			handle.hwnd,
			handle.hinstance,
			entry,
			instance,
			allocation_callbacks
		),

		#[cfg(target_os = "ios")]
		RawWindowHandle::IOS(handle) => crate::from_raw_ios(
			handle.ui_window,
			handle.ui_view,
			entry,
			instance,
			allocation_callbacks
		),
		#[cfg(target_os = "android")]
		RawWindowHandle::Android(handle) => crate::from_raw_android(
			handle.a_native_window,
			entry,
			instance,
			allocation_callbacks
		),

		// TODO: Can this be done currently?
		#[cfg(target_arch = "wasm32")]
		RawWindowHandle::Web(_) => unimplemented!("Not implemented for this platform"),

		_ => unimplemented!("Not implemented for this platform")
	}
}
pub fn required_extensions(handle: RawWindowHandle) -> [&'static str; 2] {
	match handle {
		#[cfg(target_os = "macos")]
		RawWindowHandle::MacOS(_) => crate::required_extensions_macos(),

		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Xlib(_) => crate::required_extensions_xlib(),
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Xcb(_) => crate::required_extensions_xcb(),
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Wayland(_) => crate::required_extensions_wayland(),

		#[cfg(target_os = "windows")]
		RawWindowHandle::Windows(_) => crate::required_extensions_win32(),

		#[cfg(target_os = "ios")]
		RawWindowHandle::IOS(_) => crate::required_extensions_ios(),
		#[cfg(target_os = "android")]
		RawWindowHandle::Android(_) => crate::required_extensions_android(),

		#[cfg(target_arch = "wasm32")]
		RawWindowHandle::Web(_) => unimplemented!("Not implemented for this platform"),

		_ => unimplemented!("Not implemented for this platform")
	}
}
