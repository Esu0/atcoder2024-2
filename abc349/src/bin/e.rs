use proconio::input;

const IND: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn dfs(memo: &mut [[Option<bool>; 2]], state: usize, turn: usize, a: &[i64]) -> bool {
    if let Some(win) = memo[state][turn] {
        return win;
    }
    let mut end = true;

    for is in IND {
        let cols = is.map(|i| (state >> (i * 2)) & 3);
        if cols[0] == cols[1] && cols[1] == cols[2] {
            if cols[0] == turn {
                memo[state][turn] = Some(true);
                return true;
            } else if cols[0] == 1 - turn {
                memo[state][turn] = Some(false);
                return false;
            }
        }
    }

    for i in 0..9 {
        let col = (state >> (i * 2)) & 3;
        if col == 3 {
            end = false;
            if !dfs(memo, (state & !(3 << (i * 2))) | (turn << (2 * i)), 1 - turn, a) {
                memo[state][turn] = Some(true);
                return true;
            }
        }
    }

    if end {
        // 盤面が全て埋まっている
        let mut score = 0;
        let mut vs = 0;
        for (i, &ai) in a.iter().enumerate() {
            if (state >> (i * 2)) & 3 == turn {
                score += ai;
            } else {
                vs += ai;
            }
        }
        if score > vs {
            memo[state][turn] = Some(true);
            return true;
        } else {
            assert_ne!(score, vs);
            memo[state][turn] = Some(false);
            return false;
        }
    }

    memo[state][turn] = Some(false);
    false
}

fn main() {
    input! {
        a: [i64; 9],
    }

    let mut memo = vec![[None; 2]; 1 << 18];
    if dfs(&mut memo, (1 << 18) - 1, 0, &a) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
