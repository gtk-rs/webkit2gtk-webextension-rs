// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMDocument;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLObjectElement(Object<ffi::WebKitDOMHTMLObjectElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_object_element_get_type(),
    }
}

pub trait DOMHTMLObjectElementExt {
    fn get_align(&self) -> Option<String>;

    fn get_archive(&self) -> Option<String>;

    fn get_border(&self) -> Option<String>;

    fn get_code(&self) -> Option<String>;

    fn get_code_base(&self) -> Option<String>;

    fn get_code_type(&self) -> Option<String>;

    fn get_content_document(&self) -> Option<DOMDocument>;

    fn get_data(&self) -> Option<String>;

    fn get_declare(&self) -> bool;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_height(&self) -> Option<String>;

    fn get_hspace(&self) -> libc::c_long;

    fn get_name(&self) -> Option<String>;

    fn get_standby(&self) -> Option<String>;

    fn get_type_attr(&self) -> Option<String>;

    fn get_use_map(&self) -> Option<String>;

    fn get_vspace(&self) -> libc::c_long;

    fn get_width(&self) -> Option<String>;

    fn set_align(&self, value: &str);

    fn set_archive(&self, value: &str);

    fn set_border(&self, value: &str);

    fn set_code(&self, value: &str);

    fn set_code_base(&self, value: &str);

    fn set_code_type(&self, value: &str);

    fn set_data(&self, value: &str);

    fn set_declare(&self, value: bool);

    fn set_height(&self, value: &str);

    fn set_hspace(&self, value: libc::c_long);

    fn set_name(&self, value: &str);

    fn set_standby(&self, value: &str);

    fn set_type_attr(&self, value: &str);

    fn set_use_map(&self, value: &str);

    fn set_vspace(&self, value: libc::c_long);

    fn set_width(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_code_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_declare_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_standby_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMHTMLObjectElement> + IsA<glib::object::Object>> DOMHTMLObjectElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_archive(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_archive(self.to_glib_none().0))
        }
    }

    fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_border(self.to_glib_none().0))
        }
    }

    fn get_code(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code(self.to_glib_none().0))
        }
    }

    fn get_code_base(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_base(self.to_glib_none().0))
        }
    }

    fn get_code_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_type(self.to_glib_none().0))
        }
    }

    fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_content_document(self.to_glib_none().0))
        }
    }

    fn get_data(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_data(self.to_glib_none().0))
        }
    }

    fn get_declare(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_object_element_get_declare(self.to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_height(self.to_glib_none().0))
        }
    }

    fn get_hspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_object_element_get_hspace(self.to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_standby(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_standby(self.to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_type_attr(self.to_glib_none().0))
        }
    }

    fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_use_map(self.to_glib_none().0))
        }
    }

    fn get_vspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_object_element_get_vspace(self.to_glib_none().0)
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_width(self.to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_archive(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_archive(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code_base(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_base(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_data(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_data(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_declare(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_declare(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_hspace(self.to_glib_none().0, value);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_standby(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_standby(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_vspace(self.to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::archive",
                transmute(notify_archive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::border",
                transmute(notify_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::code",
                transmute(notify_code_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::code-base",
                transmute(notify_code_base_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::code-type",
                transmute(notify_code_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::content-document",
                transmute(notify_content_document_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::data",
                transmute(notify_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_declare_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::declare",
                transmute(notify_declare_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::form",
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hspace",
                transmute(notify_hspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_standby_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::standby",
                transmute(notify_standby_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-map",
                transmute(notify_use_map_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vspace",
                transmute(notify_vspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_archive_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_border_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_code_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_code_base_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_code_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_content_document_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_data_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_declare_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_standby_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_map_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).downcast_unchecked())
}
