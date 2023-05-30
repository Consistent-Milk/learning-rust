pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
        .flat_map(|word: &str| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c: &char| c.is_ascii_uppercase())
                    .filter(|c: &char| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
