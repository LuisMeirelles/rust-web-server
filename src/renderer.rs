use crate::request::Request;
use crate::response::Response;

const DOUBLE_LINE: &str =
    "================================================================================";
const SIMPLE_LINE: &str =
    "--------------------------------------------------------------------------------";

pub fn print_section_header(header: &str) {
    println!("{SIMPLE_LINE}");
    println!("{}", header);
    println!("{SIMPLE_LINE}");
}

pub fn print_section_body(body: &str) {
    println!("{}", body);
    println!("{SIMPLE_LINE}");
}

/**
* todo: change the type of http_request
*/
pub fn print_request(request: &Request) {
    print_section_header("Request:");
    print_section_body(request.to_string().as_str());
}

pub fn print_response(response: &Response) {
    print_section_header("Response:");
    print_section_body(&response.to_string());
}

pub fn print_request_and_response(request: &Request, response: &Response) {
    println!();
    println!("{DOUBLE_LINE}");
    print_request(request);
    println!();
    print_response(response);
    println!("{DOUBLE_LINE}");
}
