use reqwasm::http::Request;

pub fn post(url: &str) -> Request {
    reqwasm::http::Request::post(url)
        .credentials(web_sys::RequestCredentials::Include)
        .header("content-type", "application/json")
}

pub fn get(url: &str) -> Request {
    reqwasm::http::Request::get(url).credentials(web_sys::RequestCredentials::Include)
}
