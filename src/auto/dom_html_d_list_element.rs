// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLDListElement(Object<webkit2_webextension_sys::WebKitDOMHTMLDListElement, webkit2_webextension_sys::WebKitDOMHTMLDListElementClass, DOMHTMLDListElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_d_list_element_get_type(),
    }
}

pub const NONE_DOMHTMLD_LIST_ELEMENT: Option<&DOMHTMLDListElement> = None;

pub trait DOMHTMLDListElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_compact(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_compact(&self, value: bool);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDListElement>> DOMHTMLDListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(
                webkit2_webextension_sys::webkit_dom_html_d_list_element_get_compact(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_d_list_element_set_compact(
                self.as_ref().to_glib_none().0,
                value.to_glib(),
            );
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compact_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLDListElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLDListElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDListElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::compact\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_compact_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLDListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLDListElement")
    }
}
