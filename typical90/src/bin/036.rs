use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
    }

    let &p1 = xy.iter().max_by_key(|&&(x, y)| x + y).unwrap();
    let &p2 = xy.iter().max_by_key(|&&(x, y)| x - y).unwrap();
    let &p3 = xy.iter().max_by_key(|&&(x, y)| -x - y).unwrap();
    let &p4 = xy.iter().max_by_key(|&&(x, y)| y - x).unwrap();
    let ps = [p1, p2, p3, p4];
    for _ in 0..q {
        input! { q: usize }
        let (xq, yq) = xy[q - 1];
        let ans = ps
            .iter()
            .map(|&(x, y)| (xq - x).abs() + (yq - y).abs())
            .max()
            .unwrap();
        // let ans_naive = xy.iter().map(|&(x, y)| (xq - x).abs() + (yq - y).abs()).max().unwrap();
        // assert_eq!(ans, ans_naive);
        println!("{ans}");
    }
}
