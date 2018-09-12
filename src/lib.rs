#![allow(non_snake_case)]

#[cfg(any(feature = "v2_18"))]
extern crate gio;
#[macro_use]
extern crate glib;
pub extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate libc;

pub extern crate webkit2gtk_webextension_sys as ffi;

pub use glib::{Error, Object};

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

#[macro_export]
macro_rules! web_extension_init {
    () => {
        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize(
            extension: *mut $crate::ffi::WebKitWebExtension)
        {
            let extension: $crate::WebExtension = ::glib::translate::FromGlibPtrNone::from_glib_none(extension);
            web_extension_initialize(&extension);
        }
    };
}

#[macro_export]
macro_rules! web_extension_init_with_data {
    () => {
        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize_with_user_data(
            extension: *mut $crate::ffi::WebKitWebExtension,
            user_data: *mut $crate::glib_ffi::GVariant)
        {
            let extension: $crate::WebExtension = ::glib::translate::FromGlibPtrNone::from_glib_none(extension);
            let user_data: ::glib::variant::Variant = ::glib::translate::FromGlibPtrNone::from_glib_none(user_data);
            web_extension_initialize(&extension, &user_data);
        }
    };
}

mod auto;
mod dom_html_field_set_element;

pub use auto::*;
pub use dom_html_field_set_element::*;

unsafe impl Send for WebExtension {}
