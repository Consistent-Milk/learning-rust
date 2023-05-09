#[derive(Debug)]
enum IpAddress {
    // Instead of combining enum and struct to specify an IP
    V4(u8, u8, u8, u8), // we can also only use enum
    V6(String),
}

fn main() {
    let home: IpAddress = IpAddress::V4(127, 0, 0, 1);
    let loopback: IpAddress = IpAddress::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}
