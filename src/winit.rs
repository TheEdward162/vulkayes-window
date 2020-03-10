use std::{ffi::c_void, ops::Deref};

use vulkayes_core::{
	ash::{self, vk},
	instance::Instance,
	surface::{error::SurfaceError, Surface},
	Vrc
};

pub use winit;
use winit::window::Window;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn create_surface(
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


#[cfg(target_os = "macos")]
pub fn create_surface(
	instance: Vrc<Instance>,
	window: &Window,
	host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
) -> Result<Surface, SurfaceError> {
	use cocoa::appkit::{NSView, NSWindow};
	use winit::platform::macos::WindowExtMacOS;

	unsafe {
		let handle: cocoa::base::id = std::mem::transmute(window.ns_window());
		let view = handle.contentView();

		let layer = metal::CoreAnimationLayer::new();
		layer.set_edge_antialiasing_mask(0);
		layer.set_presents_with_transaction(false);
		layer.remove_all_animations();
		layer.set_contents_scale(view.backingScaleFactor());

		view.setLayer(std::mem::transmute(layer.as_ref()));
		view.setWantsLayer(objc::runtime::YES);
	}

	let create_info = vk::MacOSSurfaceCreateInfoMVK {
		s_type: vk::StructureType::MACOS_SURFACE_CREATE_INFO_M,
		p_next: std::ptr::null(),
		flags: Default::default(),
		p_view: window.ns_view() as *const c_void
	};

	let macos_surface_loader =
		ash::extensions::mvk::MacOSSurface::new(instance.entry().deref(), instance.deref().deref());

	let allocation_callbacks: Option<vk::AllocationCallbacks> = host_memory_allocator.into();

	unsafe {
		let surface = macos_surface_loader
			.create_mac_os_surface_mvk(&create_info, allocation_callbacks.as_ref())?;

		Ok(vulkayes_core::surface::Surface::from_existing(
			instance,
			surface,
			allocation_callbacks
		))
	}
}
#[cfg(target_os = "macos")]
pub fn required_surface_extensions() -> impl AsRef<[&'static str]> {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::mvk::MacOSSurface::name().to_str().unwrap()
	]
}

#[cfg(target_os = "windows")]
unsafe fn create_surface(
	instance: Vrc<Instance>,
	window: &Window,
	host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
) -> Result<Surface, SurfaceError> {
	unimplemented!() // TODO: Implement and test on platform
	             // use std::ptr;
	             // use winapi::{shared::windef::HWND, um::libloaderapi::GetModuleHandleW};
	             // use winit::os::windows::WindowExt;
	             //
	             // let hwnd = window.get_hwnd() as HWND;
	             // let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
	             // let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
	             // 	s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
	             // 	p_next: ptr::null(),
	             // 	flags: Default::default(),
	             // 	hinstance,
	             // 	hwnd: hwnd as *const c_void
	             // };
	             // let win32_surface_loader = Win32Surface::new(entry, instance);
	             // win32_surface_loader.create_win32_surface(&win32_create_info, None)
}

#[cfg(all(windows))]
pub fn required_surface_extensions() -> impl AsRef<[&'static str]> {
	[
		ash::extensions::khr::Surface::name().to_str().unwrap(),
		ash::extensions::mvk::Win32Surface::name().to_str().unwrap()
	]
}
