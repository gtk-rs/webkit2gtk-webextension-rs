#[macro_use]
extern crate webkit2webextension;

use webkit2webextension::WebExtension;

web_extension_init!();

#[no_mangle]
pub fn web_extension_initialize(extension: WebExtension) {
    println!("Initialize");
}
