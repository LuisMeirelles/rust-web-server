use crate::response::Response;

const DOUBLE_LINE: &str =
    "================================================================================";
const SIMPLE_LINE: &str =
    "--------------------------------------------------------------------------------";

pub fn print_header(header: &str) {
    println!("{SIMPLE_LINE}");
    println!("{}", header);
    println!("{SIMPLE_LINE}");
}

pub fn print_body(body: &str) {
    println!("{}", body);
    println!("{SIMPLE_LINE}");
}

/**
* todo: change the type of http_request
*/
pub fn print_request(http_request: &Vec<String>) {
    print_header("Request:");
    print_body(&http_request.join("\r\n"));
}

pub fn print_response(response: &Response) {
    print_header("Response:");
    print_body(&response.to_string());
}

pub fn print_request_and_response(http_request: &Vec<String>, response: &Response) {
    println!();
    println!("{DOUBLE_LINE}");
    print_request(http_request);
    println!();
    print_response(response);
    println!("{DOUBLE_LINE}");
}
