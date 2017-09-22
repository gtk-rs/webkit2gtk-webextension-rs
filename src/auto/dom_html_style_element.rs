// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use DOMStyleSheet;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLStyleElement(Object<ffi::WebKitDOMHTMLStyleElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_style_element_get_type(),
    }
}

pub trait DOMHTMLStyleElementExt {
    fn get_disabled(&self) -> bool;

    fn get_media(&self) -> Option<String>;

    fn get_sheet(&self) -> Option<DOMStyleSheet>;

    fn get_type_attr(&self) -> Option<String>;

    fn set_disabled(&self, value: bool);

    fn set_media(&self, value: &str);

    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMHTMLStyleElement> + IsA<glib::object::Object>> DOMHTMLStyleElementExt for O {
    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_style_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_media(self.to_glib_none().0))
        }
    }

    fn get_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_sheet(self.to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_type_attr(self.to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_media(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_media(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disabled",
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::media",
                transmute(notify_media_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sheet",
                transmute(notify_sheet_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLStyleElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLStyleElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLStyleElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_media_trampoline<P>(this: *mut ffi::WebKitDOMHTMLStyleElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLStyleElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLStyleElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sheet_trampoline<P>(this: *mut ffi::WebKitDOMHTMLStyleElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLStyleElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLStyleElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLStyleElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLStyleElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLStyleElement::from_glib_borrow(this).downcast_unchecked())
}
