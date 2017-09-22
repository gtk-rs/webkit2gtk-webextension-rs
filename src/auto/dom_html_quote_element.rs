// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
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
    pub struct DOMHTMLQuoteElement(Object<ffi::WebKitDOMHTMLQuoteElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_quote_element_get_type(),
    }
}

pub trait DOMHTMLQuoteElementExt {
    fn get_cite(&self) -> Option<String>;

    fn set_cite(&self, value: &str);

    fn connect_property_cite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMHTMLQuoteElement> + IsA<glib::object::Object>> DOMHTMLQuoteElementExt for O {
    fn get_cite(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_quote_element_get_cite(self.to_glib_none().0))
        }
    }

    fn set_cite(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_quote_element_set_cite(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_cite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cite",
                transmute(notify_cite_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_cite_trampoline<P>(this: *mut ffi::WebKitDOMHTMLQuoteElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLQuoteElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLQuoteElement::from_glib_borrow(this).downcast_unchecked())
}
