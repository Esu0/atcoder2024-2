use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut segtree = (0..m).map(|_| 0usize).collect::<Segtree<_, operation::Add<_>>>();
    let mut ans = 0;
    let mut cur = 0;
    let mut offset = 0;
    for (i, &ai) in a.iter().enumerate() {
        let ai = ai % m;
        cur += (i + 1) * ai;
        let mut l = m - ai + offset;
        let r = offset;
        let mut cnt = 0;
        if l < m {
            cnt += segtree.query(l..m);
            l = m;
        }
        l -= m;
        cnt += segtree.query(l..r);
        // eprintln!("{cnt}");
        cur -= cnt * m;

        ans += cur;
        *segtree.get_mut(offset) += 1;
        offset = (offset + m - ai) % m;
    }
    println!("{ans}");
}
