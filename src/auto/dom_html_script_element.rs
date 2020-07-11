// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
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
    pub struct DOMHTMLScriptElement(Object<webkit2_webextension_sys::WebKitDOMHTMLScriptElement, webkit2_webextension_sys::WebKitDOMHTMLScriptElementClass, DOMHTMLScriptElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_script_element_get_type(),
    }
}

pub const NONE_DOMHTML_SCRIPT_ELEMENT: Option<&DOMHTMLScriptElement> = None;

pub trait DOMHTMLScriptElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_charset(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_defer(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_event(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_html_for(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_src(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_charset(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_defer(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_event(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_html_for(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_src(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_text(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    fn set_property_charset(&self, charset: Option<&str>);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLScriptElement>> DOMHTMLScriptElementExt for O {
    fn get_charset(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_charset(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_defer(&self) -> bool {
        unsafe {
            from_glib(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_defer(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_event(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_event(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_html_for(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_html_for(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_src(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_src(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_text(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_script_element_get_type_attr(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_charset(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_charset(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_defer(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_defer(
                self.as_ref().to_glib_none().0,
                value.to_glib(),
            );
        }
    }

    fn set_event(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_event(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_html_for(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_html_for(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_src(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_script_element_set_type_attr(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_property_charset(&self, charset: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"charset\0".as_ptr() as *const _,
                Value::from(charset).to_glib_none().0,
            );
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                Value::from(type_).to_glib_none().0,
            );
        }
    }

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_charset_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::charset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_charset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_defer_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::defer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_defer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_event_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_html_for_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::html-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_html_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::src\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLScriptElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLScriptElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLScriptElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLScriptElement")
    }
}
