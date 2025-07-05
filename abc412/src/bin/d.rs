use proconio::input;

fn dfs(mat: &mut [Vec<bool>], deg: &mut [usize], i: usize, j: usize, f: &mut impl FnMut(&[Vec<bool>])) {
    if i == mat.len() - 1 {
        if deg.iter().all(|&x| x == 2) {
            f(mat);
        }
        return;
    }

    let mut ni = i;
    let mut nj = j + 1;
    if nj == mat.len() {
        ni += 1;
        nj = ni + 1;
    }
    if deg[i] <= 1 && deg[j] <= 1 {
        deg[i] += 1;
        deg[j] += 1;
        mat[i][j] = true;
        dfs(mat, deg, ni, nj, f);
        deg[i] -= 1;
        deg[j] -= 1;
        mat[i][j] = false;
    }
    dfs(mat, deg, ni, nj, f);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut mat = vec![vec![false; n]; n];
    for &(ai, bi) in &ab {
        mat[ai - 1][bi - 1] = true;
        mat[bi - 1][ai - 1] = true;
    }
    let mut ans = usize::MAX;

    let mut m2 = vec![vec![false; n]; n];
    let mut deg = vec![0; n];
    dfs(&mut m2, &mut deg, 0, 1, &mut |m| {
        let mut count = 0;
        for (i, mi) in m.iter().enumerate() {
            for (j, &mij) in mi.iter().enumerate().skip(i + 1) {
                if mij != mat[i][j] {
                    count += 1;
                }
            }
        }
        ans = ans.min(count);
    });
    println!("{ans}");
}
