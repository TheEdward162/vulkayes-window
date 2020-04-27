//! Provides platform
//! specific glue between windows and Vulkan.

use std::ffi::{c_void, CStr};

use ash::vk;

use vulkayes_core::ash;

/// Controls whether Xcb is used over Xlib on unix platforms by default.
pub const UNIX_USE_XCB_DEFAULT: bool = cfg!(feature = "unix_use_xcb_default");

#[cfg(feature = "raw_window_handle")]
pub mod raw_window_handle;

#[cfg(feature = "winit_window")]
pub mod winit;

#[cfg(feature = "minifb_window")]
pub mod minifb;

/// ### Safety
///
/// * `ns_window` must be a valid NSWindow handle.
/// * `ns_view` must be a valid NSView handle.
/// * `instance` must be a valid Vulkan instance.
///
/// Note that while this function can be called on any platform, it will only
/// perform all necessary setup in conditional compilation on `macOS`.
#[allow(unused_variables)]
pub unsafe fn from_raw_macos(
	ns_window: *mut c_void,
	ns_view: *const c_void,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	#[cfg(target_os = "macos")]
	let layer = {
		use cocoa::appkit::{NSView, NSWindow};

		let window_handle: cocoa::base::id = std::mem::transmute(ns_window);
		let view = window_handle.contentView();

		let layer = metal::CoreAnimationLayer::new();
		layer.set_edge_antialiasing_mask(0);
		layer.set_presents_with_transaction(false);
		layer.remove_all_animations();
		layer.set_contents_scale(view.backingScaleFactor());

		view.setLayer(std::mem::transmute(layer.as_ref()));
		view.setWantsLayer(cocoa::base::YES);

		layer
	};
	#[cfg(target_os = "macos")]
	let ns_view: *const c_void = std::mem::transmute(layer.as_ref());

	#[cfg(not(target_os = "macos"))]
	{
		vulkayes_core::log::error!("Cannot initialize CAMetalLayer on this platform");
	}

	let mut create_info = vk::MacOSSurfaceCreateInfoMVK::builder();
	create_info.p_view = ns_view;

	vulkayes_core::log::info!("Creating macOS surface");
	let loader = ash::extensions::mvk::MacOSSurface::new(entry, instance);
	let surface = loader.create_mac_os_surface_mvk(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_macos() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::mvk::MacOSSurface::name()
	]
}


/// ### Safety
///
/// * `x11_window` must be a valid X11 Window handle.
/// * `x11_display` must be a valid X11 Display handle.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn from_raw_xlib(
	x11_window: vk::Window,
	x11_display: *mut vk::Display,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::XlibSurfaceCreateInfoKHR::builder()
		.window(x11_window)
		.dpy(x11_display);

	vulkayes_core::log::info!("Creating Xlib surface");
	let loader = ash::extensions::khr::XlibSurface::new(entry, instance);
	let surface = loader.create_xlib_surface(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_xlib() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::khr::XlibSurface::name()
	]
}

/// ### Safety
///
/// * `xcb_window` must be a valid Xcb window.
/// * `xcb_connection` must be a valid Xcb connection.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn from_raw_xcb(
	xcb_window: vk::xcb_window_t,
	xcb_connection: *mut vk::xcb_connection_t,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::XcbSurfaceCreateInfoKHR::builder()
		.connection(xcb_connection)
		.window(xcb_window);

	vulkayes_core::log::info!("Creating Xcb surface");
	let loader = ash::extensions::khr::XcbSurface::new(entry, instance);
	let surface = loader.create_xcb_surface(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_xcb() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::khr::XcbSurface::name()
	]
}

/// ### Safety
///
/// * `surface` must be a valid Wayland surface handle.
/// * `display` must be a valid Wayland display handle.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn from_raw_wayland(
	surface: *mut vk::wl_surface,
	display: *mut vk::wl_display,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::WaylandSurfaceCreateInfoKHR::builder()
		.display(display)
		.surface(surface);

	vulkayes_core::log::info!("Creating Wayland surface");
	let loader = ash::extensions::khr::WaylandSurface::new(entry, instance);
	let surface = loader.create_wayland_surface(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_wayland() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::khr::WaylandSurface::name()
	]
}

/// ### Safety
///
/// * `hwnd` must be a valid Win32 window handle.
/// * `hinstance` must be a valid Win32 instance.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn from_raw_win32(
	hwnd: vk::HWND,
	hinstance: vk::HINSTANCE,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::Win32SurfaceCreateInfoKHR::builder()
		.hwnd(hwnd)
		.hinstance(hinstance);

	vulkayes_core::log::info!("Creating Win32 surface");
	let loader = ash::extensions::khr::Win32Surface::new(entry, instance);
	let surface = loader.create_win32_surface(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_win32() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::khr::Win32Surface::name()
	]
}

/// ### Safety
///
/// * `window_handle` must be a valid UIWindow handle.
/// * `view_handle` must be a valid UIView handle.
/// * `instance` must be a valid Vulkan instance.
///
/// Note that while this function can be called on anu platform, it will only
/// perform all necessary setup in conditional compilation on `ios`.
#[allow(unused_variables)]
pub unsafe fn from_raw_ios(
	ui_window: *mut c_void,
	ui_view: *const c_void,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	#[cfg(target_os = "ios")]
	{
		// 	use cocoa::appkit::{NSView, NSWindow};
		//
		// 	let window_handle: cocoa::base::id = std::mem::transmute(window_handle);
		// 	let view = window_handle.contentView();
		//
		let layer = metal::CoreAnimationLayer::new();
		layer.set_edge_antialiasing_mask(0);
		layer.set_presents_with_transaction(false);
		layer.remove_all_animations();
		// layer.set_contents_scale(view.backingScaleFactor());

		unimplemented!("Missing initialization for CAMetalLayer")
		// 	view.setLayer(
		// 		std::mem::transmute(layer.as_ref())
		// 	);
		// 	view.setWantsLayer(cocoa::base::YES);
	}
	#[cfg(not(target_os = "ios"))]
	{
		vulkayes_core::log::error!("Cannot initialize CAMetalLayer on this platform");
	}

	let mut create_info = vk::IOSSurfaceCreateInfoMVK::builder();
	create_info.p_view = ui_view;

	vulkayes_core::log::info!("Creating iOS surface");
	let loader = ash::extensions::mvk::IOSSurface::new(entry, instance);
	let surface = loader.create_ios_surface_mvk(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_ios() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::mvk::IOSSurface::name()
	]
}

/// ### Safety
///
/// * `window` must be a valid Android native window handle.
/// * `instance` must be a valid Vulkan instance.
pub unsafe fn from_raw_android(
	window: *mut vk::ANativeWindow,
	entry: &ash::Entry,
	instance: &ash::Instance,
	allocation_callbacks: Option<&vk::AllocationCallbacks>
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
	let create_info = vk::AndroidSurfaceCreateInfoKHR::builder().window(window);

	vulkayes_core::log::info!("Creating Android surface");
	let loader = ash::extensions::khr::AndroidSurface::new(entry, instance);
	let surface = loader.create_android_surface(&create_info, allocation_callbacks)?;

	Ok(surface)
}
pub fn required_extensions_android() -> [&'static CStr; 2] {
	[
		ash::extensions::khr::Surface::name(),
		ash::extensions::khr::AndroidSurface::name()
	]
}
