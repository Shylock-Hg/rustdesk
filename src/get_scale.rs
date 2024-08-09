use std::{ffi::c_void, os::raw::c_schar, ptr};

#[cfg(target_os = "linux")]
use gdk4;
#[cfg(target_os = "linux")]
use gtk4;
#[cfg(target_os = "linux")]
use gdk4::prelude::{MonitorExt, DisplayExt, ListModelExt, Cast};
use scrap::libc::c_int;
#[cfg(target_os = "linux")]
use wayland_sys;


type logical_position_fn = extern fn(*mut c_void, c_int, c_int);
type logical_size_fn = extern fn(*mut c_void, *mut c_void, c_int, c_int);
type done_fn =  extern fn(*mut c_void, *mut c_void);
type name_fn =  extern fn(*mut c_void, *mut c_void, *const c_schar);
type description_fn =  extern fn(*mut c_void, *mut c_void, *const c_schar);
#[repr(C)]
struct zxdg_output_v1_listener {
	/**
	 * position of the output within the global compositor space
	 *
	 * The position event describes the location of the wl_output
	 * within the global compositor space.
	 *
	 * The logical_position event is sent after creating an xdg_output
	 * (see xdg_output_manager.get_xdg_output) and whenever the
	 * location of the output changes within the global compositor
	 * space.
	 * @param x x position within the global compositor space
	 * @param y y position within the global compositor space
	 */
	// void (*logical_position)(void *data,
				 // struct zxdg_output_v1 *zxdg_output_v1,
				 // int32_t x,
				 // int32_t y);
    logical_position: logical_position_fn,

	/**
	 * size of the output in the global compositor space
	 *
	 * The logical_size event describes the size of the output in the
	 * global compositor space.
	 *
	 * Most regular Wayland clients should not pay attention to the
	 * logical size and would rather rely on xdg_shell interfaces.
	 *
	 * Some clients such as Xwayland, however, need this to configure
	 * their surfaces in the global compositor space as the compositor
	 * may apply a different scale from what is advertised by the
	 * output scaling property (to achieve fractional scaling, for
	 * example).
	 *
	 * For example, for a wl_output mode 3840×2160 and a scale factor
	 * 2:
	 *
	 * - A compositor not scaling the monitor viewport in its
	 * compositing space will advertise a logical size of 3840×2160,
	 *
	 * - A compositor scaling the monitor viewport with scale factor 2
	 * will advertise a logical size of 1920×1080,
	 *
	 * - A compositor scaling the monitor viewport using a fractional
	 * scale of 1.5 will advertise a logical size of 2560×1440.
	 *
	 * For example, for a wl_output mode 1920×1080 and a 90 degree
	 * rotation, the compositor will advertise a logical size of
	 * 1080x1920.
	 *
	 * The logical_size event is sent after creating an xdg_output (see
	 * xdg_output_manager.get_xdg_output) and whenever the logical size
	 * of the output changes, either as a result of a change in the
	 * applied scale or because of a change in the corresponding output
	 * mode(see wl_output.mode) or transform (see wl_output.transform).
	 * @param width width in global compositor space
	 * @param height height in global compositor space
	 */
	// void (*logical_size)(void *data,
			     // struct zxdg_output_v1 *zxdg_output_v1,
			     // int32_t width,
			     // int32_t height);
    // @param data
    // @param zxdg_output
    // @param width
    // @param height
    logical_size: logical_size_fn,

	/**
	 * all information about the output have been sent
	 *
	 * This event is sent after all other properties of an xdg_output
	 * have been sent.
	 *
	 * This allows changes to the xdg_output properties to be seen as
	 * atomic, even if they happen via multiple events.
	 *
	 * For objects version 3 onwards, this event is deprecated.
	 * Compositors are not required to send it anymore and must send
	 * wl_output.done instead.
	 */
	// void (*done)(void *data,
		     // struct zxdg_output_v1 *zxdg_output_v1);
    done: done_fn,
	/**
	 * name of this output
	 *
	 * Many compositors will assign names to their outputs, show them
	 * to the user, allow them to be configured by name, etc. The
	 * client may wish to know this name as well to offer the user
	 * similar behaviors.
	 *
	 * The naming convention is compositor defined, but limited to
	 * alphanumeric characters and dashes (-). Each name is unique
	 * among all wl_output globals, but if a wl_output global is
	 * destroyed the same name may be reused later. The names will also
	 * remain consistent across sessions with the same hardware and
	 * software configuration.
	 *
	 * Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc.
	 * However, do not assume that the name is a reflection of an
	 * underlying DRM connector, X11 connection, etc.
	 *
	 * The name event is sent after creating an xdg_output (see
	 * xdg_output_manager.get_xdg_output). This event is only sent once
	 * per xdg_output, and the name does not change over the lifetime
	 * of the wl_output global.
	 *
	 * This event is deprecated, instead clients should use
	 * wl_output.name. Compositors must still support this event.
	 * @param name output name
	 * @since 2
	 */
	// void (*name)(void *data,
		     // struct zxdg_output_v1 *zxdg_output_v1,
		     // const char *name);
    name: name_fn,
	/**
	 * human-readable description of this output
	 *
	 * Many compositors can produce human-readable descriptions of
	 * their outputs. The client may wish to know this description as
	 * well, to communicate the user for various purposes.
	 *
	 * The description is a UTF-8 string with no convention defined for
	 * its contents. Examples might include 'Foocorp 11" Display' or
	 * 'Virtual X11 output via :1'.
	 *
	 * The description event is sent after creating an xdg_output (see
	 * xdg_output_manager.get_xdg_output) and whenever the description
	 * changes. The description is optional, and may not be sent at
	 * all.
	 *
	 * For objects of version 2 and lower, this event is only sent once
	 * per xdg_output, and the description does not change over the
	 * lifetime of the wl_output global.
	 *
	 * This event is deprecated, instead clients should use
	 * wl_output.description. Compositors must still support this
	 * event.
	 * @param description output description
	 * @since 2
	 */
	// void (*description)(void *data,
			    // struct zxdg_output_v1 *zxdg_output_v1,
			    // const char *description);
    description: description_fn,
}

#[repr(C)]
struct zxdg_output_v1;

extern fn logical_size_callback(_: *mut c_void, _: *mut c_void, width: c_int, height: c_int) {
    println!("DEBUG POINT: width is: {}, height is: {}", width, height);
}

fn main() {
    // #[cfg(target_os = "linux")]
    // {
        // let _ = gtk4::init();
        // let display = gdk4::Display::default().unwrap();
        // let monitor = display.monitors().item(0).unwrap().downcast::<gdk4::Monitor>().unwrap();
        // let scale = monitor.scale();
        // print!("{}", scale);
    // }
    let mut listener = zxdg_output_v1_listener {
        logical_position: unsafe { std::mem::transmute::<*mut c_void, logical_position_fn>(ptr::null_mut()) },
        logical_size: logical_size_callback,
        done: unsafe {
            std::mem::transmute::<*mut c_void, done_fn>(ptr::null_mut())
        },
        name: unsafe {
            std::mem::transmute::<*mut c_void, name_fn>(ptr::null_mut())
        },
        description: unsafe {
            std::mem::transmute::<*mut c_void, description_fn>(ptr::null_mut())
        },
    };
    let mut proxy = zxdg_output_v1;
    // unsafe  {
        // wayland_sys::client::wl_proxy_create(std::mem::transmute::<&mut zxdg_output_v1, *mut wayland_sys::client::wl_proxy>(&mut proxy),
            // std::mem::transmute::<*mut c_void, *mut wayland_sys::common::wl_interface>(ptr::null_mut()));
    // }
    let ret = unsafe {
        let p = std::mem::transmute::<&mut zxdg_output_v1, *mut wayland_sys::client::wl_proxy>(&mut proxy);
        wayland_sys::client::wl_proxy_destroy(p);
        wayland_sys::client::wl_proxy_add_listener(p,
            std::mem::transmute::<&mut zxdg_output_v1_listener, *mut(extern fn())>(&mut listener),
            ptr::null_mut())
    };
    println!("DEBUG POINT: add ret: {}", ret);
}
