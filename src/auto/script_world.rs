// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::Frame;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::WebPage;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::translate::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitScriptWorld")]
    pub struct ScriptWorld(Object<ffi::WebKitScriptWorld, ffi::WebKitScriptWorldClass>);

    match fn {
        type_ => || ffi::webkit_script_world_get_type(),
    }
}

impl ScriptWorld {
        pub const NONE: Option<&'static ScriptWorld> = None;
    

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_script_world_new")]
    pub fn new() -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new())
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_script_world_new_with_name")]
    #[doc(alias = "new_with_name")]
    pub fn with_name(name: &str) -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new_with_name(name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_script_world_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<ScriptWorld> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_script_world_get_default())
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
impl Default for ScriptWorld {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait ScriptWorldExt: 'static {
    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_script_world_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "window-object-cleared")]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScriptWorld>> ScriptWorldExt for O {
    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_script_world_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_object_cleared_trampoline<P: IsA<ScriptWorld>, F: Fn(&P, &WebPage, &Frame) + 'static>(this: *mut ffi::WebKitScriptWorld, page: *mut ffi::WebKitWebPage, frame: *mut ffi::WebKitFrame, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ScriptWorld::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(page), &from_glib_borrow(frame))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"window-object-cleared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(window_object_cleared_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ScriptWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ScriptWorld")
    }
}
