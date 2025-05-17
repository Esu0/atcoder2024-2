use proconio::input;

fn solve(s: &[Vec<u8>], t: &[Vec<u8>]) -> usize {
    let mut cnt = 0;
    for (si, ti) in s.iter().zip(t) {
        for (&sij, &tij) in si.iter().zip(ti) {
            if sij != tij {
                cnt += 1;
            }
        }
    }
    cnt
}

fn rot(s: &mut [Vec<u8>]) {
    let n = s.len();
    let mut new = vec![vec![0; n]; n];
    for i in 0..s.len() {
        for j in 0..s.len() {
            new[j][n - i - 1] = s[i][j];
        }
    }
    s.clone_from_slice(&new);
}

fn main() {
    input! {
        n: usize,
        mut s: [proconio::marker::Bytes; n],
        t: [proconio::marker::Bytes; n],
    }

    let mut ans = solve(&s, &t);
    rot(&mut s);
    ans = ans.min(solve(&s, &t) + 1);
    rot(&mut s);
    ans = ans.min(solve(&s, &t) + 2);
    rot(&mut s);
    ans = ans.min(solve(&s, &t) + 3);
    println!("{ans}");
}
