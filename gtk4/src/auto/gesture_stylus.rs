// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct GestureStylus(Object<ffi::GtkGestureStylus, ffi::GtkGestureStylusClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_stylus_get_type(),
    }
}

impl GestureStylus {
    #[doc(alias = "gtk_gesture_stylus_new")]
    pub fn new() -> GestureStylus {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_stylus_new()).unsafe_cast() }
    }

    //#[doc(alias = "gtk_gesture_stylus_get_axes")]
    //pub fn get_axes(&self, axes: /*Unimplemented*/&CArray TypeId { ns_id: 11, id: 4 }, values: Vec<f64>) -> bool {
    //    unsafe { TODO: call ffi:gtk_gesture_stylus_get_axes() }
    //}

    #[doc(alias = "gtk_gesture_stylus_get_axis")]
    pub fn get_axis(&self, axis: gdk::AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_axis(
                self.to_glib_none().0,
                axis.to_glib(),
                value.as_mut_ptr(),
            ));
            let value = value.assume_init();
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_backlog")]
    pub fn get_backlog(&self) -> Option<Vec<gdk::TimeCoord>> {
        unsafe {
            let mut backlog = ptr::null_mut();
            let mut n_elems = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_backlog(
                self.to_glib_none().0,
                &mut backlog,
                n_elems.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    backlog,
                    n_elems.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_device_tool")]
    pub fn get_device_tool(&self) -> Option<gdk::DeviceTool> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_stylus_get_device_tool(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn connect_down<F: Fn(&GestureStylus, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_motion<F: Fn(&GestureStylus, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_proximity<F: Fn(&GestureStylus, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn proximity_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"proximity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    proximity_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_up<F: Fn(&GestureStylus, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureStylus {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GestureStylus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureStylus")
    }
}
