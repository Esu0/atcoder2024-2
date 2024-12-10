use proconio::input;
use segtree::{Segtree, operation};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    // let tree_min = a.iter().copied().collect::<Segtree<_, operation::Min<_>>>();
    let tree_max = a.iter().copied().chain(std::iter::once(u64::MAX)).collect::<Segtree<_, operation::Max<_>>>();
    let sums = {
        let mut v = vec![0];
        v.extend_from_slice(&a);
        for i in 1..=n {
            v[i] += v[i - 1];
        }
        v
    };
    for i in 0..n {
        let (mut l, mut r) = (i, i);
        let mut sum = sums[r + 1] - sums[l];
        loop {
            l = tree_max.lower_bound(l, |&x| x < sum);
            r = tree_max.upper_bound(r + 1, |&x| x < sum) - 1;
            let next_sum = sums[r + 1] - sums[l];
            if next_sum == sum {
                break;
            }
            sum = next_sum;
        }
        let ans = sum;
        print!("{ans}");
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
