// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use webkit2_webextension_sys;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMNamedNodeMap(Object<webkit2_webextension_sys::WebKitDOMNamedNodeMap, webkit2_webextension_sys::WebKitDOMNamedNodeMapClass, DOMNamedNodeMapClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_named_node_map_get_type(),
    }
}

pub const NONE_DOM_NAMED_NODE_MAP: Option<&DOMNamedNodeMap> = None;

pub trait DOMNamedNodeMapExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_named_item(&self, name: &str) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn remove_named_item(&self, name: &str) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn remove_named_item_ns(
        &self,
        namespaceURI: &str,
        localName: &str,
    ) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_named_item<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_named_item_ns<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, glib::Error>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNamedNodeMap>> DOMNamedNodeMapExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_named_node_map_get_length(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_dom_named_node_map_get_named_item(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ),
            )
        }
    }

    fn get_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_dom_named_node_map_get_named_item_ns(
                    self.as_ref().to_glib_none().0,
                    namespaceURI.to_glib_none().0,
                    localName.to_glib_none().0,
                ),
            )
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_named_node_map_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn remove_named_item(&self, name: &str) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_named_node_map_remove_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_named_item_ns(
        &self,
        namespaceURI: &str,
        localName: &str,
    ) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_named_node_map_remove_named_item_ns(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
                localName.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_named_item<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_named_node_map_set_named_item(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_named_item_ns<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_named_node_map_set_named_item_ns(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMNamedNodeMap,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMNamedNodeMap>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMNamedNodeMap::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMNamedNodeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMNamedNodeMap")
    }
}
