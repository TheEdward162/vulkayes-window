//! Provides platform specific glue between windows and Vulkan.

use std::ffi::c_void;
use vulkayes_core::ash;
use ash::vk;

pub const UNIX_USE_XCB_DEFAULT: bool = cfg!(feature = "unix_use_xcb_default");

#[cfg(feature = "winit")]
pub mod winit;

/// ### Safety
///
/// * `window_handle` must be a valid NSWindow handle.
/// * `view_handle` must be a valid NSView handle. 
///
/// Note that while this function can be called on platforms different than macos, it will only 
/// perform all necessary setup on in conditional compilation on macos.
#[allow(unused_variables)]
pub unsafe fn raw_surface_cocoa(
	window_handle: *mut c_void,
	view_handle: *const c_void,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	
	#[cfg(target_os = "macos")]
	{
		use cocoa::appkit::{NSView, NSWindow};

		let window_handle: cocoa::base::id = std::mem::transmute(window_handle);
		let view = window_handle.contentView();

		let layer = metal::CoreAnimationLayer::new();
		layer.set_edge_antialiasing_mask(0);
		layer.set_presents_with_transaction(false);
		layer.remove_all_animations();
		layer.set_contents_scale(view.backingScaleFactor());

		view.setLayer(
			std::mem::transmute(layer.as_ref())
		);
		view.setWantsLayer(cocoa::base::YES);
	}

	let mut create_info = vk::MacOSSurfaceCreateInfoMVK::builder();
	create_info.p_view = view_handle;

	vulkayes_core::log::info!("Creating macOS surface");
	let loader = ash::extensions::mvk::MacOSSurface::new(entry, instance);
	let surface = loader.create_mac_os_surface_mvk(
		&create_info, allocation_callbacks
	)?;

	Ok(surface)
}

pub fn required_surface_extensions_cocoa() -> [&'static str; 2] {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::mvk::MacOSSurface::name().to_str().unwrap()
	]
}


/// ### Safety
///
/// * `x11_window` must be a valid X11 Window handle.
/// * `x11_display` must be a valid X11 Display handle. 
pub unsafe fn raw_surface_xlib(
	x11_window: vk::Window,
	x11_display: *mut vk::Display,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::XlibSurfaceCreateInfoKHR::builder()
		.window(x11_window)
		.dpy(x11_display)
	;

	vulkayes_core::log::info!("Creating Xlib surface");
	let loader = ash::extensions::khr::XlibSurface::new(
		entry,
		instance
	);
	let surface = loader.create_xlib_surface(
		&create_info,
		allocation_callbacks
	)?;

	Ok(surface)
}
pub fn required_surface_extensions_xlib() -> [&'static str; 2] {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::khr::XlibSurface::name().to_str().unwrap()
	]
}

/// ### Safety
///
/// * `xcb_connection` must be a valid Xcb connection.
/// * `xcb_window` must be a valid Xcb window.
pub unsafe fn raw_surface_xcb(
	xcb_connection: *mut vk::xcb_connection_t,
	xcb_window: vk::xcb_window_t,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::XcbSurfaceCreateInfoKHR::builder()
		.connection(xcb_connection)
		.window(xcb_window)
	;

	vulkayes_core::log::info!("Creating Xcb surface");
	let loader = ash::extensions::khr::XcbSurface::new(
		entry,
		instance
	);
	let surface = loader.create_xcb_surface(
		&create_info,
		allocation_callbacks
	)?;

	Ok(surface)
}
pub fn required_surface_extensions_xcb() -> [&'static str; 2] {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::khr::XcbSurface::name().to_str().unwrap()
	]
}

/// ### Safety
///
/// * `display` must be a valid Wayland display handle.
/// * `surface` must be a valid Wayland surface handle.
pub unsafe fn raw_surface_wayland(
	display: *mut vk::wl_display,
	surface: *mut vk::wl_surface,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::WaylandSurfaceCreateInfoKHR::builder()
		.display(display)
		.surface(surface)
	;

	vulkayes_core::log::info!("Creating Wayland surface");
	let loader = ash::extensions::khr::WaylandSurface::new(
		entry,
		instance
	);
	let surface = loader.create_wayland_surface(
		&create_info,
		allocation_callbacks
	)?;

	Ok(surface)
}
pub fn required_surface_extensions_wayland() -> [&'static str; 2] {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::khr::WaylandSurface::name().to_str().unwrap()
	]
}

/// ### Safety
///
/// `instance` must be a valid Win32 instance.
/// `window_handle` must be a valid Win32 window handle.
pub unsafe fn raw_surface_win32(
	window_instance: vk::HINSTANCE,
	window_handle: vk::HWND,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::Win32SurfaceCreateInfoKHR::builder()
		.hinstance(window_instance)
		.hwnd(window_handle)
	;

	vulkayes_core::log::info!("Creating Win32 surface");
	let loader = ash::extensions::khr::Win32Surface::new(
		entry,
		instance
	);
	let surface = loader.create_win32_surface(
		&create_info,
		allocation_callbacks
	)?;

	Ok(surface)
}

pub fn required_surface_extensions_win32() -> [&'static str; 2] {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::khr::Win32Surface::name().to_str().unwrap()
	]
}