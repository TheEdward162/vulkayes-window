pub use winit;

pub use inner::*;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
mod inner {
	use std::ops::Deref;

	use winit::window::Window;

	use vulkayes_core::{
		instance::Instance,
		surface::{error::SurfaceError, Surface},
		Vrc
	};

	pub fn create_surface(
		instance: Vrc<Instance>,
		window: &Window,
		host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
	) -> Result<Surface, SurfaceError> {
		create_surface_ext(instance, window, host_memory_allocator, crate::UNIX_USE_XCB_DEFAULT)
	}
	pub fn required_surface_extensions(window: &Window) -> impl AsRef<[&'static str]> {
		required_surface_extensions_ext(window, crate::UNIX_USE_XCB_DEFAULT)
	}

	pub fn create_surface_ext(
		instance: Vrc<Instance>,
		window: &Window,
		host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator,
		use_xcb: bool
	) -> Result<Surface, SurfaceError> {
		use winit::platform::unix::WindowExtUnix;

		if let (Some(wayland_display), Some(wayland_surface)) = (window.wayland_display(), window.wayland_surface()) {
			let surface = unsafe {
				crate::raw_surface_wayland(
					wayland_display,
					wayland_surface,
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


		if let Some(x11_window) = window.xlib_window() {
			let surface = if use_xcb {
				 unsafe {
					crate::raw_surface_xcb(
						window.xcb_connection().unwrap(),
						x11_window as _,
						instance.entry().deref(),
						instance.deref().deref(),
						host_memory_allocator.as_ref()
					)? 
				}
			} else {
				unsafe {
					crate::raw_surface_xlib(
						x11_window,
						window.xlib_display().unwrap() as *mut _,
						instance.entry().deref(),
						instance.deref().deref(),
						host_memory_allocator.as_ref()
					)? 
				}
			};

			let vy_surface = unsafe {
				vulkayes_core::surface::Surface::from_existing(instance, surface, host_memory_allocator)
			};

			return Ok(vy_surface)
		}

		unimplemented!("Only implemented for Wayland, Xlib and Xcb")
	}
	pub fn required_surface_extensions_ext(window: &Window, use_xcb: bool) -> impl AsRef<[&'static str]> {
		use winit::platform::unix::WindowExtUnix;
		
		if let (Some(_), Some(_)) = (window.wayland_display(), window.wayland_surface()) {
			return crate::required_surface_extensions_wayland();
		}

		if let Some(_) = window.xlib_window() {
			if use_xcb {
				return crate::required_surface_extensions_xcb();
			} else {
				return crate::required_surface_extensions_xlib();
			}
		}

		unimplemented!("Only implemented for Wayland, Xlib and Xcb")
	}
}

#[cfg(target_os = "macos")]
mod inner {
	use std::ops::Deref;

	use winit::window::Window;

	use vulkayes_core::{
		instance::Instance,
		surface::{error::SurfaceError, Surface},
		Vrc
	};

	pub fn create_surface(
		instance: Vrc<Instance>,
		window: &Window,
		host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
	) -> Result<Surface, SurfaceError> {
		use winit::platform::macos::WindowExtMacOS;

		let surface = unsafe {
			crate::raw_surface_cocoa(
				std::mem::transmute(window.ns_window()),
				std::mem::transmute(window.ns_view()),
				instance.entry().deref(),
				instance.deref().deref(),
				host_memory_allocator.as_ref()
			)?
		};

		let vy_surface = unsafe {
			vulkayes_core::surface::Surface::from_existing(
				instance,
				surface,
				host_memory_allocator
			)
		};

		Ok(
			vy_surface
		)
	}
	pub fn required_surface_extensions(window: &Window) -> [&'static str; 2] {
		crate::required_surface_extensions()
	}
}

#[cfg(target_os = "windows")]
mod inner {
	use winit::window::Window;

	use vulkayes_core::{
		instance::Instance,
		surface::{error::SurfaceError, Surface},
		Vrc
	};

	unsafe fn create_surface(
		instance: Vrc<Instance>,
		window: &Window,
		host_memory_allocator: vulkayes_core::memory::host::HostMemoryAllocator
	) -> Result<Surface, SurfaceError> {
		use winapi::{shared::windef::HWND, um::libloaderapi::GetModuleHandleW};
		use winit::os::windows::WindowExt;

		let hinstance = GetModuleHandleW(std::ptr::null()) as *const c_void;
		let hwnd = window.get_hwnd() as HWND;

		let surface = unsafe {
			crate::raw_surface_win32(
				hinstance,
				hwnd,
				instance.entry().deref(),
				instance.deref().deref(),
				host_memory_allocator.as_ref()
			)?
		};

		let vy_surface = unsafe {
			vulkayes_core::surface::Surface::from_existing(
				instance,
				surface,
				host_memory_allocator
			)
		};

		Ok(
			vy_surface
		)
	}
}
