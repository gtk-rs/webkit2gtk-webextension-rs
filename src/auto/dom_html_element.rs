// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLCollection;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLElement")]
    pub struct DOMHTMLElement(Object<ffi::WebKitDOMHTMLElement, ffi::WebKitDOMHTMLElementClass>) @extends DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_element_get_type(),
    }
}

impl DOMHTMLElement {
        pub const NONE: Option<&'static DOMHTMLElement> = None;
    
}

pub trait DOMHTMLElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_click")]
    fn click(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_access_key")]
    #[doc(alias = "get_access_key")]
    fn access_key(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_10", deprecated = "Since 2.10")]
    #[cfg(any(not(feature = "v2_10"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_10"))))]
    #[doc(alias = "webkit_dom_html_element_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_content_editable")]
    #[doc(alias = "get_content_editable")]
    fn content_editable(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_dir")]
    #[doc(alias = "get_dir")]
    fn dir(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_get_draggable")]
    #[doc(alias = "get_draggable")]
    fn is_draggable(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_get_hidden")]
    #[doc(alias = "get_hidden")]
    fn is_hidden(&self) -> bool;

    #[cfg_attr(feature = "v2_8", deprecated = "Since 2.8")]
    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    #[doc(alias = "webkit_dom_html_element_get_inner_html")]
    #[doc(alias = "get_inner_html")]
    fn inner_html(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_inner_text")]
    #[doc(alias = "get_inner_text")]
    fn inner_text(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_is_content_editable")]
    #[doc(alias = "get_is_content_editable")]
    fn is_content_editable(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_lang")]
    #[doc(alias = "get_lang")]
    fn lang(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_8", deprecated = "Since 2.8")]
    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    #[doc(alias = "webkit_dom_html_element_get_outer_html")]
    #[doc(alias = "get_outer_html")]
    fn outer_html(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_outer_text")]
    #[doc(alias = "get_outer_text")]
    fn outer_text(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_get_spellcheck")]
    #[doc(alias = "get_spellcheck")]
    fn is_spellcheck(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_tab_index")]
    #[doc(alias = "get_tab_index")]
    fn tab_index(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_get_translate")]
    #[doc(alias = "get_translate")]
    fn is_translate(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_get_webkitdropzone")]
    #[doc(alias = "get_webkitdropzone")]
    fn webkitdropzone(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_access_key")]
    fn set_access_key(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_content_editable")]
    fn set_content_editable(&self, value: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_dir")]
    fn set_dir(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_set_draggable")]
    fn set_draggable(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_set_hidden")]
    fn set_hidden(&self, value: bool);

    #[cfg_attr(feature = "v2_8", deprecated = "Since 2.8")]
    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    #[doc(alias = "webkit_dom_html_element_set_inner_html")]
    fn set_inner_html(&self, contents: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_inner_text")]
    fn set_inner_text(&self, value: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_lang")]
    fn set_lang(&self, value: &str);

    #[cfg_attr(feature = "v2_8", deprecated = "Since 2.8")]
    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    #[doc(alias = "webkit_dom_html_element_set_outer_html")]
    fn set_outer_html(&self, contents: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_outer_text")]
    fn set_outer_text(&self, value: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_set_spellcheck")]
    fn set_spellcheck(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_tab_index")]
    fn set_tab_index(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_element_set_title")]
    fn set_title(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_set_translate")]
    fn set_translate(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_element_set_webkitdropzone")]
    fn set_webkitdropzone(&self, value: &str);

    fn get_property_draggable(&self) -> bool;

    fn set_property_draggable(&self, draggable: bool);

    fn get_property_hidden(&self) -> bool;

    fn set_property_hidden(&self, hidden: bool);

    fn get_property_spellcheck(&self) -> bool;

    fn set_property_spellcheck(&self, spellcheck: bool);

    fn get_property_translate(&self) -> bool;

    fn set_property_translate(&self, translate: bool);

    fn get_property_webkitdropzone(&self) -> Option<glib::GString>;

    fn set_property_webkitdropzone(&self, webkitdropzone: Option<&str>);

    #[doc(alias = "access-key")]
    fn connect_access_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "content-editable")]
    fn connect_content_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "dir")]
    fn connect_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "draggable")]
    fn connect_draggable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "hidden")]
    fn connect_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "inner-text")]
    fn connect_inner_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-content-editable")]
    fn connect_is_content_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "lang")]
    fn connect_lang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "outer-text")]
    fn connect_outer_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "spellcheck")]
    fn connect_spellcheck_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tab-index")]
    fn connect_tab_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "translate")]
    fn connect_translate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "webkitdropzone")]
    fn connect_webkitdropzone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLElement>> DOMHTMLElementExt for O {
    fn click(&self) {
        unsafe {
            ffi::webkit_dom_html_element_click(self.as_ref().to_glib_none().0);
        }
    }

    fn access_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_access_key(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_10"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_10"))))]
    fn children(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_children(self.as_ref().to_glib_none().0))
        }
    }

    fn content_editable(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_content_editable(self.as_ref().to_glib_none().0))
        }
    }

    fn dir(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_dir(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_draggable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_draggable(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_hidden(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    fn inner_html(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_inner_html(self.as_ref().to_glib_none().0))
        }
    }

    fn inner_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_inner_text(self.as_ref().to_glib_none().0))
        }
    }

    fn is_content_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_is_content_editable(self.as_ref().to_glib_none().0))
        }
    }

    fn lang(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_lang(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    fn outer_html(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_outer_html(self.as_ref().to_glib_none().0))
        }
    }

    fn outer_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_outer_text(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_spellcheck(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_spellcheck(self.as_ref().to_glib_none().0))
        }
    }

    fn tab_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_element_get_tab_index(self.as_ref().to_glib_none().0)
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_title(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_translate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_translate(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn webkitdropzone(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_webkitdropzone(self.as_ref().to_glib_none().0))
        }
    }

    fn set_access_key(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_access_key(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_content_editable(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_content_editable(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_dir(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_dir(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_draggable(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_element_set_draggable(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_hidden(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_element_set_hidden(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    fn set_inner_html(&self, contents: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_inner_html(self.as_ref().to_glib_none().0, contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_inner_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_inner_text(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_lang(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_lang(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(not(feature = "v2_8"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_8"))))]
    fn set_outer_html(&self, contents: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_outer_html(self.as_ref().to_glib_none().0, contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_outer_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_outer_text(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_spellcheck(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_element_set_spellcheck(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    fn set_tab_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_element_set_tab_index(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_title(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_title(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_translate(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_element_set_translate(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_webkitdropzone(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_webkitdropzone(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_draggable(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "draggable")
    }

    fn set_property_draggable(&self, draggable: bool) {
        glib::ObjectExt::set_property(self.as_ref(),"draggable", &draggable)
    }

    fn get_property_hidden(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "hidden")
    }

    fn set_property_hidden(&self, hidden: bool) {
        glib::ObjectExt::set_property(self.as_ref(),"hidden", &hidden)
    }

    fn get_property_spellcheck(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "spellcheck")
    }

    fn set_property_spellcheck(&self, spellcheck: bool) {
        glib::ObjectExt::set_property(self.as_ref(),"spellcheck", &spellcheck)
    }

    fn get_property_translate(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "translate")
    }

    fn set_property_translate(&self, translate: bool) {
        glib::ObjectExt::set_property(self.as_ref(),"translate", &translate)
    }

    fn get_property_webkitdropzone(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "webkitdropzone")
    }

    fn set_property_webkitdropzone(&self, webkitdropzone: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(),"webkitdropzone", &webkitdropzone)
    }

    fn connect_access_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_access_key_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::access-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_access_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_content_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_editable_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::content-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_content_editable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dir_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dir\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_dir_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_draggable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draggable_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::draggable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_draggable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hidden_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_hidden_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_inner_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inner_text_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inner-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_inner_text_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_is_content_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_content_editable_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-content-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_is_content_editable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_lang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lang_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lang\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_lang_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_outer_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_outer_text_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::outer-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_outer_text_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_spellcheck_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spellcheck_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spellcheck\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_spellcheck_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_tab_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_index_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_tab_index_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_translate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_translate_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::translate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_translate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_webkitdropzone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_webkitdropzone_trampoline<P: IsA<DOMHTMLElement>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::webkitdropzone\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_webkitdropzone_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLElement")
    }
}
