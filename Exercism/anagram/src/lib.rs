use std::collections::HashSet;

pub fn lower(c: char) -> String {
    if !c.is_lowercase() {
        return c.to_lowercase().to_string();
    } else {
        return c.to_string();
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lw: Vec<_> = word.chars().map(lower).collect();
    let mut sc: Vec<_> = word.chars().map(lower).collect();
    sc.sort_unstable();
    let result = possible_anagrams
        .iter()
        .filter(|w| {
            let mut sci: Vec<_> = w.chars().map(lower).collect();
            let same_word = lw == sci;
            sci.sort_unstable();
            sci == sc && !same_word
        })
        .map(|&w| w);

    HashSet::from_iter(result)
}
