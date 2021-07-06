enum IpAddrKind {
    V4,
    V6,
}
// We’ve used a struct to bundle the `kind` and `address` values
// together, so now the variant is associated with the value.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
