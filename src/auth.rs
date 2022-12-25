use crate::config;
use actix_web::http::{self, header::HeaderMap};

pub fn valid_auth_token(headers: &HeaderMap) -> bool {
    headers
        .get(http::header::AUTHORIZATION)
        .filter(|&value| value == config::AUTH)
        .is_some()
}

#[cfg(test)]
mod test {
    use actix_web::test::TestRequest;

    use crate::config;

    use super::valid_auth_token;

    #[test]
    fn test_no_header() {
        let req = TestRequest::default().to_http_request();
        let valid = valid_auth_token(req);
        assert!(!valid);
    }

    #[test]
    fn test_invalid() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Basic 1234abcd"))
            .to_http_request();
        let valid = valid_auth_token(req);
        assert!(!valid);
    }

    #[test]
    fn test_valid() {
        let req = TestRequest::default()
            .insert_header(("Authorization", config::AUTH))
            .to_http_request();
        let valid = valid_auth_token(req);
        assert!(valid);
    }
}
