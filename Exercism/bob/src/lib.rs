fn is_yelling_test(message: &str) -> bool {
    let have_letters: bool = message.chars().filter(|x: &char| x.is_alphabetic()).count() > 0;
    return message.to_uppercase() == message && have_letters;
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.trim().len() == 0 => return "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling_test(m) => return "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => return "Sure.",
        m if is_yelling_test(m) => return "Whoa, chill out!",
        _ => return "Whatever.",
    }
}