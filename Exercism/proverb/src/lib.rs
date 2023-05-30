pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => {
            return String::new();
        }
        1 => {
            return format!("And all for the want of a {}.", list[0]);
        }
        2 => {
            return format!("For want of a {} the {} was lost.\nAnd all for the want of a {}.", list[0], list[1], list[0]);
        }
        _ => {
            let mut rhyme = String::new();
            let iter_end = list.len() - 1;
            for i in 0..iter_end {
                let append = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
                rhyme.push_str(&append);
            }

            let last_line = format!("And all for the want of a {}.", list[0]);

            rhyme.push_str(&last_line);

            return rhyme;
        }
    }
}
