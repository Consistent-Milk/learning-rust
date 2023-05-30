#[allow(unused)]
fn main() {
    let s: &str = "Hello";
    let len: usize = 2;

    let mut v: Vec<String> = Vec::new();

    for (i, c) in s.char_indices() {
        let check = i + len;

        if check < s.len() + 1 {
            v.push(s[i..check].to_string())
        } else {
            break;
        }
    }

    println!("{:?}", v);
}
