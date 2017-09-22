// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMObject;
use Error;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSValue(Object<ffi::WebKitDOMCSSValue>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_value_get_type(),
    }
}

pub trait DOMCSSValueExt {
    fn get_css_text(&self) -> Option<String>;

    fn get_css_value_type(&self) -> libc::c_ushort;

    fn set_css_text(&self, value: &str) -> Result<(), Error>;

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMCSSValue> + IsA<glib::object::Object>> DOMCSSValueExt for O {
    fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_value_get_css_text(self.to_glib_none().0))
        }
    }

    fn get_css_value_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_css_value_get_css_value_type(self.to_glib_none().0)
        }
    }

    fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_value_set_css_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::css-text",
                transmute(notify_css_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::css-value-type",
                transmute(notify_css_value_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_css_text_trampoline<P>(this: *mut ffi::WebKitDOMCSSValue, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSValue> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSValue::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_css_value_type_trampoline<P>(this: *mut ffi::WebKitDOMCSSValue, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSValue> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSValue::from_glib_borrow(this).downcast_unchecked())
}
