#![allow(non_snake_case, unused_imports)]

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gtk;

extern crate webkit2webextension_sys as ffi;

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
        extern crate webkit2webextension_sys;

        #[no_mangle]
        pub unsafe fn webkit_web_extension_initialize(extension: *mut ::webkit2webextension_sys::WebKitWebExtension) {
            web_extension_initialize(::glib::translate::FromGlibPtr::from_glib_none(extension));
        }
    };
}

mod auto;

pub use auto::*;
