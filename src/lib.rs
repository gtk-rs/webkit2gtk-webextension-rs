#![allow(let_unit_value, new_without_default, non_snake_case, transmute_ptr_to_ref, type_complexity, unused_imports)]

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate libc;

extern crate webkit2gtk_webextension_sys as ffi;

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

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

#[macro_export]
macro_rules! web_extension_init {
    () => {
        extern crate glib;
        extern crate glib_sys;
        extern crate webkit2gtk_webextension_sys;

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize(extension: *mut ::webkit2gtk_webextension_sys::WebKitWebExtension)
        {
            let extension: $crate::WebExtension = ::glib::translate::FromGlibPtrNone::from_glib_none(extension);
            web_extension_initialize(&extension);
        }
    };
}

#[macro_export]
macro_rules! web_extension_init_with_data {
    () => {
        extern crate glib;
        extern crate glib_sys;
        extern crate webkit2gtk_webextension_sys;

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize_with_user_data(
            extension: *mut ::webkit2gtk_webextension_sys::WebKitWebExtension,
            user_data: *mut ::glib_sys::GVariant)
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
