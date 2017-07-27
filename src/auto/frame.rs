// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Frame(Object<ffi::WebKitFrame>);

    match fn {
        get_type => || ffi::webkit_frame_get_type(),
    }
}

impl Frame {
    //#[cfg(feature = "v2_2")]
    //pub fn get_javascript_context_for_script_world(&self, world: &ScriptWorld) -> /*Ignored*/Option<java_script_core::GlobalContext> {
    //    unsafe { TODO: call ffi::webkit_frame_get_javascript_context_for_script_world() }
    //}

    //#[cfg(feature = "v2_2")]
    //pub fn get_javascript_global_context(&self) -> /*Ignored*/Option<java_script_core::GlobalContext> {
    //    unsafe { TODO: call ffi::webkit_frame_get_javascript_global_context() }
    //}

    #[cfg(feature = "v2_2")]
    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_frame_get_uri(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn is_main_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_frame_is_main_frame(self.to_glib_none().0))
        }
    }
}
