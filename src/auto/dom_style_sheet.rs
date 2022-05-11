// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMMediaList;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMStyleSheet")]
    pub struct DOMStyleSheet(Object<ffi::WebKitDOMStyleSheet, ffi::WebKitDOMStyleSheetClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_style_sheet_get_type(),
    }
}

impl DOMStyleSheet {
        pub const NONE: Option<&'static DOMStyleSheet> = None;
    
}

pub trait DOMStyleSheetExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_content_type")]
    #[doc(alias = "get_content_type")]
    fn content_type(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_disabled")]
    #[doc(alias = "get_disabled")]
    fn is_disabled(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_href")]
    #[doc(alias = "get_href")]
    fn href(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_media")]
    #[doc(alias = "get_media")]
    fn media(&self) -> Option<DOMMediaList>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_owner_node")]
    #[doc(alias = "get_owner_node")]
    fn owner_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_parent_style_sheet")]
    #[doc(alias = "get_parent_style_sheet")]
#[must_use]
    fn parent_style_sheet(&self) -> Option<DOMStyleSheet>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_style_sheet_set_disabled")]
    fn set_disabled(&self, value: bool);

    #[doc(alias = "type")]
    fn type_(&self) -> Option<glib::GString>;

    #[doc(alias = "disabled")]
    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "href")]
    fn connect_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "media")]
    fn connect_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "owner-node")]
    fn connect_owner_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent-style-sheet")]
    fn connect_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMStyleSheet>> DOMStyleSheetExt for O {
    fn content_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_style_sheet_get_disabled(self.as_ref().to_glib_none().0))
        }
    }

    fn href(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_href(self.as_ref().to_glib_none().0))
        }
    }

    fn media(&self) -> Option<DOMMediaList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_media(self.as_ref().to_glib_none().0))
        }
    }

    fn owner_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_style_sheet_get_owner_node(self.as_ref().to_glib_none().0))
        }
    }

    fn parent_style_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_parent_style_sheet(self.as_ref().to_glib_none().0))
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_style_sheet_set_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    fn type_(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "type")
    }

    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_disabled_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::disabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_href_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::href\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_href_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::media\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_media_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_owner_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_node_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::owner-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_owner_node_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_style_sheet_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::parent-style-sheet\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_parent_style_sheet_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P: IsA<DOMStyleSheet>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMStyleSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMStyleSheet")
    }
}
