pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    if minefield[0].is_empty() {
        return vec![String::new()];
    }

    let width: usize = minefield[0].len();

    let mut result: Vec<char> = Vec::new();

    for (y, &line) in minefield.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                result.push('*');
                continue;
            }

            let neighbor_mines: usize = minefield
                .iter()
                .take(y + 2)
                .skip((y as isize - 1).max(0) as usize)
                .flat_map(|&line| {
                    line.chars()
                        .take(x + 2)
                        .skip((x as isize - 1).max(0) as usize)
                })
                .filter(|&chr| chr == '*')
                .count();

            if neighbor_mines > 0 {
                result.push(format!("{}", neighbor_mines).chars().nth(0).unwrap());
            } else {
                result.push(' ');
            }
        }
    }

    return result.chunks(width).map(String::from_iter).collect();
}

pub fn annotate_2(minefield: &[&str]) -> Vec<String> {
    let offset:Vec<_> = [-1,-1,-1,0,0,1,1,1].iter().zip([-1,0,1,-1,1,-1,0,1]).collect();
    let width = if minefield.len() > 0 { minefield[0].len() as i32 } else { 0 };
    let height = minefield.len() as i32;
 
    minefield.iter().enumerate().map(|(i, r)| {
        r.char_indices().map(|(j, c)| { match c {
            '*' => c,
             _ => match offset.iter().map(|&(n, m)| (i as i32 + n, j as i32 + m))
                .filter(|&(n, m)| n >= 0 && n < height && m >= 0 && m < width)
                .filter(|&(n, m)| minefield[n as usize].as_bytes()[m as usize] == b'*')
                .count() {
                    0 => ' ',
                    mines => (mines as u8 + '0' as u8) as char,
                }
        }}).collect()
    }).collect()
}

static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0),  (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
pub fn annotate_3(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y| {
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match NEIGBOURHOOD_OFFSETS.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                        0 => ' ',
                        n => (n as u8 + '0' as u8) as char
                    }
            }
        }).collect()
    }).collect()
}