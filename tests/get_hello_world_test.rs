use iron::Headers;
use iron_test::{request, response};
use rust_rest_api_with_database::my_router;

#[test]
fn test_hello_world() {
    let response = request::get("http://ignored.host:9999/",
                                Headers::new(),
                                &my_router()).unwrap();
    let body = response::extract_body_to_string(response);

    assert_eq!(body, "Hello World.");
}
