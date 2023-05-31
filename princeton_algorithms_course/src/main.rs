use princeton_algorithms_course::algorithms::quick_find_uf::QuickFindUF;
use princeton_algorithms_course::algorithms::quick_union_uf::QuickUnionUF;

fn main() {
    let n: i32 = 10;
    let mut v: Vec<i32> = Vec::new();

    for i in 0..n {
        v.push(i);
    }

    println!("{:?}", v);

    let mut qf: QuickFindUF = QuickFindUF::new(n);

    qf.union(0, 1);
    qf.union(1, 3);

    println!("\n0 and 3 connected -> {}", qf.connected(0, 3));
    println!("1 and 5 connected -> {}", qf.connected(1, 5));
    println!("{}", qf);

    let mut qu: QuickUnionUF = QuickUnionUF::new(n);

    qu.union(4, 3);
    qu.union(3, 8);
    qu.union(9, 4);

    println!("\n3 and 9 connected -> {}", qu.connected(3, 9));
    println!("0 and 1 connected -> {}", qu.connected(0, 1));
    println!("{}", qu);
}
