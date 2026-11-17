//! Shared HAR records and parser; input admission, redaction and analysis stay app-owned.
pub use craft_har::{Har, HarBody, HarEntry, HarError, HarRequest, HarResponse, parse};

#[cfg(test)]
#[path = "har_legacy.rs"]
mod legacy;

#[cfg(test)]
mod parity_tests {
    use super::*;
    #[test]
    fn retained_records_errors_and_truncation_match() {
        for data in [b"".as_slice(), b"{}", b"bad", br#"{"log":{"entries":[]}}"#, br#"{"log":{"entries":[null,{},1,{"request":{"method":"GET","url":"https://example.test","headers":[{"name":"X-Test","value":"v"}]},"response":{"status":200,"content":{"text":"body"}}}]}}"#] {
            for limit in [0, 1, 2, 20] {
                assert_eq!(format!("{:?}", legacy::parse(data, limit)), format!("{:?}", parse(data, limit)));
            }
        }
    }
}
