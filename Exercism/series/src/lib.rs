pub fn series(digits: &str, len: usize) -> Vec<String> {
    // let total_chars: usize = digits.len();

    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()

    // let mut v: Vec<String> = Vec::new();

    // if total_chars < len {
    //     return v;
    // }

    // for (i, _) in digits.char_indices() {
    //     let check: usize = i + len;

    //     if check < total_chars + 1 {
    //         v.push(digits[i..check].to_string());
    //     } else {
    //         break;
    //     }
    // }

    // v
}
