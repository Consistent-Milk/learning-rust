fn main() {
    let num: Vec<u16> = (1..=100).collect();

    for (i, n) in num.iter().enumerate() {
        let curr = *n;
        println!("{} {}", i + 1, mod3(curr));
    }
}

fn mod3(a: u16) -> u16 {
    let mut r: u16 = (a >> 8) + (a & 0xff); // r mod 255 == a mod 255
    r = (r >> 4) + (r & 0xf); // r' mod 15 == r mod 15
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3

    let t: i16 = r as i16 - 3;
    let c: i16 = t >> 15;

    ((c as u16) & r) as u16 ^ (!c & t) as u16
}
