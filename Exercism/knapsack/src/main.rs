use knapsack::*;
fn main() {
    let max_weight: u32 = 10;
    let items: [Item; 5] = [
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 10,
            value: 50,
        },
    ];

    maximum_value(max_weight, &items);
}