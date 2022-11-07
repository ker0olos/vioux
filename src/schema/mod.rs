#[allow(clippy::all)]
pub mod placeholder {
    include!(concat!(env!("OUT_DIR"), "/placeholder.rs"));
}

#[cfg(test)]
mod tests {
    use crate::schema::placeholder;

    #[test]
    fn test_placeholder_request() {
        let request = placeholder::Request {
            nonce: String::default(),
        };

        assert!(request.nonce.is_empty())
    }

    #[test]
    fn test_placeholder_response() {
        let response = placeholder::Response {
            nonce: String::default(),
        };

        assert!(response.nonce.is_empty())
    }
}
