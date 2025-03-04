use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    use segtree::operation;
    let mut ans = vec![usize::MAX; n];
    let mut segtree = std::iter::repeat(1).take(n + 1).collect::<segtree::Segtree<usize, operation::Add<_>>>();

    let mut num = n;
    for &pi in p.iter().rev() {
        // eprintln!("{pi}");
        // eprintln!("{:?}", &segtree[..n]);
        let i = segtree.upper_bound(0, |&x| x < pi);
        ans[i] = num;
        num -= 1;
        segtree.update(i, 0);
    }
    ans.iter().for_each(|&ai| print!("{ai} "));
    println!();
    // eprintln!("{ans:?}");
}
