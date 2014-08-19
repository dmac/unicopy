extern crate clipboard;
extern crate http;
extern crate regex;
extern crate url;

use std::os;

use http::client::RequestWriter;
use http::method::Get;
use regex::Regex;
use url::Url;

fn main() {
    let mut args = os::args();
    args.remove(0);
    let search_str = args.connect(" ");

    let mut url_str = "http://www.fileformat.info/info/unicode/char/search.htm?preview=entity&q=".to_string();
    url_str.push_str(search_str.as_slice());
    let url = match Url::parse(url_str.as_slice()) {
        Ok(url) => url,
        Err(e) => fail!("{}", e)
    };

    let request: RequestWriter = match RequestWriter::new(Get, url) {
        Ok(request) => request,
        Err(e) => fail!("{}", e),
    };

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err((_, e)) => fail!("{}", e),
    };

    let html = match response.read_to_string() {
        Ok(html) => html,
        Err(e) => fail!("{}", e)
    };

    let i = match html.as_slice().find_str("<tr class=\"row0\">") {
        Some(i) => i,
        None => {
            println!("no results ☹", );
            os::set_exit_status(1);
            return;
        }
    };

    let s = html.as_slice().slice_from(i);
    let re = match Regex::new("<a .*>U\\+(.+)</a>") {
        Ok(re) => re,
        Err(e) => fail!("{}", e)
    };

    let code = re.captures_iter(s).skip(0).next().unwrap().at(1);
    let num = std::f32::from_str_hex(code).unwrap() as u32;
    let c = std::char::from_u32(num).unwrap();

    match clipboard::write(format!("{}", c).as_slice()) {
        Ok(_) => {},
        Err(e) => fail!("{}", e)
    }

    println!("{} copied to clipboard", c);
}
