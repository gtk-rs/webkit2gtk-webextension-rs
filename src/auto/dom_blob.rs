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
use DOMObject;

glib_wrapper! {
    pub struct DOMBlob(Object<webkit2_webextension_sys::WebKitDOMBlob, webkit2_webextension_sys::WebKitDOMBlobClass, DOMBlobClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_blob_get_type(),
    }
}

pub const NONE_DOM_BLOB: Option<&DOMBlob> = None;

pub trait DOMBlobExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_size(&self) -> u64;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMBlob>> DOMBlobExt for O {
    fn get_size(&self) -> u64 {
        unsafe {
            webkit2_webextension_sys::webkit_dom_blob_get_size(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMBlob,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMBlob>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMBlob::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMBlob")
    }
}
