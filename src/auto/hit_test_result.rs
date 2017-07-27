// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct HitTestResult(Object<ffi::WebKitHitTestResult>);

    match fn {
        get_type => || ffi::webkit_hit_test_result_get_type(),
    }
}

pub trait HitTestResultExt {
    fn context_is_editable(&self) -> bool;

    fn context_is_image(&self) -> bool;

    fn context_is_link(&self) -> bool;

    fn context_is_media(&self) -> bool;

    fn context_is_scrollbar(&self) -> bool;

    fn context_is_selection(&self) -> bool;

    fn get_context(&self) -> u32;

    fn get_image_uri(&self) -> Option<String>;

    fn get_link_label(&self) -> Option<String>;

    fn get_link_title(&self) -> Option<String>;

    fn get_link_uri(&self) -> Option<String>;

    fn get_media_uri(&self) -> Option<String>;
}

impl<O: IsA<HitTestResult>> HitTestResultExt for O {
    fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_editable(self.to_glib_none().0))
        }
    }

    fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_image(self.to_glib_none().0))
        }
    }

    fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_link(self.to_glib_none().0))
        }
    }

    fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_media(self.to_glib_none().0))
        }
    }

    fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_scrollbar(self.to_glib_none().0))
        }
    }

    fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_selection(self.to_glib_none().0))
        }
    }

    fn get_context(&self) -> u32 {
        unsafe {
            ffi::webkit_hit_test_result_get_context(self.to_glib_none().0)
        }
    }

    fn get_image_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_image_uri(self.to_glib_none().0))
        }
    }

    fn get_link_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_label(self.to_glib_none().0))
        }
    }

    fn get_link_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_title(self.to_glib_none().0))
        }
    }

    fn get_link_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_uri(self.to_glib_none().0))
        }
    }

    fn get_media_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_media_uri(self.to_glib_none().0))
        }
    }
}
