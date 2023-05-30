const CHILDRENS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let position: usize = CHILDRENS.iter().position(|&n| n == _student).unwrap() * 2;

    let apply_char = |c: char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    };

    _diagram
        .lines()
        .flat_map(|line: &str| line[position..=position + 1].chars().map(apply_char))
        .collect()
}
