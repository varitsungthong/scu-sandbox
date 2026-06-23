enum IpAddrKind{
    V4,
    V6,
}

struct IpAddress {
    kind : IpAddrKind,
    address : String,
}

fn main() {
    let home = IpAddress {
        kind : IpAddrKind::V4,
        address : String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind : IpAddrKind::V6,
        address : String::from("::1"),
    };
}
