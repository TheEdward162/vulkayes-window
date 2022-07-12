use std::{ffi::CStr, ops::Deref};

use ash::vk;
use raw_window_handle::HasRawWindowHandle;
use vulkayes_core::{
	ash,
	prelude::{Instance, Surface, Vrc},
	surface::error::SurfaceError
};
pub use winit;
use winit::window::Window;

pub fn create_surface(
	instance: Vrc<Instance>,
	window: &Window,
	host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
) -> Result<Surface, SurfaceError> {
	let surface = unsafe {
		create_surface_raw(
			window,
			instance.entry().deref(),
			instance.deref().deref(),
			host_memory_allocator.as_ref()
		)?
	};

	let vy_surface = unsafe { vulkayes_core::surface::Surface::from_existing(instance, surface, host_memory_allocator) };

	return Ok(vy_surface)
}

/// ### Safety
///
/// `instance` must be a valid Vulkan instance.
pub unsafe fn create_surface_raw(
	window: &Window,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	crate::raw_window_handle::create_surface_raw(
		resolve_window_handle(window),
		entry,
		instance,
		allocation_callbacks
	)
}

pub fn required_extensions(window: &Window) -> [&'static CStr; 2] {
	crate::raw_window_handle::required_extensions(resolve_window_handle(window))
}


#[allow(unreachable_code)]
fn resolve_window_handle(window: &Window) -> raw_window_handle::RawWindowHandle {
	#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
	{
		return unix::resolve_window_handle(window, crate::UNIX_USE_XCB_DEFAULT)
	}

	window.raw_window_handle()
}


#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
pub mod unix {
	use super::*;

	/// `use_xcb` controls whether to use Xcb over Xlib (doesn't affect Wayland).
	pub fn create_surface(
		instance: Vrc<Instance>,
		window: &Window,
		host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator,
		use_xcb: bool
	) -> Result<Surface, SurfaceError> {
		let surface = unsafe {
			create_surface_raw(
				window,
				instance.entry().deref(),
				instance.deref().deref(),
				host_memory_allocator.as_ref(),
				use_xcb
			)?
		};

		let vy_surface = unsafe { vulkayes_core::surface::Surface::from_existing(instance, surface, host_memory_allocator) };

		return Ok(vy_surface)
	}

	/// ### Safety
	///
	/// `instance` must be a valid Vulkan instance.
	///
	/// `use_xcb` controls whether to use Xcb over Xlib (doesn't affect Wayland).
	pub unsafe fn create_surface_raw(
		window: &Window,
		entry: &ash::Entry,
		instance: &ash::Instance,
		allocation_callbacks: Option<&vk::AllocationCallbacks>,
		use_xcb: bool
	) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
		crate::raw_window_handle::create_surface_raw(
			resolve_window_handle(window, use_xcb),
			entry,
			instance,
			allocation_callbacks
		)
	}

	/// `use_xcb` controls whether to use Xcb over Xlib (doesn't affect Wayland).
	pub fn required_extensions(window: &Window, use_xcb: bool) -> [&'static CStr; 2] {
		crate::raw_window_handle::required_extensions(resolve_window_handle(window, use_xcb))
	}

	pub(super) fn resolve_window_handle(window: &Window, use_xcb: bool) -> raw_window_handle::RawWindowHandle {
		use winit::platform::unix::WindowExtUnix;
		if use_xcb {
			if let raw_window_handle::RawWindowHandle::Xlib(_) = window.raw_window_handle() {
				return raw_window_handle::RawWindowHandle::Xcb(raw_window_handle::unix::XcbHandle {
					window: window.xlib_window().unwrap() as _,
					connection: window.xcb_connection().unwrap(),
					..raw_window_handle::unix::XcbHandle::empty()
				})
			}
		}

		window.raw_window_handle()
	}
}
