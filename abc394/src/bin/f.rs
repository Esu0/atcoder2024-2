use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];

    for &(ai, bi) in &ab {
        adj_list[ai - 1].push(bi - 1);
        adj_list[bi - 1].push(ai - 1);
    }
    let degrees = adj_list.iter().map(|s| s.len()).collect::<Vec<_>>();
    let candidates = degrees.iter().enumerate().filter(|&(_, &d)| d >= 4).map(|(i, _)| i).collect::<Vec<_>>();

    
}
