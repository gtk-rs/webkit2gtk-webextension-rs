// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMObject;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMClientRect(Object<ffi::WebKitDOMClientRect>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_client_rect_get_type(),
    }
}

pub trait DOMClientRectExt {
    #[cfg(feature = "v2_18")]
    fn get_bottom(&self) -> f32;

    #[cfg(feature = "v2_18")]
    fn get_height(&self) -> f32;

    #[cfg(feature = "v2_18")]
    fn get_left(&self) -> f32;

    #[cfg(feature = "v2_18")]
    fn get_right(&self) -> f32;

    #[cfg(feature = "v2_18")]
    fn get_top(&self) -> f32;

    #[cfg(feature = "v2_18")]
    fn get_width(&self) -> f32;

    fn get_property_bottom(&self) -> f32;

    fn get_property_height(&self) -> f32;

    fn get_property_left(&self) -> f32;

    fn get_property_right(&self) -> f32;

    fn get_property_top(&self) -> f32;

    fn get_property_width(&self) -> f32;

    fn connect_property_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMClientRect> + IsA<glib::object::Object>> DOMClientRectExt for O {
    #[cfg(feature = "v2_18")]
    fn get_bottom(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_bottom(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_18")]
    fn get_height(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_height(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_18")]
    fn get_left(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_left(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_18")]
    fn get_right(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_right(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_18")]
    fn get_top(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_top(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_18")]
    fn get_width(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_width(self.to_glib_none().0)
        }
    }

    fn get_property_bottom(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bottom".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_height(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_left(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "left".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_right(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "right".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_top(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "top".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_width(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_property_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bottom",
                transmute(notify_bottom_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::left",
                transmute(notify_left_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::right",
                transmute(notify_right_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::top",
                transmute(notify_top_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_bottom_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_left_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_right_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_top_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMClientRect> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMClientRect::from_glib_borrow(this).downcast_unchecked())
}