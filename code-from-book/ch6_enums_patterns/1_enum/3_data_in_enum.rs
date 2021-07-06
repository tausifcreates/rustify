// This new definition of the `IpAddr` enum says that both
// `V4` and `V6` variants will have associated `String` values.
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    // We attach data to each variant of the enum directly,
    // so there is no need for an extra struct.
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
}
