use std::fmt::Write;

/// Given a bytearray, get a string hex.
pub fn to_hex(a: &[u8]) -> String {
    let mut s = String::new();
    for v in a {
        write!(s, "{:02x}", *v).unwrap();
    }
    s
}
