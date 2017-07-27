// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct URIResponse(Object<ffi::WebKitURIResponse>);

    match fn {
        get_type => || ffi::webkit_uri_response_get_type(),
    }
}

impl URIResponse {
    pub fn get_content_length(&self) -> u64 {
        unsafe {
            ffi::webkit_uri_response_get_content_length(self.to_glib_none().0)
        }
    }

    //pub fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call ffi::webkit_uri_response_get_http_headers() }
    //}

    pub fn get_mime_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_status_code(&self) -> u32 {
        unsafe {
            ffi::webkit_uri_response_get_status_code(self.to_glib_none().0)
        }
    }

    pub fn get_suggested_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_suggested_filename(self.to_glib_none().0))
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_uri(self.to_glib_none().0))
        }
    }
}
