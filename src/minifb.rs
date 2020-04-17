use std::ops::Deref;

use ash::vk;
use vulkayes_core::{
	ash,
	prelude::{Instance, Surface, Vrc},
	surface::error::SurfaceError
};

use raw_window_handle::HasRawWindowHandle;

pub use minifb;
use minifb::Window;

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

	let vy_surface = unsafe {
		vulkayes_core::surface::Surface::from_existing(instance, surface, host_memory_allocator)
	};

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
		window.raw_window_handle(),
		entry,
		instance,
		allocation_callbacks
	)
}

pub fn required_extensions(window: &Window) -> [&'static str; 2] {
	crate::raw_window_handle::required_extensions(window.raw_window_handle())
}
