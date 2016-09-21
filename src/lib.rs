/*
 * TODO: The send-request event bugs.
 */

#![allow(non_snake_case, unused_imports)]

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gtk;

extern crate webkit2gtk_webextension_sys as ffi;

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
        pub unsafe fn webkit_web_extension_initialize_with_user_data(extension: *mut ::webkit2gtk_webextension_sys::WebKitWebExtension, user_data: *mut ::glib_sys::GVariant) {
            web_extension_initialize(::glib::translate::FromGlibPtr::from_glib_none(extension), ::glib::translate::FromGlibPtr::from_glib_none(user_data));
        }
    };
}

mod auto;

pub use auto::*;

unsafe impl Send for WebExtension {}
