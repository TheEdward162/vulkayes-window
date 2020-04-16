use std::ffi::c_void;

use vulkayes_core::ash::{self, vk};

#[cfg(feature = "winit")]
pub mod winit;

#[cfg(target_os = "macos")]
pub unsafe fn raw_surface(
	window_handle: cocoa::base::id,
	view_handle: cocoa::base::id,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	use cocoa::appkit::{NSView, NSWindow};

	let view = window_handle.contentView();

	let layer = metal::CoreAnimationLayer::new();
	layer.set_edge_antialiasing_mask(0);
	layer.set_presents_with_transaction(false);
	layer.remove_all_animations();
	layer.set_contents_scale(view.backingScaleFactor());

	view.setLayer(std::mem::transmute(layer.as_ref()));
	view.setWantsLayer(cocoa::base::YES);

	let mut create_info = vk::MacOSSurfaceCreateInfoMVK::builder();
	create_info.p_view = view_handle as *const c_void;

	let macos_surface_loader = ash::extensions::mvk::MacOSSurface::new(entry, instance);

	let surface = macos_surface_loader.create_mac_os_surface_mvk(
		&create_info, allocation_callbacks
	)?;

	Ok(surface)
}
#[cfg(target_os = "macos")]
pub fn required_surface_extensions() -> impl AsRef<[&'static str]> {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::mvk::MacOSSurface::name().to_str().unwrap()
	]
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn raw_surface(
	instance: Vrc<Instance>,
	window: &Window,
	host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
) -> Result<Surface, SurfaceError> {
	unimplemented!() // TODO: Implement and test on platform
	// use winit::platform::unix::WindowExtUnix;

	// let x11_display = window.get_xlib_display().unwrap();
	// let x11_window = window.get_xlib_window().unwrap();

	// let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
	// 	.window(x11_window)
	// 	.dpy(x11_display as *mut vk::Display)
	// 	.build();

	// let xlib_surface_loader =
	// 	ash::extensions::khr::XlibSurface::new(instance.entry().deref(), instance.deref().deref());

	// let allocation_callbacks: Option<vk::AllocationCallbacks> = host_memory_allocator.into();

	// let surface =
	// 	xlib_surface_loader.create_xlib_surface(&x11_create_info, allocation_callbacks.as_ref())?;

	// unsafe {
	// 	Ok(vulkayes_core::surface::Surface::new(instance, surface, allocation_callbacks))
	// }
}
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn required_surface_extensions() -> impl AsRef<[&'static str]> {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::mvk::XlibSurface::name().to_str().unwrap()
	]
}