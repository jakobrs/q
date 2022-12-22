pub mod treap;
use std::io::Read;

use treap::Treap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_ascii_whitespace();

    macro_rules! read {
        ($ty:ty) => {{
            words.next().unwrap().parse::<$ty>().unwrap()
        }};
    }

    let n = read!(usize);
    let l = read!(usize);
    let k = read!(usize);
    let lst: Vec<i64> = words.take(n).map(|s| s.parse().unwrap()).collect();

    let mut set = Treap::new();

    for i in 0..l {
        set.insert_value(lst[i]);
    }

    let mut best = set.sum() - set.sum_of_n_greatest(k);

    for i in 0..n - l {
        set.remove_value(lst[i]);
        set.insert_value(lst[i + l]);

        best = best.min(set.sum() - set.sum_of_n_greatest(k));
    }

    println!("{best}");
}
