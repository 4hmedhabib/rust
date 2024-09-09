fn main() {
    enum IpAddrKind {
        V4,
        V6
    }



    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using structs
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let office = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("office address: {}", office.address);
    println!("loopback address: {}", loopback.address);

    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl std::fmt::Display for IpAddrEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                IpAddrEnum::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
                IpAddrEnum::V6(ref s) => write!(f, "{}", s),
            }
        }
    }

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("office address: {}", home);
    println!("loopback address: {}", loopback);
}
