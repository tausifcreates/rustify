// `IpAddrKind` enumeration and listing the possible kinds
// an IP address can be, `V4` and `V6`. These are the variants
// of the enum.
enum IpAddrKind {
    V4,
    V6,
}

// `IpAddrKind` is now a custom data type.

fn main() {
    // We can create instances of each of the two variants of
    // `IpAddrKind` like this:

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // The variants of the enum are namespaced under its identifier,
    // and we use `::` to separate the two. The reason this is useful
    // is that now both values `IpAddrKind::V4` and `IpAddrKind::V6`
    // are of the same type: `IpAddrKind`.

    // And we can call this function with either variant:
    route(&four);
    route(&six);
}

// define a function that takes any `&IpAddrKind`
fn route(ip_kind: &IpAddrKind) {}
