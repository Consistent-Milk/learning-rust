pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let max_weight_usize: usize = max_weight as usize;
    let mut dp: Vec<u32> = vec![0; max_weight_usize + 1];

    for i in 0..items.len() {
        for w in (0..=max_weight_usize).rev() {
            let item_weight: usize = items[i].weight as usize;

            if item_weight <= w {
                dp[w] = dp[w].max(dp[w - item_weight] + items[i].value);
                println!("(i = {i}, w = {w}, weight = {item_weight}): {:?}", dp);
            }
        }
    }
    return dp[max_weight_usize];
}
