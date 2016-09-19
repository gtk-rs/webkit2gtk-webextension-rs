#[macro_use]
extern crate webkit2webextension;

use webkit2webextension::WebExtension;

web_extension_init!();

#[no_mangle]
pub fn web_extension_initialize(extension: WebExtension) {
    extension.connect_page_created(|_, page| {
        page.connect_document_loaded(|page| {
            println!("Page {} created for {:?}", page.get_id(), page.get_uri());
            let document = page.get_dom_document().unwrap();
            println!("URL: {:?}", document.get_url());
            println!("Title: {:?}", document.get_title());
            document.set_title("My Web Page");
        });

        /*page.connect_send_request(|page, request, response| {
            let url = page.get_uri();
            println!("{:?}", url);
            // Block t411.
            url == Some("http://www.t411.ch".to_string())
        });*/
    });
}
